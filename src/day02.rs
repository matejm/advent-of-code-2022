use std::collections::HashMap;
use std::fs;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOOSE: i32 = 0;

fn play_game(a: i32, b: i32) -> i32 {
    match (a, b) {
        (ROCK, ROCK) | (PAPER, PAPER) | (SCISSORS, SCISSORS) => DRAW,
        (ROCK, PAPER) | (PAPER, SCISSORS) | (SCISSORS, ROCK) => WIN,
        (PAPER, ROCK) | (SCISSORS, PAPER) | (ROCK, SCISSORS) => LOOSE,
        (_, _) => panic!("Unexpected value"),
    }
}

fn get_score(a: i32, b: i32) -> i32 {
    b + play_game(a, b)
}

fn guess_move(a: i32, target: i32) -> i32 {
    target
        + (if play_game(a, ROCK) == target {
            ROCK
        } else if play_game(a, PAPER) == target {
            PAPER
        } else if play_game(a, SCISSORS) == target {
            SCISSORS
        } else {
            panic!("Unexpected game end")
        })
}

pub fn day02() {
    let lines = fs::read_to_string("inputs/02.in").expect("Could not read file");
    let lines = lines.lines();

    let mapping_a = HashMap::from([("A", ROCK), ("B", PAPER), ("C", SCISSORS)]);
    let mapping_b = HashMap::from([("X", ROCK), ("Y", PAPER), ("Z", SCISSORS)]);
    let mapping_result = HashMap::from([("X", LOOSE), ("Y", DRAW), ("Z", WIN)]);

    let mut score = 0;
    let mut score2 = 0;

    for line in lines {
        let mut line = line.split(" ");
        let a = line.next().unwrap();
        let b = line.next().unwrap();

        score += get_score(mapping_a[a], mapping_b[b]);
        let s2 = guess_move(mapping_a[a], mapping_result[b]);
        score2 += s2;
    }

    println!("{}", score);
    println!("{}", score2);
}
