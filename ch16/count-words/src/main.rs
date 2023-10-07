use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
fn main() {
    let filename = match env::args().nth(1) {
        Some(_filename) => _filename,
        None => {
            println!("Provide a valid filename:\n Example: count-word input_file.txt");
            return;
        }
    };

    let data = match File::open(filename) {
        Ok(_data) => _data,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };
    let mut dictionary: HashMap<String, i32> = HashMap::new();
    let mut top_occurences: HashMap<String, i32> = HashMap::new();
    let reader = io::read_to_string(data).unwrap();
    for _line in reader.lines() {
        for _word in _line.to_lowercase().split_whitespace() {
            let entry = dictionary.entry(_word.to_string()).or_insert(0);
            *entry += 1;
        }
    }
    let top_words_count = 5;
    let mut count = dictionary.len();
    if count > top_words_count {
        count = top_words_count;
    }
    let mut top_words: Vec<String> = Vec::new();
    let mut is_index_checked: bool = false;
    for i in 0..count {
        for (_key, _value) in dictionary.iter() {
            if top_words.len() == i && !is_index_checked {
                is_index_checked = true;
                top_words.push(_key.to_string());
                continue;
            }
            let check_val = dictionary.get(&top_words[i]).unwrap();
            if _value > check_val {
                top_words.pop();
                top_words.push(_key.to_string());
            }
        }
        is_index_checked = false;
        top_occurences.insert(
            top_words[i].to_string(),
            *dictionary.get(&top_words[i]).unwrap(),
        );
        dictionary.remove(&top_words[i]);
    }
    for word in top_words {
        println!("{} : {}", word, top_occurences.get(&word).unwrap());
    }
}
