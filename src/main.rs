// use std::collections::HashMap;
use std::time::Instant;

mod trie;
mod csv_reader;
use csv_reader::extract_from_csv;
use trie::{serialize_trie, deserialize_trie, Trie};

fn main() {
    let mut trie = Trie::new();
    // trie.add_word(String::from("car"));
    // trie.add_word(String::from("card"));
    // trie.add_word(String::from("cards"));
    // trie.add_word(String::from("cat"));
    // trie.add_word(String::from("trie"));
    // trie.add_word(String::from("try"));
    // trie.add_word(String::from("trying"));
    // trie.add_word(String::from("top"));

    // println!("{}", trie);
    let filename = String::from("./serialized_files/n_gram");

    println!("Extracting contents...");
    let mut contents = extract_from_csv("./res/ngram_freq_dict.csv".to_string(), "word".to_string());
    // contents.reverse();
    println!("Contents Extracted, Adding to Trie...");
    for word in contents {
        // println!("{}", word);
        trie.add_word(word);
    }


    // println!("Trie generation complete, Serializing and saving to {}", filename);

    // serialize_trie(trie, filename);
    // println!("Serialization complete");


    // let current_word = String::from("menag");

    // println!("Trie generated with {} nodes, Running autocomplete on: {}", trie.get_size(), current_word);


    // let now = Instant::now();
    // println!("Extracting from serialized tree...");
    // let now = Instant::now();
    // let trie = deserialize_trie(filename);
    // println!("Extraction complete, took {:.2?}", now.elapsed());
    
    
    for word in trie.get_suggested_words("t".to_string(), 5) {
        println!("{}", word);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_input() {
        let mut trie = Trie::new();
        trie.add_word(String::from("car"));
        assert_eq!(trie.get_size(), 3);
        trie.add_word(String::from("card"));
        assert_eq!(trie.get_size(), 4);
        trie.add_word(String::from("cards"));
        assert_eq!(trie.get_size(), 5);
        trie.add_word(String::from("cat"));
        assert_eq!(trie.get_size(), 6);
        trie.add_word(String::from("trie"));
        assert_eq!(trie.get_size(), 10);
        trie.add_word(String::from("try"));
        assert_eq!(trie.get_size(), 11);
        trie.add_word(String::from("trying"));
        assert_eq!(trie.get_size(), 14);
    }
}
