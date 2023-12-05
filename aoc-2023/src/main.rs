use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::path::Path;

use regex::Regex;

fn main(){
    let re_first_digit = Regex::new(r"^[[:alpha:]]*?(\d)[A-Za-z0-9]*?").unwrap();
    let re_last_digit = Regex::new(r"^[[:alpha:]0-9]*?(\d)[[:alpha:]]*?$").unwrap();

    let mut sum: i32 = 0;

    if let Ok(lines) = read_lines("day-1-input.txt") {
        for line in lines {
            if let Ok(target) = line {
                let Some(digit_1) = re_first_digit.captures(&target) else { return };
                let Some(digit_2) = re_last_digit.captures(&target) else { return };

                let val: i32 = (&digit_1[1].parse::<i32>().unwrap() * 10) + &digit_2[1].parse::<i32>().unwrap();
                sum += val;
            }
        }
    }
    let sum: String = sum.to_string();
    println!("{}", &sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
