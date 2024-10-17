use std::time::Instant;

mod trie;
mod csv_reader;
use csv_reader::extract_from_csv;
use trie::{serialize_trie, deserialize_trie, Trie};

fn create_trie_from_csv_file(filename: String, column_name: String) -> Trie {
    println!("Extracting from {}...", filename);
    let mut trie = Trie::new();
    let mut contents = extract_from_csv(filename, column_name);

    for word in contents {
        trie.add_word(word);
    }
    
    println!("Contents extracted, trie generated");

    trie
}

fn save_trie(trie: Trie, filename: String) {
    println!("Saving trie to {}...", filename);
    let now = Instant::now();
    serialize_trie(trie, filename);
    println!("Trie saved in {:.2?}", now.elapsed());
}

fn load_trie(filename: String) -> Trie {
    println!("Loading trie from {}...", filename);
    let now = Instant::now();
    let trie = deserialize_trie(filename);
    println!("Trie loaded in {:.2?}", now.elapsed());
    trie
}

fn main() {
    let trie = create_trie_from_csv_file("./res/ngram_freq_dict.csv".to_string(), "word".to_string());
    
    // save_trie(trie, "./serialzed_files/ngram".to_string());
    // let trie = load_trie("./serialzed_files/ngram".to_string());
    
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
