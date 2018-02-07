use std::{u8, fmt};

// a block describing three digits
pub struct Block {
    single: u8,
    ten: u8,
    hundred: u8,
    // indicates occurence of block (1st, 2nd, 3rd, ...)
    level: u8
}

impl Block {
    // creates a new block
    pub fn new(single: u8, ten: u8, hundred: u8, level: u8) -> Block {
        Block {
            single: single,
            ten: ten,
            hundred: hundred,
            level: level
        }
    }
}

// implement formatting of a block as a way of serializing it
impl fmt::Display for Block {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // let mut single = self.single.to_human();
        let single;

        let ten = match self.ten {
            1 => {
                single = match self.single {
                    0 => "ten",
                    1 => "eleven",
                    2 => "twelve",
                    3 => "thirteen",
                    4 => "fourteen",
                    5 => "fifteen",
                    6 => "sixteen",
                    7 => "seventeen",
                    8 => "eighteen",
                    9 => "nineteen",
                    _ => ""
                };

                ""
            },
            _ => {
                single = self.single.to_human();

                match self.ten {
                    2 => "twenty",
                    3 => "thirty",
                    4 => "forty",
                    5 => "fifty",
                    6 => "sixty",
                    7 => "seventy",
                    8 => "eighty",
                    9 => "ninety",
                    _ => ""
                }
            }
        }.trim_right();

        let hundred = match self.hundred {
            1...9 => format!("{} hundred", self.hundred.to_human()),
            _ => "".to_owned()
        };

        match self.level {
            // 'base' block doesn't have an identifier
            0 => {
                write!(formatter, "{} {} {}", hundred, ten, single)
            },
            _ => {
                if single == "" && ten == "" && hundred == "" {
                    write!(formatter, "")
                } else {
                    let identifier = match self.level {
                        1 => "thousand",
                        2 => "million",
                        3 => "billion",
                        4 => "trillion",
                        5 => "quadrillion",
                        6 => "quintillion",
                        7 => "sextillion",
                        8 => "septillion",
                        _ => ""
                    };

                    let block = format!("{} {} {}", hundred, ten, single);
                    write!(formatter, "{} {}", block.trim_right(), identifier)
                }
            }
        }
    }
}

// make u8 human readable
trait ToHuman {
    fn to_human(&self) -> &str;
}

impl ToHuman for u8 {
    fn to_human(&self) -> &str {
        match self {
            &1 => "one",
            &2 => "two",
            &3 => "three",
            &4 => "four",
            &5 => "five",
            &6 => "six",
            &7 => "seven",
            &8 => "eight",
            &9 => "nine",
            _ => ""
        }
    }
}
