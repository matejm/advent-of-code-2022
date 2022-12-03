use std::fs;

fn get_item(backback: &str) -> char {
    let first_compartment = &backback[0..backback.len() / 2];
    let second_compartment = &backback[backback.len() / 2..backback.len()];

    for c in first_compartment.chars() {
        if second_compartment.find(c).is_some() {
            return c;
        }
    }
    panic!("No matching characters.");
}

fn get_item2(b1: &str, b2: &str, b3: &str) -> char {
    for c in b1.chars() {
        if b2.find(c).is_some() && b3.find(c).is_some() {
            return c;
        }
    }
    panic!("No matching characters.");
}

fn get_score(c: char) -> u32 {
    if c.is_lowercase() {
        u32::from(c) - u32::from('a') + 1
    } else {
        u32::from(c) - u32::from('A') + 27
    }
}

pub fn day03() {
    let lines = fs::read_to_string("inputs/03.in").expect("Could not read file");
    let lines: Vec<&str> = lines.lines().collect();

    let mut score = 0;
    let mut score2 = 0;

    for line in lines.iter() {
        let item = get_item(line);

        score += get_score(item);
    }

    println!("{}", score);

    for i in (0..lines.len()).step_by(3) {
        let item = get_item2(lines[i], lines[i + 1], lines[i + 2]);

        score2 += get_score(item);
    }

    println!("{}", score2);
}
