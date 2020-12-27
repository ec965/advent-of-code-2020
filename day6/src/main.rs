// for part2 : add everything to a HashMap with key as the char, val as the count
// if count == # of people in the group then add 1 to count2
use std::fs;
use std::collections::{HashMap, HashSet};

fn main() {
    // read in the inputs
    let mut forms = fs::read_to_string("input.txt").expect("failed to read input.txt");
    forms.pop();
    let forms: Vec<&str> = forms.split("\n\n").collect();
    // println!("{:#?}", forms);
    // put the inputs in a 2d vector for groups travelling together and their answers
    let mut group_forms:Vec<Vec<&str>> = Vec::new();
    for form in forms {
        group_forms.push(form.split(|c| c=='\n').collect());
    }
    
    // println!("{:#?}", group_forms);

    let mut count1 = 0;
    let mut count2=0;
    for forms in group_forms { //forms is the group of forms from people travelling together
        let group_size = forms.len();
        let mut group_ans_set = HashSet::new();
        let mut all_yes_map = HashMap::new();
        // println!("{:#?}", forms);
        for f in forms { // f is the string of letters that is one person's answers
            for c in f.chars() {
                if group_ans_set.insert(c.clone()) { //if c is new to the set
                    count1 += 1; //add 1 to the total count
                }
                // increment the value at key 'c'
                let ans = all_yes_map.entry(c.clone()).or_insert(0);
                *ans += 1;
            }            
        }
        // println!("{:#?}", all_yes_map);
        // after finishing with the group,
        // if the value is equal to the number of people in the group, increment count2
        for (_key, value) in &all_yes_map {
            if *value == group_size {
                count2 += 1;
            }
        }
        // println!("count2: {}",count2);

    }

    println!("part 1: total count: {}", count1);
    println!("part 2: total count: {}", count2);
}
