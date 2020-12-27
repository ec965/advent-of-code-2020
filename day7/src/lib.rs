pub mod suitcase {
    use regex::Regex;
    // parses the suitcase rule and returns a key, value pair as a tuple
    // the key is the parent suitcase, value is a vector of the child suitcases
    pub fn parse_rule(s:&str)->(String, Vec<String>){
        // match 'bag', 'bags', 'contain', and all non-alphas
        let re = Regex::new(r"(\bbags\b|\bbag\b|\bcontain\b|[^a-zA-Z\s])").unwrap();
        let mut s = re.replace_all(s, "");

        // match spaces
        let re = Regex::new(r"([\s])").unwrap();
        let s = re.split(s.to_mut())
            .filter(|x| x != &"")
            .collect::<Vec<&str>>();

        let mut v_s = Vec::new();
        for (i,val) in s.iter().enumerate().step_by(2){
            v_s.push(val.to_string() + s[i+1]);
        }

        (v_s.remove(0),v_s)
    }
}
#[cfg(test)]
mod test {
    use crate::suitcase;

    #[test]
    fn test_suitcase_rule(){
        let test = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let test2 = "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.";
        let test3 = "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.";
        let test4 = "faded blue bags contain no other bags.";

        let (key, value) = suitcase::parse_rule(test);
        assert_eq!(String::from("lightred"), key);
        assert_eq!(String::from("brightwhite"),value[0]);
        assert_eq!(String::from("mutedyellow"),value[1]);

        let (key,value) = suitcase::parse_rule(test2);
        assert_eq!(String::from("mutedyellow"), key);
        assert_eq!(String::from("shinygold"),value[0]);
        assert_eq!(String::from("fadedblue"),value[1]);

        let (key,value) = suitcase::parse_rule(test3);
        assert_eq!(String::from("vibrantplum"), key);
        assert_eq!(String::from("fadedblue"),value[0]);
        assert_eq!(String::from("dottedblack"),value[1]);

        let (key,value) = suitcase::parse_rule(test4);
        assert_eq!(String::from("fadedblue"), key);
        assert_eq!(String::from("noother"),value[0]);
    }
    
}
