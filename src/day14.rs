use std::cmp::{max, min};
use std::{collections::HashSet, fs};

fn parse_pair(pair: &str) -> (i32, i32) {
    let mut p = pair.split(",").map(|x| x.parse::<i32>().unwrap());
    (p.next().unwrap(), p.next().unwrap())
}

fn fill(rocks: &mut HashSet<(i32, i32)>, curr: &(i32, i32), next: &(i32, i32)) {
    for i in min(curr.0, next.0)..max(curr.0, next.0) + 1 {
        for j in min(curr.1, next.1)..max(curr.1, next.1) + 1 {
            rocks.insert((i, j));
        }
    }
}

fn fill_sand(
    rocks: &mut HashSet<(i32, i32)>,
    position: &(i32, i32),
    floor: i32,
    stop_at_floor: bool,
) -> bool {
    if position.1 > floor || rocks.contains(position) {
        false
    } else {
        let above_floor = !stop_at_floor || position.1 < floor - 1;

        if !rocks.contains(&(position.0, position.1 + 1)) && above_floor {
            // Fall down
            fill_sand(rocks, &(position.0, position.1 + 1), floor, stop_at_floor)
        } else if !rocks.contains(&(position.0 - 1, position.1 + 1)) && above_floor {
            // Fall left
            fill_sand(
                rocks,
                &(position.0 - 1, position.1 + 1),
                floor,
                stop_at_floor,
            )
        } else if !rocks.contains(&(position.0 + 1, position.1 + 1)) && above_floor {
            // Fall right
            fill_sand(
                rocks,
                &(position.0 + 1, position.1 + 1),
                floor,
                stop_at_floor,
            )
        } else {
            // Fill
            rocks.insert(*position);
            true
        }
    }
}

pub fn day14() {
    let lines = fs::read_to_string("inputs/14.in").expect("Could not read file");
    let lines = lines.lines();

    let mut rocks = HashSet::new();
    let mut inf = 0;

    // Read input and fill lines
    for line in lines {
        let mut positions = line.split(" -> ").map(parse_pair);

        let mut curr = positions.next().unwrap();
        inf = max(inf, curr.1);

        for next in positions {
            fill(&mut rocks, &curr, &next);
            curr = next;

            inf = max(inf, next.1);
        }
    }

    // fill sand
    let mut count = 0;
    while fill_sand(&mut rocks, &(500, 0), inf, false) {
        count += 1;
    }

    println!("{}", count);

    let floor = inf + 2;

    while fill_sand(&mut rocks, &(500, 0), floor, true) {
        count += 1;
    }

    println!("{}", count);
}
