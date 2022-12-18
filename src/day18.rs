use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

const MAX_INTERIOR_SIZE: i32 = 2000;

static DIRECTIONS: &[(i32, i32, i32)] = &[
    (0, 0, 1),
    (0, 0, -1),
    (0, 1, 0),
    (0, -1, 0),
    (1, 0, 0),
    (-1, 0, 0),
];

fn is_interior(
    grid: &HashSet<(i32, i32, i32)>,
    start: (i32, i32, i32),
    memo: &mut HashMap<(i32, i32, i32), bool>,
) -> bool {
    if memo.contains_key(&start) {
        return memo[&start];
    }

    // BFS
    let mut q: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();

    let mut count = 1;

    // visit start.
    q.push_back(start.clone());
    visited.insert(start.clone());

    while !q.is_empty() {
        let a = q.pop_front().unwrap();

        if count > MAX_INTERIOR_SIZE {
            break;
        }

        // we already know if this is outside or not
        if memo.contains_key(&a) {
            return memo[&a];
        }

        for d in DIRECTIONS.iter() {
            let b = (a.0 + d.0, a.1 + d.1, a.2 + d.2);

            if grid.contains(&b) {
                // cannot go through rock
            } else if !visited.contains(&b) {
                // expand steam
                q.push_back(b.clone());
                visited.insert(b);
                count += 1;
            }
        }
    }

    // Interior - bfs finished
    memo.insert(start, count <= MAX_INTERIOR_SIZE);
    return memo[&start];
}

pub fn day18() {
    let lines = fs::read_to_string("inputs/18.in").expect("Could not read file");
    let lines = lines.lines();

    let mut grid: HashSet<(i32, i32, i32)> = HashSet::new();

    for line in lines {
        let data: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        let point = (data[0], data[1], data[2]);
        grid.insert(point);
    }

    // BFS
    let mut q: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();

    let mut sides = 0;
    let mut out_sides = 0;
    let mut memo = HashMap::new();

    for start in grid.iter() {
        // Check if start was not visited yet.
        if visited.contains(&start) {
            continue;
        }

        // Else visit start.
        q.push_back(start.clone());
        visited.insert(start.clone());

        while !q.is_empty() {
            let a = q.pop_front().unwrap();

            for d in DIRECTIONS.iter() {
                let b = (a.0 + d.0, a.1 + d.1, a.2 + d.2);

                if grid.contains(&b) {
                    // go to neighbor if not visited.
                    if !visited.contains(&b) {
                        visited.insert(b.clone());
                        q.push_back(b);
                    }
                } else {
                    // found a side
                    sides += 1;

                    if !is_interior(&grid, b, &mut memo) {
                        out_sides += 1;
                    }
                }
            }
        }
    }

    println!("{}", sides);
    println!("{}", out_sides);
}
