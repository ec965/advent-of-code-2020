use day7::suitcase;
use std::collections::HashMap;
use std::fs::File;
use std::io::{
    BufReader,
    prelude::*,
};

fn find_case(rule_map:&HashMap<String,Vec<String>>, search_case: &str){
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let input = BufReader::new(input);
    
    let mut rule_map = HashMap::new();
    for i in input.lines(){
        let rule = suitcase::parse_rule(&i.unwrap());
        rule_map.insert(rule.0,rule.1);
    }
    // println!("{:#?}", rule_map);

    for (key, val) in rule_map.iter(){
        find_case(&rule_map, key); 
    }
}
