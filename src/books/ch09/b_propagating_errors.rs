use std::fs::File;
use std::{fs, io};
use std::io::Read;
/*
Result<T,E> {
    OK(T)
    Err(E)
}
*/

// '?' operator should return one of Result, Option, FromResidual
fn propagating_errors_v1() -> Result<String, io::Error>{
    let file_open = File::open("hello.txt");
    let mut file = match file_open {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn propagating_errors_v2() -> Result<String, io::Error>{
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;

    Ok(username)
}

fn propagating_errors_v3() -> Result<String, io::Error>{
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn propagating_errors_v4() -> Result<String, io::Error>{
    fs::read_to_string("hello.txt")
}

// Section : validate with struct

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        };
        Guess {value}
    }

    fn value(&self) -> i32 {
        self.value
    }
}