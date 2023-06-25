use std::{env, fs::File, io::{BufReader, BufRead}, collections::HashMap, error::Error};


pub fn read_file(file_name: &str) -> Result<(), Box<dyn Error>> {

    // let args: Vec<String> = env::args().collect();
    // let file_name = &args[1];

    let file = File::open(file_name)
    .unwrap_or_else(|_| panic!("file not found: {}", &file_name));

    let reader = BufReader::new(file);
    let mut word_count: HashMap<String, u64> = HashMap::new();
    
    for result in reader.lines() {
        let line = result?;
        for word in line.split_whitespace() {
            *word_count.entry(word.to_string()).or_insert(0) += 1;
        }
    }

    let mut word_count_ordered: Vec<(String, u64)> = word_count.into_iter().collect();
    word_count_ordered.sort_by(|&(_, cnt1), &(_, cnt2)| cnt1.cmp(&cnt2));
    for (word, count) in word_count_ordered {
        println!("{} {}", word, count);
    }
    Ok(())


}