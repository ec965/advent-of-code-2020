// Enoch Chau
// Advent of code 2020 day 1 part 1
// find the two entries that sum to 2020 and multiply the numbers together
use std::fs::File;
use std::io::{
    self, 
    BufReader, 
    prelude::*,
};
use std::collections::HashSet;

fn find(inputs: &HashSet<i32>) -> (i32, i32){
    let mut ans = (0i32, 0i32);
    for n in inputs {
        let find = 2020 - n;
        match inputs.get(&find){
            Some(&number) => {
                ans.0 = *n;
                ans.1 = number;
                break
            },
            _ => (),
        }
    }
    ans
}

fn main() -> io::Result<()>{
    let file = File::open("inputs.txt")?;
    let reader = BufReader::new(file);
    let mut inputs =HashSet::new();

    for line in reader.lines(){
        let n = line
            .unwrap()
            .to_string()
            .parse::<i32>().unwrap();
        inputs.insert(n);
    }
//    println!("{:?}", input);

    let ans = find(&inputs);
    println!("{} x {} = {}", ans.0, ans.1, ans.0 * ans.1);

    Ok(())
}
