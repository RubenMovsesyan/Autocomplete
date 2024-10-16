use std::fs::read_to_string;

pub fn extract_from_csv(filename: String, column: String) -> Vec<String> {
    let mut extracted: Vec<String> = Vec::new();

    let mut contents: Vec<String> = 
        read_to_string(filename)
            .unwrap()
            .lines()
            .map(String::from)
            .collect();

    let mut columns: Vec<String> = contents.first().unwrap().split(",").map(String::from).collect();
    contents.remove(0);
    let mut column_to_use = -1;


    for (index, category) in columns.iter().enumerate() {
        if &column == category {
            column_to_use = index as i32;
        }
    }

    for line in contents {
        columns = line.split(",").map(String::from).collect();
        extracted.push(columns.get(column_to_use as usize).unwrap().to_string());
    }

    extracted
}