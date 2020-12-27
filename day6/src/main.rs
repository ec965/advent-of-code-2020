use std::fs;
use std::collections::HashSet;

fn main() {
    // read in the inputs
    let forms = fs::read_to_string("input.txt").expect("failed to read input.txt");
    let forms: Vec<&str> = forms.split("\n\n").collect();
    // put the inputs in a 2d vector for groups travelling together and their answers
    let mut group_forms:Vec<Vec<&str>> = Vec::new();
    for form in forms {
        group_forms.push(form.split(|c| c=='\n').collect());
    }
    
    // println!("{:#?}", group_forms);

    let mut count = 0;
    for forms in group_forms { //forms is the group of forms from people travelling together
        let mut group_ans = HashSet::new();
        for f in forms { // f is the string of letters that is one person's answers
            for c in f.chars() {
                if group_ans.insert(c.clone()) { //if c is new to the set
                    count += 1; //add 1 to the total count
                }
            }            
        }
    }

    println!("part 1: total count: {}", count);
}
