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
        let single = self.single.to_human();

        let ten = match self.ten {
            1 => "ten",
            2 => "twenty",
            3 => "thirty",
            4 => "forthy",
            5 => "fifty",
            6 => "sixty",
            7 => "seventy",
            8 => "eighty",
            9 => "ninety",
            _ => ""
        };

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

// make u8s human readable
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
