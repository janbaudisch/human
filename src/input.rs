use std::io;

pub fn read_line() -> String {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.pop();
        }
        Err(error) => panic!("{}", error)
    }

    input
}
