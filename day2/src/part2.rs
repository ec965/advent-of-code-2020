// Enoch Chau
// Advent of code 2020 day 2 part 2

use std::fs::File;
use std::io::{
    self,
    BufReader,
    prelude::*,
};

#[derive(Debug)]
struct Password {
    index1: usize,
    index2: usize,
    character: char,
    password: String,
}

impl Password {
    fn from(s:String) -> Password{
        let split: Vec<&str> = s.split(" ").collect();
        let mm = split[0];
        let mm: Vec<&str> = mm.split("-").collect();
        let min = &mm[0].parse::<usize>().unwrap();
        let max = &mm[1].parse::<usize>().unwrap();
        let character = split[1].chars().next().unwrap();
        let password = split[2].to_string();
        Password{
            index1: *min,
            index2: *max,
            character,
            password
        }
    }
}

fn main(){
    let file = File::open("inputs.txt").unwrap();
    let reader = BufReader::new(file);
    let mut passwords = Vec::new();

    for line in reader.lines(){
        let parse = line
            .unwrap()
            .to_string();

        passwords.push(Password::from(parse));
    }

    let mut valid = 0;
    for p in passwords{
        // println!("{:#?}", p);
        let ch1 = p.password.chars().nth(p.index1-1).unwrap();
        let ch2 = p.password.chars().nth(p.index2-1).unwrap();

        if ch1 == p.character {
            if ch2 != p.character{
                valid += 1;
            }
        } else if ch2 == p.character {
            valid += 1;
        }
    }

    println!("valid passwords: {}", valid);
}