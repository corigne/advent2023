use std::i32;
use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::ops::Index;
use std::task::Wake;

use rand::distributions::DistString;

pub fn missing_part(filepath: &str, debug: bool) {

    let file = match File::open(filepath) {
        Err(e) => return println!("IO error: {}", e),
        Ok(f) => f,
    };
    let reader = BufReader::new(file);
    let mut schem: Vec<String> = Vec::new();
    let mut matches: Vec<String> = vec![];

    for line in reader.lines() {

        match line {
            Ok(l) => {
                if debug { dbg!(&l); }
                let mut l = l.chars().collect::<String>();
                l.insert(0, '.');
                l.push('.');
                schem.push(l);
            },
            Err(e) => return println!("IO Error: {}", e),
        }
    };

    let pad: String = vec!['.'; schem[0].len()].iter().collect();

    schem.insert(0, pad.clone());
    schem.push(pad);

    for line in 1..schem.len()-1 {
        let ln = &schem[line];
        let above = &schem[line-1];
        let below = &schem[line+1];

        let mut valid: bool = false;
        let mut buf: String = String::new();

        for c in 1..ln.len()-1 {
            let up_sym = &above[c-1..=c+1];
            let dwn_sym = &below[c-1..=c+1];
            let l = &ln.chars().collect::<Vec<_>>()[c-1].to_string();
            let r = &ln.chars().collect::<Vec<_>>()[c+1].to_string();
            let neighbors = [up_sym, dwn_sym, l, r];

            match ln.chars().collect::<Vec<_>>()[c] {

                tok if tok.is_ascii_digit() => {
                    buf.push(tok);
                    for neighbor in neighbors {
                        valid = valid | neighbor.chars()
                            .any(|c| c.is_ascii_punctuation() && c != '.')
                    }
                    if r.parse::<i32>().is_err() && valid && !buf.is_empty() {
                        matches.push(buf.clone());
                        buf.clear()
                    }

                },

                '.' => {

                    buf.clear();

                    for neighbor in neighbors {
                        valid = valid && neighbor.chars().all(|c| c == '.')
                    }

                }
                _ => ()
            };
        }

    }
    if debug { dbg!(&matches); }

    let parts = matches.iter().fold(0 as u32, |acc, num| acc + num.parse().unwrap_or(0));
    println!("Parts sum: {}", parts);
}
