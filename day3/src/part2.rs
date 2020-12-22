// Enoch Chau
// Advent of code 2020 day 3 part 1
use std::fs::File;
use std::io::{
    BufReader, 
    prelude::*,
};

fn check_slope(slope: &Vec<Vec<char>>, right: usize, down: usize) -> u32 {
    let mut trees = 0;
    let mut index = 0;
    let v_size = slope[0].len();
    for s in slope.iter().skip(down).step_by(down) {
        index += right;
        index = index%v_size;
    
        match s[index as usize] {
            '#' => {
                trees += 1
            }
            _ => ()
        } 
    }
    trees
}

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

    let s1 = check_slope(&slope, 1, 1);
    let s2 = check_slope(&slope, 3, 1);
    let s3 = check_slope(&slope, 5, 1);
    let s4 = check_slope(&slope, 7, 1);
    let s5 = check_slope(&slope, 1, 2);

    println!("s1: {}\ns2: {}\ns3: {}\ns4: {}\ns5: {}", s1, s2, s3, s4, s5);

    println!("ans: {}", s1*s2*s3*s4*s5);
}
