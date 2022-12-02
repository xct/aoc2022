use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn play1(a: &str, b: &str) -> i32 {
    // normalize X Y Z back to A B C
    let normalize = HashMap::from([
        ("X", "A"), 
        ("Y", "B"),
        ("Z", "C"),
    ]); 
    let beats = HashMap::from([
        ("A", "B"), // Paper beats Rock
        ("B", "C"), // Rock beats Scissors
        ("C", "A"), // Scissors beats Paper
    ]);  
    let score = HashMap::from([
        ("A", 1), // Rock 
        ("B", 2), // Paper
        ("C", 3), // Scissors    
    ]); 
    let my = normalize[b];
    // win
    //println!("{},{},{}",a,my, beats[a]);
    if my == beats[a] {
        //println!("Win (Score: {})", score[my]+6);
        return score[my] + 6;
    }
    // draw
    if my == a {
        //println!("Draw (Score: {})", score[my]+3);
        return score[my] + 3;
    }
    // lost
    //println!("Loss (Score: {})", score[my]);
    return score[my];
}

fn play2(a: &str, b: &str) -> i32 {
    let beats = HashMap::from([
        ("A", "B"), // Paper beats Rock
        ("B", "C"), // Rock beats Scissors
        ("C", "A"), // Scissors beats Paper
    ]);  
    let loses = HashMap::from([
        ("B", "A"), 
        ("C", "B"), 
        ("A", "C"), 
    ]);  
    let score = HashMap::from([
        ("A", 1), // Rock 
        ("B", 2), // Paper
        ("C", 3), // Scissors    
    ]); 
    //println!("{},{},{}",a,b, beats[a]);
    // win 
    if b == "Z" {
        let my = beats[a];
        //println!("Win (Score: {})", score[my]+6);
        return score[my] + 6;
    }
    // draw
    if b == "Y" {
        let my = a;
        //println!("Draw (Score: {})", score[my]+3);
        return score[my] + 3;
    }
    // lose
    let my = loses[a];
    //println!("Loss (Score: {})", score[my]);
    return score[my]; 
}

fn main() {
    let mut total1 = 0;
    let mut total2 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line) = line {
                let words: Vec<&str> = line.split_whitespace().collect();              
                let a = words[0];
                let b = words[1];

                let result1 = play1(a, b);
                total1 += result1;

                let result2 = play2(a, b);
                total2 += result2;
            }
        }
    }
    println!("Score Part I  is: {}", total1); // 14163
    println!("Score Part II is: {}", total2); // 12091
}