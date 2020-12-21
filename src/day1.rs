use std::fs::File;
use std::io::prelude::*;

pub fn execute() -> std::io::Result<()> {
    let mut f = File::open("resources/day1.txt")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let nums: Vec<u32> = contents
        .split("\r\n")
        .map(|line| line.parse::<u32>())
        .filter_map(Result::ok)
        .collect();

    for a in &nums {
        for b in &nums {
            if a + b == 2020 {
                println!("{}", a * b);
            }
            for c in &nums {
                if a + b + c == 2020 {
                    println!("{}", a * b * c);
                }
            }
        }
    }

    Ok(())
}
