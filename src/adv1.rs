use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::collections::VecDeque;
use std::char;
use aho_corasick::AhoCorasick;

pub fn calibration_parse(filepath: & str) {
    println!("Advent Calendar 1 - 1\nInput filepath: {filepath}");

    let file = match File::open(filepath) {
        Err(e) => return println!("IO error: {}", e),
        Ok(f) => f,
    };
    let reader = BufReader::new(file);

    let mut cal_sum: u32 = 0;

    for line in reader.lines() {
        match line {
            Ok(l) => {
                let mut chars: VecDeque<char> = l.chars().filter(|c| c.is_digit(10)).collect();

                let first: char = match chars.pop_front() {
                    Some(first_digit) => first_digit,
                    None => {
                        continue;
                    }
                };

                let last = match chars.pop_back() {
                    Some(first_digit) => first_digit,
                    None => {
                        first
                    },
                };

                let mut cal_val: String = String::from(first);
                cal_val.push(last);

                let cal_val = match cal_val.parse::<u32>() {
                    Ok(value) => value,
                    Err(_) => continue,
                };
                cal_sum += cal_val;

            },
            Err(e) => return println!("IO Error: {}", e),
        }
    };

    println!("Calibration value: {cal_sum}");
}

fn str_to_num(input: &str) -> char {
    match input {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => ' '
    }
}

pub fn calibration_parse_2(filepath: & str, debug: bool) {

    println!("Advent Calendar 1 - 2\nInput filepath: {filepath}");

    let file = match File::open(filepath) {
        Err(e) => return println!("IO error: {}", e),
        Ok(f) => f,
    };
    let reader = BufReader::new(file);

    let mut cal_sum: u32 = 0;

    let ac_pat = ["one","two","three","four","five","six","seven","eight",
        "nine","1","2","3","4","5","6","7","8","9"];
    let ac = AhoCorasick::new(ac_pat).unwrap();

    for line in reader.lines() {
        match line {
            Ok(l) => {
                let mut nums: VecDeque<_> = ac.find_overlapping_iter(&l).collect();

                let first = nums.pop_front().unwrap();
                let first = &l[first.start()..first.end()];
                let first: char = match first.parse() {
                    Ok(c) => c,
                    Err(_) => str_to_num(&first),
                };
                let last: char = match nums.pop_back() {
                    Some(num) => match &l[num.start()..num.end()].parse() {
                        Ok(c) => *c,
                        Err(_) => str_to_num(&l[num.start()..num.end()]),
                    },
                    None => first,
                };

                let cal_val = String::from(first) + &last.to_string();
                let cal_val = match cal_val.parse::<u32>() {
                    Ok(value) => value,
                    Err(_) => {
                        dbg!("PARSE ERROR");
                        return
                    },
                };

                if debug {
                    dbg!(&l);
                    dbg!(nums);
                    dbg!(first, last);
                    dbg!(cal_val);
                }

                cal_sum += cal_val;
            },
            Err(e) => return println!("IO Error: {}", e),
        }
    };

    println!("Calibration value: {}", cal_sum);
}
