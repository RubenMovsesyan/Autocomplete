use std::collections::HashMap;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::fmt;

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


pub struct AutoCompleteMemory {
    word: String,
    node_ids: Vec<i32>,
}


impl AutoCompleteMemory {
    pub fn new() -> Self {
        Self {
            word: String::new(),
            node_ids: Vec::new(),
        }
    }

    pub fn from_string(string: String) -> Self {
        Self {
            word: string,
            node_ids: Vec::new(),
        }
    }

    // Updates the AutoCompleteMemory
    // If the new string is an extension of the word it keeps the memory
    // If the new string is a different word it gets rid of the memory
    // This is not that big a deal because in my testing it really only
    // makes a difference of 2ns (which is negligible in this application)
    pub fn update(&mut self, updated_string: String) {
        if !updated_string.contains(&self.word) {
            self.node_ids.clear();
        }
        self.word = updated_string;
    }

    // Returns copy of the word
    pub fn get_word(&self) -> String {
        self.word.clone()
    }

    // Returns a reference to the node ids
    pub fn get_node_ids(&self) -> &Vec<i32> {
        &self.node_ids
    }

    pub fn push_node_id(&mut self, node_id: i32) {
        self.node_ids.push(node_id);
    }

    // This is mostly just used for testing
    pub fn reset_node_ids(&mut self) {
        self.node_ids.clear();
    }

    pub fn update_and_reset(&mut self, updated_string: String) {
        self.word = updated_string;
        self.node_ids.clear();
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

    pub fn get_suggested_words(&self, current_word: &mut AutoCompleteMemory, amount: i32) -> Vec<String> {
        let mut suggested_words: Vec<String> = Vec::new();

        let mut current_node = 0;

        // If the trie traversal has already been calculated then go to that node
        if current_word.get_node_ids().len() > 0 {
            current_node = *current_word.get_node_ids().last().unwrap();
        }

        for letter in current_word.get_word()[current_word.get_node_ids().len()..].chars() {
            let node_with_character = self.check_for_character(current_node, letter);

            if node_with_character == -1 {
                break;
            }

            current_node = node_with_character;
            current_word.push_node_id(current_node);
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


pub fn serialize_trie(trie: Trie, filename: String) {
    let file = File::create(filename).unwrap();
    let _ = serde_bare::to_writer(&file, &trie);
    // file.write(serialized.as_bytes());
}

pub fn deserialize_trie(filename: String) -> Trie {
    // TODO implement error handling
    // let mut contents: String = Default::default();
    let file = File::open(filename).unwrap();

    // serde_json::from_str(&contents).unwrap()
    serde_bare::from_reader(&file).unwrap()
}