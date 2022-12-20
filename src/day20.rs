use std::fs;

const KEY: i64 = 811589153;

fn shuffle(v: &Vec<i64>) -> Vec<i64> {
    let mut v: Vec<(i64, bool)> = v.iter().map(|x| (x.clone(), false)).collect();

    let mut i: i64 = 0;
    while i < v.len() as i64 {
        while !v[i as usize].1 {
            let p = v.remove(i as usize);

            v.insert(
                (((i + p.0 - 1).rem_euclid(v.len() as i64)) + 1) as usize,
                (p.0, true),
            );
        }
        i += 1;
    }

    v.iter().map(|x| x.0).collect()
}

fn shuffle2(v: &Vec<i64>, n: usize) -> Vec<i64> {
    // i-th original is not in this position.
    let mut positions: Vec<usize> = (0..v.len()).collect();

    let mut v: Vec<(usize, &i64)> = v.iter().enumerate().collect();

    for _ in 0..n {
        // for all original positions
        for i in 0..v.len() {
            // remove from current position
            let prev_position = positions[i];
            let a = v.remove(prev_position);

            let new_position =
                ((((prev_position as i64) + a.1 - 1).rem_euclid(v.len() as i64)) + 1) as usize;
            positions[i] = new_position;

            // insert in new position
            v.insert(new_position, a);

            // update all position of moved elements
            if prev_position < new_position {
                // update all for one down
                for j in prev_position..new_position {
                    let a = v[j];
                    positions[a.0] -= 1;
                }
            } else if prev_position > new_position {
                // update all 1 up
                for j in new_position + 1..prev_position + 1 {
                    let a = v[j];
                    positions[a.0] += 1;
                }
            }
        }
    }

    v.iter().map(|x| *x.1).collect()
}

pub fn day20() {
    let lines = fs::read_to_string("inputs/20.in").expect("Could not read file");
    let lines = lines.lines();

    let numbers: Vec<i64> = lines.map(|x| x.parse::<i64>().unwrap()).collect();

    let shuffled = shuffle(&numbers);

    // find 0
    let i = shuffled.iter().position(|x| *x == 0).unwrap();
    let result = shuffled[(i + 1000) % shuffled.len()]
        + shuffled[(i + 2000) % shuffled.len()]
        + shuffled[(i + 3000) % shuffled.len()];

    println!("{}", result);

    // prepare numbers
    let mut numbers: Vec<i64> = numbers.iter().map(|x| (*x as i64) * KEY).collect();

    let numbers = shuffle2(&mut numbers, 10);

    // find 0
    let i = numbers.iter().position(|x| *x == 0).unwrap();
    let result = numbers[(i + 1000) % numbers.len()]
        + numbers[(i + 2000) % numbers.len()]
        + numbers[(i + 3000) % numbers.len()];

    // println!(
    //     "i {} // {} {} {}",
    //     i,
    //     numbers[(i + 1000) % numbers.len()],
    //     numbers[(i + 2000) % numbers.len()],
    //     numbers[(i + 3000) % numbers.len()]
    // );
    println!("{}", result);
}
