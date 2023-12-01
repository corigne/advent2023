use std::io::{self};

fn main() {
    println!("Hello, world!\nPlease enter a string: ");
    let mut instr = String::new();

    io::stdin().read_line(&mut instr).expect("Error occurred on input.");

    println!("You entered: {instr}");
}
