// Enoch Chau
// Advent of code 2020 day 4 part 1
use std::fs;

fn passport_check(document: &Vec<&str>) -> bool{
    // println!();
    let mut attribute_count = 0;
    for attribute in document {
        println!("{}",attribute);
        let name = &attribute[0..3];
        let value = &attribute[4..];
        // println!("{} : {}", name, value);
        match name {
            "byr" => { 
                if !check_byr(value.parse::<u16>().unwrap()) {
                    return false;   
                }
                attribute_count += 1;
            }
            "iyr" => {
                if !check_iyr(value.parse::<u16>().unwrap()) {
                    return false;
                }
                attribute_count += 1;
            }
            "eyr" => {
                if !check_eyr(value.parse::<u16>().unwrap()) {
                    return false;
                }
                attribute_count += 1;
            }
            "hgt" => {
                match check_hgt(value.to_string()){
                    Ok(x) => {
                        if !x{
                            return false;
                        }
                    }
                    Err(_e) => {
                        println!("Error while checking height!");
                        return false;
                    }
                }
                attribute_count += 1;
            }
            "hcl" => {
                if !check_hcl(value.to_string()) {
                    return false;
                }
                attribute_count += 1;
            }
            "ecl" => {
                if !check_ecl(value.to_string()) {
                    return false;
                }
                attribute_count += 1;
            }
            "pid" => {
                if !check_pid(value.to_string()) {
                    return false;
                }
                attribute_count += 1;
            }
            "cid" => (), //ignore cid
            _ => return false
        }
    }

    if attribute_count < 7 {
        return false;
    }
    true
}

fn check_byr(byr:u16) -> bool{
    if 1920 <= byr && byr <= 2002 {
        return true;
    }
    false
}

fn check_iyr(iyr:u16) -> bool {
    if 2010 <= iyr && iyr <= 2020 {
        return true;
    }
    false
}

fn check_eyr(eyr:u16) -> bool {
    if 2020 <= eyr && eyr <= 2030 {
        return true;
    }
    false
}

fn check_hgt(hgt:String) -> Result<bool, std::num::ParseIntError> {
    // set the min and max
    let unit:&str = &hgt[hgt.len()-2..];
    let min:u8;
    let max:u8;
    if unit == "cm" {
        min = 150;
        max = 193; 
    } else if unit == "in" {
        min = 59;
        max = 76;
    } else {
        return Ok(false);
    } 

    // print!("height\tmin: {}\tmax: {}", min, max);

    // handle parsing errors
    match &hgt[..hgt.len()-2].parse::<u8>(){
        Ok(h) => {
            let h = h.clone();
            if min <= h && h <= max {
               return Ok(true); 
            }
            Ok(false)
        }
        Err(_e) => {
            let e = _e.clone();
            return Err(e)
        }
    }
}

fn check_hcl(hcl:String) -> bool{
    for (i,c) in hcl.chars().enumerate() {
        if i == 0 {
            match c{
                '#' => continue,
                _ => return false,
            }
        } else if 1 <= i && i <= 6 {
            match c{
                '0' => continue,
                '1' => continue,
                '2' => continue,
                '3' => continue,
                '4' => continue,
                '5' => continue,
                '6' => continue,
                '7' => continue,
                '8' => continue,
                '9' => continue,
                'a' => continue,
                'b' => continue,
                'c' => continue,
                'd' => continue,
                'e' => continue,
                'f' => continue,
                _ => return false,
            }
        } else {
          return false;  
        }
    }
    true 
}

fn check_ecl(ecl:String) -> bool {
    //take a string slice of the whole string to match the types
    //alternatively, you could convert all the eye colors to strings using .to_string()
    //but that's too much typing
    // println!("ecl: {}", &ecl[..]);
    match &ecl[..] {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
    }
}

fn check_pid(pid:String) -> bool {
    if pid.chars().count() == 9 {
        match pid[1..].parse::<u32>() {
            Ok(_n) => return true,
            Err(_e) => return false,
        }
    }
    false
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
        println!("{:#?}", d);
        if passport_check(&d) {
            valid += 1;
        }
    }

    println!("valid passports: {}", valid);
}

