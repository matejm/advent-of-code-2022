use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

pub fn day12() {
    let lines = fs::read_to_string("inputs/12.in").expect("Could not read file");
    let lines = lines.lines();

    let mut elevations: HashMap<(i32, i32), i32> = HashMap::new();

    let mut start = (-1, -1);
    let mut end = (-1, -1);

    for (j, line) in lines.enumerate() {
        for (i, mut char) in line.chars().enumerate() {
            if char == 'S' {
                start = (i as i32, j as i32);
                char = 'a';
            } else if char == 'E' {
                end = (i as i32, j as i32);
                char = 'z';
            }

            elevations.insert((i as i32, j as i32), char as i32);
        }
    }

    let mut distance: HashMap<(i32, i32), i32> = HashMap::new();
    let mut visited: HashMap<(i32, i32), bool> = HashMap::new();
    let mut q = VecDeque::new();

    q.push_back((start, 0));

    let directions: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    visited.insert(start, true);
    distance.insert(start, 0);

    while !q.is_empty() {
        let ((px, py), d) = q.pop_front().unwrap();
        if (px, py) == end {
            break;
        }

        for (x, y) in directions.iter() {
            let e = elevations.get(&(px + x, py + y));
            if e.is_none() {
                continue;
            }
            if visited.get(&(px + x, py + y)).is_none() && *e.unwrap() <= elevations[&(px, py)] + 1
            {
                visited.insert((px + x, py + y), true);
                distance.insert((px + x, py + y), d + 1);
                q.push_back(((px + x, py + y), d + 1));
            }
        }
    }

    println!("{}", distance[&end]);

    // Reverse
    visited.clear();
    q.clear();

    q.push_back((end, 0));
    visited.insert(end, true);

    while !q.is_empty() {
        let ((px, py), d) = q.pop_front().unwrap();
        if elevations[&(px, py)] == 'a' as i32 {
            println!("{}", d);
            break;
        }

        for (x, y) in directions.iter() {
            let e = elevations.get(&(px + x, py + y));
            if e.is_none() {
                continue;
            }
            if visited.get(&(px + x, py + y)).is_none() && *e.unwrap() + 1 >= elevations[&(px, py)]
            {
                visited.insert((px + x, py + y), true);
                q.push_back(((px + x, py + y), d + 1));
            }
        }
    }
}
