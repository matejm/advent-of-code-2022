use std::{fs, vec};

pub fn day01() {
    let lines = fs::read_to_string("inputs/01.in").expect("Could not read file");
    let lines = lines.lines();

    // Group by elves
    let mut elves = vec![];
    let mut elf = 0;

    for line in lines {
        if line == "" {
            elves.push(elf);
            elf = 0;
        } else {
            elf += line.parse::<i64>().expect("not an integer");
        }
    }

    // 1st star
    println!("{:?}", elves.iter().max().unwrap());

    // 2nd star
    elves.sort();
    let n = elves.len();
    println!("{:?}", elves[n - 1] + elves[n - 2] + elves[n - 3]);
}
