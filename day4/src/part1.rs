// Enoch Chau
// Advent of code 2020 day 4 part 1
use std::fs;

struct Passport{
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
    cid: bool,
}

impl Passport{
    fn from(document:&Vec<&str>) -> Passport {
        let mut byr=false;
        let mut iyr=false;
        let mut eyr=false;
        let mut hgt=false;
        let mut hcl=false;
        let mut ecl=false;
        let mut pid=false;
        let mut cid=false;

        println!();
        for attribute in document{
            println!("{}", &attribute[0..3]);
            match &attribute[0..3] {
                "byr" => byr=true,
                "iyr" => iyr=true,
                "eyr" => eyr=true,
                "hgt" => hgt=true,
                "hcl" => hcl=true,
                "ecl" => ecl=true,
                "pid" => pid=true,
                "cid" => cid=true,
                _ => println!("attribute not found!")
            }
        }
        Passport {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            cid,
        }
    }
    fn check(&self)-> bool {
        if !self.byr {
            return false
        }
        if !self.iyr{
            return false
        }
        if !self.eyr{
            return false
        }
        if !self.hgt{
            return false
        }
        if !self.hcl{
            return false
        }
        if !self.ecl{
            return false
        }
        if !self.pid{
            return false
        }
        //ignore cid
        true
    }
}

fn main(){
    let docs = fs::read_to_string("input.txt")
        .expect("failed to read the input file");
    
    let docs: Vec<&str> = docs.split("\n\n").collect();

    let mut doc_v: Vec<Vec<&str>> = Vec::new();

    for d in docs{
        doc_v.push(d.split(|c| c=='\n' || c==' ').collect());
    }
    // println!("{:#?}", doc_v);

    let mut valid = 0;
    for d in doc_v{
        let p = Passport::from(&d);
        if p.check() {
            valid += 1;
        }
    }

    println!("valid passports: {}", valid);
}

