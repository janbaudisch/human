use std::{u8, fmt};

pub struct Block {
    single: u8,
    ten: u8,
    hundred: u8,
    level: u8
}

impl Block {
    pub fn new(single: u8, ten: u8, hundred: u8, level: u8) -> Block {
        Block {
            single: single,
            ten: ten,
            hundred: hundred,
            level: level
        }
    }
}

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
            0 => "".to_owned(),
            _ => format!("{} hundred", self.hundred.to_human())
        };

        match self.level {
            0 => {
                write!(formatter, "{} {} {} {}", hundred, ten, "and", single)
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

                write!(formatter, "{} {} {} {}", hundred, ten, single, identifier)
            }
        }
    }
}

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
