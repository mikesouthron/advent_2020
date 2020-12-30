extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

pub fn execute() -> std::io::Result<()> {
    let mut f = File::open("resources/day4.txt")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let lines: Vec<&str> = contents.split("\r\n").collect();

    let mut byr = false;
    let mut iyr = false;
    let mut eyr = false;
    let mut hgt = false;
    let mut hcl = false;
    let mut ecl = false;
    let mut pid = false;

    let mut valid_count = 0;

    let color_regex = Regex::new(r"#[0-9a-f]{6}$").unwrap();
    let pid_regex = Regex::new(r"^\d{9}$").unwrap();

    for line in lines {
        let params: Vec<Vec<&str>> = line.split(" ").map(|s| s.split(":").collect()).collect();
        if params[0][0] == "" {
            if byr && iyr && eyr && hgt && hcl && ecl && pid {
                valid_count = valid_count + 1;
            }
            byr = false;
            iyr = false;
            eyr = false;
            hgt = false;
            hcl = false;
            ecl = false;
            pid = false;
        }

        for param in params {
            match param[0] {
                "byr" => {
                    let val = param[1].parse::<u32>().unwrap();
                    if val >= 1920 && val <= 2002 {
                        byr = true;
                    }
                }
                "iyr" => {
                    let val = param[1].parse::<u32>().unwrap();
                    if val >= 2010 && val <= 2020 {
                        iyr = true;
                    }
                }
                "eyr" => {
                    let val = param[1].parse::<u32>().unwrap();
                    if val >= 2020 && val <= 2030 {
                        eyr = true;
                    }
                }
                "hgt" => {
                    if param[1].ends_with("in") {
                        let val = param[1].strip_suffix("in").unwrap().parse::<u32>().unwrap();
                        if val >= 59 && val <= 76 {
                            hgt = true;
                        }
                    } else if param[1].ends_with("cm") {
                        let val = param[1].strip_suffix("cm").unwrap().parse::<u32>().unwrap();
                        if val >= 150 && val <= 193 {
                            hgt = true;
                        }
                    }
                }
                "hcl" => {
                    if color_regex.is_match(param[1]) {
                        hcl = true;
                    }
                }
                "ecl" => {
                    //amb blu brn gry grn hzl oth
                    ecl = match param[1] {
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
                "pid" => {
                    if pid_regex.is_match(param[1]) {
                        pid = true;
                    }
                }
                _ => (),
            }
        }
    }

    println!("Part 2: {}", valid_count);

    Ok(())
}
