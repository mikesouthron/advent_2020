use std::fs::File;
use std::io::prelude::*;

fn slope(lines: &Vec<&str>, width: usize, count: usize, right: usize, down: usize) -> u32 {
    let mut h_pos: usize = 0;
    let mut v_pos: usize = 0;

    let mut tree_count = 0;

    loop {
        h_pos = h_pos + right;
        if h_pos >= width {
            h_pos = h_pos - width;
        }
        v_pos = v_pos + down;

        if v_pos >= count {
            break;
        }
        //Performance could be improved by doing the as_bytes() when parsing the file.
        //However, could not figure out how to store a Vec<[u8]>, as rust arrays have to be compile time sized.
        if lines[v_pos].as_bytes()[h_pos] as char == '#' {
            tree_count = tree_count + 1;
        }
    }

    tree_count
}

pub fn execute() -> std::io::Result<()> {
    let mut f = File::open("resources/day3.txt")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let lines: Vec<&str> = contents.split("\r\n").collect();

    let width = lines[0].len();
    let count = lines.len() - 1;

    let part_1 = slope(&lines, width, count, 3, 1);
    println!("Part 1: {}", part_1);

    let a = slope(&lines, width, count, 1, 1);
    let b = slope(&lines, width, count, 5, 1);
    let c = slope(&lines, width, count, 7, 1);
    let d = slope(&lines, width, count, 1, 2);

    println!("Part 2: {}", (a * part_1 * b * c * d));

    Ok(())
}
