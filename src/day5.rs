use std::fs::File;
use std::io::prelude::*;

pub fn execute() -> std::io::Result<()> {
    let mut f = File::open("resources/day5.txt")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let lines: Vec<&str> = contents.split("\r\n").collect();

    let mut high_id = 0_f32;

    let mut ids: Vec<i32> = Vec::new();

    for line in lines {
        let mut low_col = 0_f32;
        let mut high_col = 7_f32;
        let mut low_row = 0_f32;
        let mut high_row = 127_f32;
        for c in line.chars() {
            match c {
                'F' => high_row = ((high_row + low_row) / 2_f32).floor(),
                'B' => low_row = ((high_row + low_row) / 2_f32).ceil(),
                'L' => high_col = ((high_col + low_col) / 2_f32).floor(),
                'R' => low_col = ((high_col + low_col) / 2_f32).ceil(),
                _ => (),
            }
        }

        let id = (low_row * 8_f32) + low_col;

        if id > high_id {
            high_id = id;
        }

        ids.push(id as i32);
    }

    ids.sort();

    let mut prev: i32 = 0;

    for id in ids {
        if id == 0 && prev == 0 {
            continue;
        }

        if id - prev != 1 {
            println!("Part 2: {}", id - 1);
        }

        prev = id;
    }

    println!("Part 1: {}", high_id);

    Ok(())
}
