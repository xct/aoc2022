use common::read_lines;

fn find_marker(marker_len: usize) -> usize {
    // read input data
    let raw: Vec<String> = read_lines("input.txt").unwrap().map(|line | {
        return line.unwrap();
    }).collect();
    let data = &raw[0];

    // find marker
    let mut stack: Vec<char> = Vec::new();
    let mut marker_pos = 0;
    'out: for c in data.chars(){        
        // get a slice of the last chars that could make up a marker
        if stack.len() > marker_len-1 {
            let stack_range = &stack[stack.len()-marker_len..stack.len()];
            // compare each char with each char
            let mut found = 0;
            for j in stack_range {
                for k in stack_range {
                    if j == k {
                        found += 1;
                        continue;
                    }
                }
            }
            if found == marker_len {
                marker_pos = stack.len();
                println!("Found marker at position {} {:?} ",marker_pos, stack_range);                
                break 'out;
            }
        }
        stack.push(c);
    }
    return marker_pos;
}

fn main() {    
    println!("Part1: {}", find_marker(4));  // 1623
    println!("Part1: {}", find_marker(14)); // 3774
}
