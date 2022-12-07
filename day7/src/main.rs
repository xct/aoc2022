use common::read_lines;
use std::collections::HashMap;
use regex::Regex;

fn main() {        
    // Example: Map<"/a/e/../../d/file.txt",4000>
    let mut fs: HashMap<String,i32> = HashMap::new();
    let re = Regex::new(r"(\d+) ([a-z\.]+)").unwrap();
    let limit = 100000;

    // read & prase input data
    let mut cwd: String = "".to_string();
    for line in read_lines("input.txt").unwrap() {
        let line = line.unwrap();
        let mut key = ""; 
        let mut temp_cwd;
        // handle input
        if line.contains("$") {
            if line.starts_with("$ cd") {
                // changing cwd
                temp_cwd = line.split("cd ").collect::<Vec<&str>>()[1];  
                if temp_cwd.ends_with("/"){
                    cwd = format!("{cwd}{temp_cwd}");          
                } else if temp_cwd.contains(".."){
                    // move up one dir
                    let mut cwd_split = cwd.split("/").collect::<Vec<&str>>();
                    cwd_split.pop();
                    cwd_split.pop();
                    cwd = cwd_split.join("/");
                    cwd = format!("{cwd}/");
                } else {
                    cwd = format!("{cwd}{temp_cwd}/");   
                }                
                //println!("cd: {:?}",cwd);               
            }
            if line.contains("$ ls") {
                // listing contents, can be ignored does not change state
            }
        
        } else {      
        // handle output 
            if line.contains("dir") {
                // it's a directory
                key = line.split('-').next().unwrap();
                key = &key[4..line.len()];
                //println!("dir {:?}", key);
            } else {
                // it's a file
                let caps = re.captures(&line).unwrap();
                let fsize: i32 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let fname: &str = caps.get(2).unwrap().as_str();
                //println!("{} {}", fsize, fname);
                fs.insert(format!("{cwd}§{fname}"),fsize);
            }
        }
    };
    println!("{:?}", fs);
    
    // sum up sizes per dir, no tree required we store abc ab a
    let mut dir_sizes: HashMap<String,i32> = HashMap::new();
    for node in fs.into_iter() {
        let key = node.0;
        let value = node.1;       
        let mut key_split = key.split("/").collect::<Vec<&str>>();
        key_split = key_split[0..key_split.len()-1].to_vec();

        for i in 0..key_split.len() {
            let mut cur_key: String = key_split[0..i+1].join("");            
            //println!("Adding {} to {:?}", value, cur_key);
            
            if !dir_sizes.contains_key(&cur_key){
                dir_sizes.insert(cur_key, value);
            }else{
                dir_sizes.entry(cur_key).and_modify(|v| { *v += value});
            }
        }
    }
    println!("{:?}", dir_sizes);

    let mut total_part_1 = 0;
    for kv in dir_sizes.iter() {
        if kv.1 < &limit {
            total_part_1 += kv.1;
        }
    }
    println!("Part1: {}", total_part_1); // 1307902


    let size_limit = 70000000 - 30000000;
    let mut smallest = size_limit;
    let total = dir_sizes[""];    
    for kv in dir_sizes.iter() {
        if total - kv.1 < size_limit {
            if kv.1 < &smallest {
                smallest = *kv.1;
            }
        }
    }    
    println!("Part2: {}", smallest); // 7068748   
}
