mod block;
mod input;

use block::Block;
use std::process;

fn main() {
    // get user input
    let input: String = input::read_line();

    // convert to vec of digits
    let mut digits: Vec<u8> = Vec::new();
    for digit in input.chars() {
        let digit = digit.to_digit(10);

        if digit.is_none() {
            // exit when one character is not a digit
            println!("Error deserializing input!");
            process::exit(1);
        }

        // we don't need u32s
        let digit = digit.unwrap() as u8;

        digits.push(digit);
    }

    // don't proceed without digits
    let mut i = digits.len();
    if i == 0 {
        println!("No numeric litaral given!");
        process::exit(1);
    }

    // don't proceed with leading zero
    if digits[0] == 0 && i > 1 {
        println!("Why would you start with 0?");
        process::exit(1);
    }

    // with one zero, just print it, no need to add logic for that
    if digits[0] == 0 {
        println!("zero");
        process::exit(0);
    }

    // everything should be fine
    // make blocks of three digits
    let mut level = 0;
    let mut blocks: Vec<Block> = Vec::new();

    while i > 0 {
        let single = digits[i - 1];

        let ten = if i >= 2 { digits[i - 2] } else { 0 };

        let hundred = if i >= 3 { digits[i - 3] } else { 0 };

        blocks.push(Block::new(single, ten, hundred, level));

        if i < 3 {
            break;
        }

        level += 1;
        i -= 3;
    }

    // chain all block together in a single string
    // start with last block specifically to prevent leading comma
    let mut i = blocks.len();
    let mut output: String = format!("{}", blocks[i - 1]);

    while i > 1 {
        i -= 1;

        let new = format!("{}", blocks[i - 1]);

        if new != "" {
            output = format!("{}, {}", output, new.trim_left());
        }
    }

    // done
    println!("{}", output.trim_left());
}
