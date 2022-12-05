use common::read_lines;
use regex::Regex;

fn main() {
    let stack_size = 9;
    let mut is_action = false;

    let mut map: Vec<String> = read_lines("input.txt").unwrap().filter_map(|line | {
        let line = line.unwrap();
        if line.len() == 0 {
            is_action= true;
        }
        if !is_action== true {
            return Some(line);
        }
        return None;
    }).collect();
    map = map.into_iter().rev().collect();
        
    let mut stack_init: Vec<Vec<char>> = Vec::new();
    for _ in 0..stack_size {
        let v : Vec<char> = Vec::new();
        stack_init.push(v);
    }    
    
    // -- Part 1 --    
    for line in map {        
        if line.contains("1") {
            // pass, we dont care about index line
        } else {
            let mut i = 0;
            let mut last = '?';
            let mut slast = '?';
            let mut tlast = '?';
            // parse one line into their different stacks
            for j in 0..line.len() {
                let cur = line.chars().nth(j).unwrap();
                // push letters
                if last == '[' {
                    stack_init[i].push(cur.to_owned());
                    i += 1;
                }
                // dont push empty but increment stack pointer
                if cur == ' ' && last == ' ' && slast == ' ' && tlast == ' ' {
                    last = cur;
                    slast = '?';
                    tlast = '?';
                    i += 1;
                } else {
                    tlast = slast;
                    slast = last;         
                    last = cur;
                } 
            }            
        }
    };    

    let mut stack_1 = stack_init.clone();
    is_action= false;
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in read_lines("input.txt").unwrap() {
        let line = line.unwrap();
        if line.len() == 0 {
            is_action= true;
            continue;
        }
        if is_action== true {
            let caps = re.captures(&line).unwrap();
            let amount: i32 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let from: usize = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let to: usize = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

            for _ in 0..amount {
                let cur = stack_1[from-1].pop().unwrap();
                stack_1[to-1].push(cur);
            }          
        }
    };

    let mut result_list: Vec<char> = Vec::new();
    for i in 0..stack_size { 
        result_list.push(stack_1[i][stack_1[i].len()-1]);
    }
    let part_1: String = result_list.into_iter().collect();
    println!("Part 1: {}", part_1); // WHTLRMZRC

    // -- Part 2 --
    let mut stack_2 = stack_init;
    is_action= false;
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in read_lines("input.txt").unwrap() {
        let line = line.unwrap();
        if line.len() == 0 {
            is_action= true;
            continue;
        }
        if is_action== true {
            let caps = re.captures(&line).unwrap();
            let amount: i32 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let from: usize = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let to: usize = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

            let mut temp: Vec<char> = Vec::new();
            for _ in 0..amount {
                let cur = stack_2[from-1].pop().unwrap();
                temp.push(cur);                
            }
            temp = temp.into_iter().rev().collect();
            for cur in temp {
                stack_2[to-1].push(cur);
            }        
        }
    };

    let mut result_list: Vec<char> = Vec::new();
    for i in 0..stack_size { 
        result_list.push(stack_2[i][stack_2[i].len()-1]);
    }
    let part_2: String = result_list.into_iter().collect();
    println!("Part 2: {}", part_2); // GMPMLWNMG

}
