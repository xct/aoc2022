use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1() {
    // input: number of calories each elf is carrying, per item, separated by blank line
    // result: how many calories on elf with most calories  
    let mut max = 0;
    let mut cur = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 {
                    if cur > max {
                        max = cur;
                    }
                    cur = 0;
                    continue;
                }
                let value = line.parse::<i32>().unwrap();
                cur += value;                
            }
        }
    }
    println!("Max:  {}", max);
}

fn part2() {
    // top 3 added together instead of max
    let mut totals = Vec::new();
    let mut cur = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 {
                    totals.push(cur);
                    cur = 0;                    
                    continue;
                }
                let value = line.parse::<i32>().unwrap();
                cur += value;                
            }
        }
    }
    totals.sort_by(|a, b| b.cmp(a));
    println!("Top3: {}", totals[0]+totals[1]+totals[2]);
}

fn main() {
    part1(); // 64929
    part2(); // 193697
}