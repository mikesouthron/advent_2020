use std::fs::File;
use std::io::prelude::*;

fn valid_line(line: &str) -> bool {
    let space_split: Vec<&str> = line.split(" ").collect();
    let nums: Vec<usize> = space_split[0]
        .split("-")
        .map(|x| x.parse::<usize>())
        .filter_map(Result::ok)
        .collect();

    if nums.len() == 0 {
        return false;
    }

    let start = nums[0];
    let end = nums[1];
    let match_char = space_split[1].chars().nth(0).unwrap();
    let match_count = space_split[2].chars().filter(|c| c == &match_char).count();
    match_count >= start && match_count <= end
}

fn valid_line_part_2(line: &str) -> bool {
    let space_split: Vec<&str> = line.split(" ").collect();
    let nums: Vec<usize> = space_split[0]
        .split("-")
        .map(|x| x.parse::<usize>())
        .filter_map(Result::ok)
        .collect();

    if nums.len() == 0 {
        return false;
    }

    let start = nums[0] - 1;
    let end = (nums[1] - 1) - (start + 1);
    let match_char = space_split[1].chars().nth(0).unwrap();
    let mut str_chars = space_split[2].chars();

    let start_char = str_chars.nth(start).unwrap();
    let end_char = str_chars.nth(end).unwrap();

    (start_char == match_char && end_char != match_char)
        || (start_char != match_char && end_char == match_char)
}

pub fn execute() -> std::io::Result<()> {
    let mut f = File::open("resources/day2.txt")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let count = contents
        .split("\r\n")
        .filter(|line| valid_line(line))
        .count();

    println!("Part 1: {}", count);

    let count = contents
        .split("\r\n")
        .filter(|line| valid_line_part_2(line))
        .count();

    println!("Part 2: {}", count);

    Ok(())
}
