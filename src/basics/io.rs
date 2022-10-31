use std::{fs::File, fs::read_to_string, fs::read, io::{BufReader, BufRead}};


// https://kerkour.com/rust-read-file

pub(super) fn read_file(file_name: &str){
    let res = File::open(file_name);
    // let f = res.unwrap(); // crashed program if file not found
    // let f = res.expect("error"); // crash with custom error
    /*
    if res.is_ok() {
        let f = res.unwrap();  // will never crash
    }
    */

    match res {
        Ok(f) => {
            let reader = BufReader::new(f);

            for (index, line) in reader.lines().enumerate() {
                let line = line.unwrap(); // Ignore errors.
                // Show the line and its number.
                println!("{}. {}", index + 1, line);            }

        },
        Err(e) => {println!("{} - {}", e, file_name)},
    }

}

// Read file as a vector
pub(super) fn read_file_vec(filepath: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

    let contents = read_to_string(filepath)
    .expect("Something went wrong reading the file");

    println!("\n\nReading Poem as a string:\n\n{}", contents);

    // Unicode vector
    let data = read(filepath)?;
    Ok(data)
}

// Annonymous function called closure
