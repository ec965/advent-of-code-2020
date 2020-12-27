pub mod boarding_pass {
    use std::cmp::Ordering;
    // there are 128 row -> 0 to 127
    // there are 8 col -> 0 to 7
    pub fn get_id (s:String) -> u16 {
        let mut row: (u16,u16) = (0, 127);
        let mut col: (u16,u16) = (0, 7);
        for c in s.chars(){
            match c {
                'F' => row.1 -= ((row.1 - row.0)/2) + 1, 
                'B' => row.0 += ((row.1 - row.0)/2) + 1,  
                'L' => col.1 -= ((col.1 - col.0)/2) + 1,
                'R' => col.0 += ((col.1 - col.0)/2) + 1,
                _ => {
                    panic!("invalid boarding pass!");
                }
            }
        }
        // println!("row\tmin: {}\tmax: {}", row.0, row.1);
        // println!("col\tmin: {}\tmax: {}", col.0, col.1);
        match row.0.cmp(&row.1){
            Ordering::Greater => row.0 = row.1,
            _ => (),
        }
        match col.0.cmp(&col.1){
            Ordering::Greater => col.0 = col.1,
            _ => (),
        }
        row.0 * 8 + col.0
   }
}
#[cfg(test)]
mod tests {
//    use std::fs::File;
    use crate::*;
    #[test]
    fn test_seat_id() {
        let input = String::from("FBFBBFFRLR");
        let seat_id = boarding_pass::get_id(input);
        assert_eq!(seat_id, 357);
    }
}

