use std::collections::HashMap;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::fmt;


// Database support ------------------------------------------------------
// use sqlite::Error as sqErr;

// #[derive(Debug)]
// pub enum TrieErr {
//     DbErr(sqErr),
// }

// impl From<sqErr> for TrieErr {
//     fn from(value: sqErr) -> Self {
//         TrieErr::DbErr(s)
//     }
// }

// -----------------------------------------------------------------------



// Database support ------------------------------------------------------
extern crate rusqlite;

use rusqlite::{Connection, Result};
// -----------------------------------------------------------------------

const INVALID: i32 = -1;

#[derive(Serialize, Deserialize)]
enum TrieNode {
    Incomplete { node: Node },
    Complete { node: Node, word: String },
}

// Node implementation to work with rust hashmaps
#[derive(Serialize, Deserialize)]
struct Node {
    data: char,
    children: Vec<i32>,
}

impl TrieNode {
    fn add_child(&mut self, id: i32) {
        match self {
            TrieNode::Incomplete {node} => {
                node.children.push(id);
            }
            TrieNode::Complete {node, word: _} => {
                node.children.push(id);
            },
        }
    }

    fn get_node(&self) -> &Node {
        match self {
            TrieNode::Incomplete { node } => return node,
            TrieNode::Complete { node, word: _ } => return node,
        }
    }
}

impl Node {
    fn new(value: char) -> Self {
        Self {
            data: value,
            children: Vec::new(),
        }
    }

    fn get_children(&self) -> &Vec<i32> {
        &self.children
    }
}

// Trie implementation that uses hashmaps to store node information instead of references
#[derive(Serialize, Deserialize)]
pub struct Trie {
    data: HashMap<i32, TrieNode>,
    current_size: i32,
}

impl Trie {
    pub fn new() -> Self {
        let mut hash_map: HashMap<i32, TrieNode> = HashMap::new();
        let root = TrieNode::Incomplete {
            node: Node::new(' ')
        };
        hash_map.insert(0, root);
        Self {
            data: hash_map,
            current_size: 0,
        }
    }

    fn check_for_character(&self, current_node: i32, character: char) -> i32 {
        let parent_node = self.data.get(&current_node).unwrap();

        for node_id in parent_node.get_node().children.iter() {
            let check_node = self.data.get(node_id).unwrap();
            if check_node.get_node().data == character {
                return node_id.clone();
            }
        }

        // If the character is not in the children nodes return INVALID
        INVALID
    }

    pub fn add_word(&mut self, word: String) {
        let mut current_node = 0; // This is the root node id always

        for (index, character) in word.char_indices() {
            // If the character is not a child of the current node then add it, otherwise move on to
            // the child and get the next character
            let node_with_character = self.check_for_character(current_node, character);
            
            if node_with_character == INVALID {
                let new_node = Node::new(character);
                let node_to_add: TrieNode;
                // Add the proper node type based on if the word is complete or not
                if index == word.len() - 1 {
                    node_to_add = TrieNode::Complete { node: new_node, word: word.clone() };
                } else {
                    node_to_add = TrieNode::Incomplete { node: new_node };
                }

                self.current_size += 1;
                self.data
                    .get_mut(&current_node)
                    .unwrap()
                    .add_child(self.current_size);
                current_node = self.current_size;
                self.data.insert(self.current_size, node_to_add);
            } else {
                current_node = node_with_character;
            }

            // println!("{}", self);
        }
    }

    pub fn get_size(&self) -> i32 {
        self.current_size
    }

    fn get_trie_node(&self, node_id: i32) -> &TrieNode {
        self.data.get(&node_id).unwrap()
    }

    pub fn get_suggested_words(&self, current_word: String, amount: i32) -> Vec<String> {
        let mut suggested_words: Vec<String> = Vec::new();

        let mut current_node = 0;

        for letter in current_word.chars() {
            let node_with_character = self.check_for_character(current_node, letter);

            if node_with_character == -1 {
                break;
            }

            current_node = node_with_character;
        }

        // Depth first search
        let mut visited: Vec<i32> = Vec::new();
        let mut stack: Vec<i32> = Vec::new();

        while suggested_words.len() < amount as usize {
            visited.push(current_node);

            let ref_node = self.get_trie_node(current_node);

            match ref_node {
                TrieNode::Complete { node: _, word } => {
                    suggested_words.push(word.clone());
                },
                _ => {},
            }
            let children = ref_node
                .get_node()
                .get_children();

            for child in children
                .iter()
                .rev() { // Get rid of this line to make it work based on the least popular words
                if !visited.contains(child) {
                    stack.push(*child);
                }
            }

            current_node = stack.pop().unwrap_or(-1);

            if current_node == -1 {
                break;
            }
        }

        suggested_words
    }
}

// This is to make debugging easier
impl fmt::Display for Trie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "   ID   | value |          word          | Children")?;
        writeln!(f, "________|_______|________________________|_________")?;
        Ok(for (id, value) in &self.data {
            let val: String;
            let mut word_to_print = String::from("");

            match value {
                TrieNode::Incomplete { node } => {
                    val = format!(" {} ", node.data);
                },
                TrieNode::Complete { node, word } => {
                    val = format!("[{}]", node.data);
                    word_to_print = word.clone();
                }, 
            }
            write!(
                f,
                "{:>8}|{:>5}  |{:^24}|",
                id,
                val,
                word_to_print
            )?;


            for child in value.get_node().children.iter() {
                write!(f, "{}, ", child)?;
            }
            writeln!(f)?;
        })
    }
}


pub fn serialize_trie(trie: Trie, filename: String) -> Result<()> {
    // let file = File::create(filename).unwrap();
    // let _ = serde_bare::to_writer(&file, &trie);

    // let connection = sqlite::open("./common_words.db");
    let connection = Connection::open("./common_words.db")?;

    connection.execute(
        "drop table if exists common_words",
        (),
    )?;

    connection.execute(
        "create table if not exists common_words (
            id integer primary key,
            node_data text not null
        )",
        ()
    )?;

    // This insersts the hash table into the database
    for (id, node) in trie.data {
        let serialized_node = serde_json::to_string(&node).unwrap();

        connection.execute(
            "insert into common_words (id, node_data)
            values (?1, ?2)",
            &[&id.to_string(), &serialized_node.to_string()]
        )?;
    }

    Ok(())
}

pub fn deserialize_trie(filename: String) -> Trie {
    // TODO implement error handling
    // let mut contents: String = Default::default();
    let file = File::open(filename).unwrap();

    // serde_json::from_str(&contents).unwrap()
    serde_bare::from_reader(&file).unwrap()
}