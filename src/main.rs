use std::time::Instant;

mod csv_reader;
mod trie;
use csv_reader::extract_from_csv;
use trie::{deserialize_trie, serialize_trie, AutoCompleteMemory, Trie};

const NUM_BENCHMARKS: i32 = 500;

fn create_trie_from_csv_file(filename: String, column_name: String) -> Trie {
    println!("Extracting from {}...", filename);
    let mut trie = Trie::new();
    let now = Instant::now();
    let contents = extract_from_csv(filename, column_name);

    for word in contents {
        trie.add_word(word);
    }

    println!(
        "Contents extracted, trie generated in {:.2?}",
        now.elapsed()
    );

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

fn benchmark_speed_with_memory(trie: &Trie, benchmark_times: i32) {
    let mut iteration_1_avg = 0;
    let mut iteration_2_avg = 0;
    let mut iteration_3_avg = 0;

    println!("Running {} tests with memory...", benchmark_times);
    let test_now = Instant::now();
    for i in 1..=benchmark_times {
        let mut current_word = AutoCompleteMemory::from_string(String::from("unbend"));

        let now = Instant::now();
        let _ = trie.get_suggested_words(&mut current_word, 5);
        iteration_1_avg += now.elapsed().as_nanos() as i64;
        iteration_1_avg /= i as i64;

        current_word.update(String::from("unbendi"));

        let now = Instant::now();
        let _ = trie.get_suggested_words(&mut current_word, 5);
        iteration_2_avg += now.elapsed().as_nanos() as i64;
        iteration_2_avg /= i as i64;

        current_word.update(String::from("unbendin"));

        let now = Instant::now();
        let _ = trie.get_suggested_words(&mut current_word, 5);
        iteration_3_avg += now.elapsed().as_nanos() as i64;
        iteration_3_avg /= i as i64;
    }

    println!(
        "Ran {} tests in {:.2?}",
        benchmark_times,
        test_now.elapsed()
    );
    println!("    Iteration 1 avg: {}ns", iteration_1_avg);
    println!("    Iteration 2 avg: {}ns", iteration_2_avg);
    println!("    Iteration 3 avg: {}ns", iteration_3_avg);
}

fn benchmark_speed_without_memory(trie: &Trie, benchmark_times: i32) {
    let mut iteration_1_avg = 0;
    let mut iteration_2_avg = 0;
    let mut iteration_3_avg = 0;

    println!("Running {} tests without memory...", benchmark_times);
    let test_now = Instant::now();
    for i in 1..=benchmark_times {
        let mut current_word = AutoCompleteMemory::from_string(String::from("unbend"));

        let now = Instant::now();
        let _ = trie.get_suggested_words(&mut current_word, 5);
        iteration_1_avg += now.elapsed().as_nanos() as i64;
        iteration_1_avg /= i as i64;

        current_word.update_and_reset(String::from("unbendi"));

        let now = Instant::now();
        let _ = trie.get_suggested_words(&mut current_word, 5);
        iteration_2_avg += now.elapsed().as_nanos() as i64;
        iteration_2_avg /= i as i64;

        current_word.update_and_reset(String::from("unbendin"));

        let now = Instant::now();
        let _ = trie.get_suggested_words(&mut current_word, 5);
        iteration_3_avg += now.elapsed().as_nanos() as i64;
        iteration_3_avg /= i as i64;
    }

    println!(
        "Ran {} tests in {:.2?}",
        benchmark_times,
        test_now.elapsed()
    );
    println!("    Iteration 1 avg: {}ns", iteration_1_avg);
    println!("    Iteration 2 avg: {}ns", iteration_2_avg);
    println!("    Iteration 3 avg: {}ns", iteration_3_avg);
}

fn main() {
    let trie =
        create_trie_from_csv_file("./res/ngram_freq_dict.csv".to_string(), "word".to_string());

    // save_trie(trie, "./serialized_files/ngram".to_string());

    // let trie = load_trie("./serialized_files/ngram".to_string());

    benchmark_speed_with_memory(&trie, NUM_BENCHMARKS);
    benchmark_speed_without_memory(&trie, NUM_BENCHMARKS);
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
