use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

fn is_legit(red: u32, green: u32, blue: u32) -> bool {
    red <= 12 && green <= 13 && blue <= 14
}

pub fn cube_game(filepath: &str, debug: bool) {
    let file = match File::open(filepath) {
        Err(e) => return println!("IO error: {}", e),
        Ok(f) => f,
    };
    let reader = BufReader::new(file);

    let mut game_sum: u32 = 0;
    let mut viable: Vec<u32> = vec![];

    for line in reader.lines() {

        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        match line {
            Ok(l) => {
                if debug { dbg!(&l); }
                let (game_number, game_info) = l.split_once(':').unwrap();
                let game_number = game_number.split_whitespace().find_map(|n| n.parse::<u32>().ok()).unwrap();
                let game_sets: Vec<&str> = game_info.split(';').collect();
                for set in game_sets {
                    red = match set.split(',').filter(|s| s.contains("red")).next() {
                        Some(s) => {
                            red.max(s.split_whitespace().find_map(|n| n.parse().ok()).unwrap())
                        }
                        None => red,
                    };
                    green = match set.split(',').filter(|s| s.contains("green")).next() {
                        Some(s) => {
                            green.max(s.split_whitespace().find_map(|n| n.parse().ok()).unwrap())
                        }
                        None => green,
                    };
                    blue = match set.split(',').filter(|s| s.contains("blue")).next() {
                        Some(s) => {
                            blue.max(s.split_whitespace().find_map(|n| n.parse().ok()).unwrap())
                        }
                        None => blue,
                    };
                }
                if is_legit(red, green, blue) { viable.push(game_number); game_sum += game_number }
            },
            Err(e) => return println!("IO Error: {}", e),
        }
    };

    if debug { dbg!(&viable); }
    println!("Sum of viable game numbers: {game_sum}");
}

pub fn cube_power(filepath: &str, debug: bool) {
    let file = match File::open(filepath) {
        Err(e) => return println!("IO error: {}", e),
        Ok(f) => f,
    };
    let reader = BufReader::new(file);

    let mut game_sum: u32 = 0;
    let viable: Vec<u32> = vec![];

    for line in reader.lines() {

        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        match line {
            Ok(l) => {
                if debug { dbg!(&l); }
                let (_game_number, game_info) = l.split_once(':').unwrap();
                let game_sets: Vec<&str> = game_info.split(';').collect();
                for set in game_sets {
                    red = match set.split(',').filter(|s| s.contains("red")).next() {
                        Some(s) => {
                            red.max(s.split_whitespace().find_map(|n| n.parse().ok()).unwrap())
                        }
                        None => red,
                    };
                    green = match set.split(',').filter(|s| s.contains("green")).next() {
                        Some(s) => {
                            green.max(s.split_whitespace().find_map(|n| n.parse().ok()).unwrap())
                        }
                        None => green,
                    };
                    blue = match set.split(',').filter(|s| s.contains("blue")).next() {
                        Some(s) => {
                            blue.max(s.split_whitespace().find_map(|n| n.parse().ok()).unwrap())
                        }
                        None => blue,
                    };
                }
                game_sum += red * blue * green;
            },
            Err(e) => return println!("IO Error: {}", e),
        }
    };

    if debug { dbg!(&viable); }

    println!("Sum of viable game numbers: {game_sum}");
}
