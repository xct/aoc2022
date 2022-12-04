#![feature(array_chunks)]

use std::collections::HashMap;
use common::read_lines;

fn score_1(line: &String) -> i32 {
    let mut priority = HashMap::from([
        ]); 
    for i in 0..26 {
        priority.insert(97+i, i+1);
        priority.insert(65+i, i+26+1);
    }

    let left = &line[0..line.len()/2];
    let right = &line[line.len()/2..line.len()];  
    
    for c1 in left.as_bytes() {                
        for c2 in right.as_bytes() {
            if c1==c2 {
                return priority[c1] as i32;
            }
        }
    }
    return 0;
}

fn score_2(group: &[String; 3]) -> i32 {
    let mut priority = HashMap::from([
        ]); 
    for i in 0..26 {
        priority.insert(97+i, i+1);
        priority.insert(65+i, i+26+1);
    } 
    
    for c1 in group[0].as_bytes() {                
        for c2 in group[1].as_bytes() {
            for c3 in group[2].as_bytes() {
                if c1==c2 && c3==c2 {
                    return priority[c1] as i32;
                }
            }
        }
    }
    return 0;
}

fn main() {
    let data: Vec<String> = read_lines("input.txt").unwrap().map(|line | line.unwrap()).collect();

    let total_1: i32 = data.iter().map(score_1).sum();
    println!("Part 1: {}", total_1); // 8139

    let total_2: i32 = data.array_chunks::<3>().map(| group | {
        return score_2(group);
    }).sum(); 
    println!("Part 2: {}", total_2); // 2668
}