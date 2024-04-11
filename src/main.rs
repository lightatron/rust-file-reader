use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

struct Person {
    name: String,
    age: u8,
    gender: char,
}

impl Person {
    fn new(name: String, age: u8, gender: char) -> Person {
        Person {
            name,
            age,
            gender
        }
    }
}

fn main() {
    // change file to be a user defined path in std input - to be typed in the terminal
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]);
    
    let mut errfile = OpenOptions::new().create(true).append(true).open("error.log").unwrap();
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                writeln!(errfile, "File not found: {}\n", error).unwrap();
                panic!("File not found: {}", error)
            },
            std::io::ErrorKind::PermissionDenied => {
                writeln!(errfile, "Permission denied: {}\n", error).unwrap();
                panic!("Permission denied: {}", error)
            },
            _ => {
                writeln!(errfile, "Error opening file: {}\n", error).unwrap();
                panic!("Error opening file: {}", error)
            },
        }
    };


    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}