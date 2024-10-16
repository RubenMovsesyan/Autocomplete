use std::collections::{HashMap, LinkedList};
use std::fmt;

const INVALID: i32 = -1;
const END_WORD: i32 = -2;

enum TrieNode {
    Incomplete { node: Node },
    Complete { node: Node, word: String },
}

// Node implementation to work with rust hashmaps
struct Node {
    data: char,
    children: Vec<i32>,
    is_complete: bool,
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
            is_complete: false,
        }
    }

    fn add_child(&mut self, id: i32) {
        self.children.push(id);
    }

    fn get_children(&self) -> &Vec<i32> {
        &self.children
    }

    fn get_value(&self) -> char {
        self.data
    }

    fn set_complete(&mut self, value: bool) {
        self.is_complete = value;
    }

    fn is_complete(&self) -> bool {
        self.is_complete
    }
}

// Trie implementation that uses hashmaps to store node information instead of references
pub struct Trie {
    data: HashMap<i32, TrieNode>,
    current_size: i32,
}

impl Trie {
    pub fn new() -> Self {
        let mut hash_map: HashMap<i32, TrieNode> = HashMap::new();
        // let root = Node::new(' ');
        let root = TrieNode::Incomplete {
            node: Node::new(' ')
        };
        hash_map.insert(0, root);
        Self {
            data: hash_map,
            current_size: 0,
        }
    }

    // fn check_for_character(&self, current_node: i32, character: char) -> i32 {
    //     let parent_node = self.data.get(&current_node).unwrap();

    //     // Check each child for the character defined
    //     for node_id in parent_node.children.iter() {
    //         let check_node = self.data.get(node_id).unwrap();
    //         if check_node.data == character {
    //             return node_id.clone();
    //         }
    //     }

    //     // If the character is not in the children nodes return INVALID
    //     INVALID
    // }

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

            if node_with_character == -1 {
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
        }
    }

    // pub fn add_word(&mut self, word: String) {
    //     let mut current_node = 0; // This is the root node id always

    //     for (index, character) in word.char_indices() {
    //         // If the character is not a child of the current node then add it, otherwise move on to
    //         // the child and get the next character
    //         let node_with_character = self.check_for_character(current_node, character);

    //         if node_with_character == -1 {
    //             let new_node = Node::new(character);
    //             self.current_size += 1;
    //             self.data
    //                 .get_mut(&current_node)
    //                 .unwrap()
    //                 .add_child(self.current_size);
    //             current_node = self.current_size;
    //             self.data.insert(self.current_size, new_node);
    //         } else {
    //             current_node = node_with_character;
    //         }

    //         if index == word.len() - 1 {
    //             self.data.get_mut(&current_node).unwrap().set_complete(true);
    //         }
    //     }
    // }

    pub fn get_size(&self) -> i32 {
        self.current_size
    }

    fn get_node(&self, node_id: i32) -> &Node {
        self.data.get(&node_id).unwrap().get_node()
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

            for child in children {
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
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("   ID   | value |          word          | Children");
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
            print!(
                "{:>8}|{:>5}  |{:>24}|",
                id,
                val,
                word_to_print
            );


            for child in value.get_node().children.iter() {
                print!("{}, ", child);
            }
            println!();
        })
    }
}
