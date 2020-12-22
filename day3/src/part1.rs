// Enoch Chau
// Advent of code 2020 day 3 part 1
use std::fs::File;
use std::io::{
    self, 
    BufReader, 
    prelude::*,
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut slope = Vec::new();
    for line in reader.lines(){
        let n = line
            .unwrap()
            .to_string();
        let mut vec_str = Vec::new();
        for c in n.chars(){
            vec_str.push(c);
        }
        slope.push(vec_str);
    }

    let mut trees = 0;
    let mut index: i32 = 0;
    let v_size = slope[0].len() as i32;
    for (i,s) in slope.iter().skip(1).enumerate() {
        index += 3;
        index = index%v_size;
        
        println!("i:{}\tindex:{}",i,index);

        match s[index as usize] {
            '#' => {
                println!("Tree!");
                trees += 1
            }
            _ => ()
        } 
    }

    println!("trees: {}", trees);
}
