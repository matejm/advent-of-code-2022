use std::collections::HashSet;
use std::fs;

fn all_different(chars: &str) -> bool {
    let s: HashSet<char> = chars.chars().collect();
    return s.len() == chars.len();
}

pub fn day06() {
    let lines = fs::read_to_string("inputs/06.in").expect("Could not read file");
    let mut lines = lines.lines();

    let line = lines.next().unwrap();

    for i in 0..line.len() {
        let chars = &line[i..(i + 4)];

        if all_different(chars) {
            println!("{}", i + 4);
            break;
        }
    }

    for i in 0..line.len() {
        let chars = &line[i..(i + 14)];

        if all_different(chars) {
            println!("{}", i + 14);
            break;
        }
    }
}
