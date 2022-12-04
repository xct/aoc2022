use std::collections::HashSet;
use common::read_lines;

fn main() {
    let data: Vec<(HashSet<i32>,HashSet<i32>)> = read_lines("input.txt").unwrap().map(|line | {
        let line = line.unwrap();
        let mut split = line.split(",");
        let (a, b) = (split.next().unwrap(), split.next().unwrap());
        
        let mut split_a = a.split("-");
        let (a1, a2) = (split_a.next().unwrap().parse::<i32>().unwrap(), split_a.next().unwrap().parse::<i32>().unwrap());
        let a3 : Vec<i32> = (a1..a2+1).collect();

        let mut split_b = b.split("-");
        let (b1, b2) = (split_b.next().unwrap().parse::<i32>().unwrap(), split_b.next().unwrap().parse::<i32>().unwrap());
        let b3 : Vec<i32> = (b1..b2+1).collect();

        return (HashSet::from_iter(a3), HashSet::from_iter(b3));
    }).collect();

    let result: i32 = data.iter().map(|score_1 | {
        let (a, b) = score_1;
        let int: HashSet<_> = a.intersection(&b).map(|i| *i).collect();

        if int.eq(a) || int.eq(b) {
           //println!("{:?},{:?} => {:?}", a, b, int);
           return 1;
        }             
        return 0;
    }).sum();

    println!("Part 1: {}", result); // 534

    let result: i32 = data.iter().map(|score_1 | {
        let (a, b) = score_1;
        let int: HashSet<_> = a.intersection(&b).map(|i| *i).collect();

        if !int.is_empty() {
           //println!("{:?},{:?} => {:?}", a, b, int);
           return 1;
        }             
        return 0;
    }).sum();

    println!("Part 2: {}", result); // 841
}
