// Enoch Chau
// Advent of code 2020 day 1 part 2
//find the 3 entries that sum to 2020 and return their multiple
use std::fs::File;
use std::io::{
    self, 
    BufReader, 
    prelude::*,
};

fn find(inputs: &Vec<i32>) -> (i32, i32, i32){
    let mut ans = (0i32, 0i32, 0i32);
    for i in inputs {
        for j in inputs{
            if i + j >= 2020 {continue;}
            for k in inputs{
                if i + j + k == 2020 {
                    ans = (*i,*j,*k)
                }
            }
        }
    }
    ans
}

fn main() -> io::Result<()>{
    let file = File::open("inputs.txt")?;
    let reader = BufReader::new(file);
    let mut inputs = Vec::new();

    for line in reader.lines(){
        let n = line
            .unwrap()
            .to_string()
            .parse::<i32>().unwrap();
        inputs.push(n);
    }

    let ans = find(&inputs);
    println!("{} x {} x {} = {}", ans.0, ans.1, ans.2, ans.0*ans.1*ans.2);

    Ok(())
}
