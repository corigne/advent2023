use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

pub fn TODO_rename(filepath: &str, debug: bool) {

    let file = match File::open(filepath) {
        Err(e) => return println!("IO error: {}", e),
        Ok(f) => f,
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {

        match line {
            Ok(l) => {
                if debug { dbg!(l); }
            },
            Err(e) => return println!("IO Error: {}", e),
        }
    };
}
