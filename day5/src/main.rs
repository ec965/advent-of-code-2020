use std::fs::File;
use std::io::{
    BufReader,
    prelude::*,
};

use day5::boarding_pass;

fn main() {
    let input = File::open("input.txt").unwrap();
    let input = BufReader::new(input);
    
    let mut tracker = [true; 995];
    let mut highest = 0;
    for i in input.lines(){
        let id = boarding_pass::get_id(i.unwrap());
        if id > highest {
            highest = id;
        }
        tracker[id as usize] = false;
    } 

    for i in 0..tracker.len()-1 {
        if tracker[i as usize] {
            println!("{}", i); //answer to part 2: look for the odd one out
        }
    }
    println!("part 1: highest seat id: {}", highest);
}

