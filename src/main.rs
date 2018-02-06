// TODO: disallow leading zero
// TODO: trim whitespace after hundred without ten
// TODO: make 11, 12, 13, ... work properly
// TODO: add tests

#[macro_use] extern crate text_io;

mod block;

use block::Block;
use std::process;

fn main() {
    // get user input
    let input: String = read!("{}\n");

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

    // make blocks of three digits
    let mut i = digits.len();
    let mut level = 0;
    let mut blocks: Vec<Block> = Vec::new();

    while i > 0 {
        let single = digits[i - 1];

        let ten;
        if i >= 2 {
            ten = digits[i - 2];
        } else {
            ten = 0;
        }

        let hundred;
        if i >= 3 {
            hundred = digits[i - 3];
        } else {
            hundred = 0;
        }

        blocks.push(Block::new(single, ten, hundred, level));

        if i < 3 {
            break;
        }

        level = level + 1;
        i = i - 3;
    }

    // chain all block together in a single string
    // start with last block specifically to prevent leading comma
    let mut i = blocks.len();
    let mut output: String = format!("{}", blocks[i - 1]);

    if i > 0 {
        i = i - 1;
    }

    while i > 0 {
        let new = format!("{}", blocks[i - 1]);
        output = format!("{}, {}", output, new.trim_left());
        i = i - 1;
    }

    // done
    println!("{}", output.trim_left());
}
