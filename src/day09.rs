use std::{cmp::max, collections::HashMap, fs};

fn move_tail(head_x: i32, head_y: i32, tail_x: i32, tail_y: i32) -> (i32, i32) {
    let dx = head_x - tail_x;
    let dy = head_y - tail_y;

    if max(dx.abs(), dy.abs()) == 1 {
        (0, 0)
    } else {
        (dx.clamp(-1, 1), dy.clamp(-1, 1))
    }
}

pub fn day09() {
    let lines = fs::read_to_string("inputs/09.in").expect("Could not read file");
    let lines: Vec<&str> = lines.lines().collect();

    let mut head_x = 0;
    let mut head_y = 0;
    let mut tail_x = 0;
    let mut tail_y = 0;
    let mut visited: HashMap<(i32, i32), bool> = HashMap::new();

    for line in lines.iter() {
        let mut s = line.split(" ");
        let direction = s.next().unwrap();
        let distance = s.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..distance {
            // Parse
            match direction {
                "L" => {
                    head_x -= 1;
                }
                "R" => {
                    head_x += 1;
                }
                "U" => {
                    head_y += 1;
                }
                "D" => {
                    head_y -= 1;
                }
                _ => panic!(),
            }

            let (dx, dy) = move_tail(head_x, head_y, tail_x, tail_y);
            tail_x += dx;
            tail_y += dy;

            visited.insert((tail_x, tail_y), true);
        }
    }

    println!("{}", visited.len());

    visited.clear();
    let mut rope_x = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut rope_y = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for line in lines.iter() {
        let mut s = line.split(" ");
        let direction = s.next().unwrap();
        let distance = s.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..distance {
            // Parse
            match direction {
                "L" => {
                    rope_x[0] -= 1;
                }
                "R" => {
                    rope_x[0] += 1;
                }
                "U" => {
                    rope_y[0] += 1;
                }
                "D" => {
                    rope_y[0] -= 1;
                }
                _ => panic!(),
            }

            for i in 1..10 {
                let (dx, dy) = move_tail(rope_x[i - 1], rope_y[i - 1], rope_x[i], rope_y[i]);
                rope_x[i] += dx;
                rope_y[i] += dy;
            }

            visited.insert((rope_x[9], rope_y[9]), true);
        }
    }

    println!("{}", visited.len());
}
