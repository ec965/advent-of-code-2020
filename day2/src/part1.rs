// Enoch Chau
// Advent of code 2020 day 2 part 1
// find the valid password
// https://adventofcode.com/2020/day/2

use std::fs::File;
use std::io::{
    self,
    BufReader,
    prelude::*,
};

#[derive(Debug)]
struct Password {
    min: u16,
    max: u16,
    character: char,
    password: String,
}

impl Password {
    fn from(s:String) -> Password{
        let split: Vec<&str> = s.split(" ").collect();
        let mm = split[0];
        let mm: Vec<&str> = mm.split("-").collect();
        let min = &mm[0].parse::<u16>().unwrap();
        let max = &mm[1].parse::<u16>().unwrap();
        let character = split[1].chars().next().unwrap();
        let password = split[2].to_string();
        Password{
            min: *min,
            max: *max,
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
    for pw in passwords{
        let occurance = pw.password.matches(pw.character).count();
        if pw.min as usize <= occurance && occurance <= pw.max as usize { valid += 1 };
    }

    println!("valid passwords: {}", valid);
}