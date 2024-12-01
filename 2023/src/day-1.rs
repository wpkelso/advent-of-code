use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::path::Path;
use unicode_segmentation::UnicodeSegmentation;

use regex::Regex;

fn main(){
    let re_first_digit = Regex::new(r"^[[:alpha:]]*?(\d|one|two|three|four|five|six|seven|eight|nine|zero|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|orez)").unwrap();

    let mut sum: i32 = 0;

    if let Ok(lines) = read_lines("day-1-input.txt") {
        for line in lines {
            if let Ok(target) = line {
                let target_rev: String = target.graphemes(true).rev().collect(); // This is a
                                                                                 // horrible
                                                                                 // solution that
                                                                                 // happens to work
                //println!("{}", target);
                //println!("P{}", target_rev);

                let Some(digit_1) = re_first_digit.captures(&target) else { panic!("Failed to grab first digit") };
                //println!("Digit 1: {}", &digit_1[1]);
                let num_1: &str;
                if digit_1[1].len() > 1 {
                    num_1 = name_to_num(&digit_1[1]);
                } else {
                    num_1 = &digit_1[1];
                }

                let Some(digit_2) = re_first_digit.captures(&target_rev) else { panic!("Failed to grab second digit") };
                //println!("Digit 2: {}", &digit_2[1]);
                let num_2: &str;
                if digit_2[1].len() > 1 {
                    num_2 = name_to_num(&digit_2[1]);
                } else {
                    num_2 = &digit_2[1];
                }

                let val: i32 = (num_1.parse::<i32>().unwrap() * 10) + num_2.parse::<i32>().unwrap();
                sum += val;
                //println!("Current sum is: {}", sum);
            }
        }
    }
    let sum: String = sum.to_string();
    println!("Sum total: {}", &sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn name_to_num(input: &str) -> &str {
    match input {
        "one"|"eno"=>"1",
        "two"|"owt"=>"2",
        "three"|"eerht"=>"3",
        "four"|"ruof"=>"4",
        "five"|"evif"=>"5",
        "six"|"xis"=>"6",
        "seven"|"neves"=>"7",
        "eight"|"thgie"=>"8",
        "nine"|"enin"=>"9",
        "zero"|"orez"=>"0",
        _=>panic!("Error in name_to_num(), no value found. Input was {}", input)
    }
}
