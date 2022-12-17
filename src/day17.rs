use std::{cmp::max, collections::HashSet, fs};

fn add(shape: &Vec<(i64, i64)>, d: (i64, i64)) -> Vec<(i64, i64)> {
    shape.iter().map(|p| (p.0 + d.0, p.1 + d.1)).collect()
}

fn collision(shape: &Vec<(i64, i64)>, board: &HashSet<(i64, i64)>) -> bool {
    for p in shape {
        if board.contains(&p) {
            return true;
        }
        if p.0 < 0 || p.0 > 6 {
            return true;
        }
    }
    return false;
}

fn fall_rock(
    board: &mut HashSet<(i64, i64)>,
    wind: Vec<char>,
    wind_i: i64,
    rock: &Vec<(i64, i64)>,
    height: i64,
) -> (i64, i64) {
    let mut rock = rock.clone();

    // Move in wind
    let c = wind.get((wind_i % (wind.len() as i64)) as usize).unwrap();
    let direction = match c {
        '>' => (1, 0),
        '<' => (-1, 0),
        _ => panic!(),
    };

    if !collision(&add(&rock, direction), board) {
        rock = add(&rock, direction);
    }

    // Fall
    let down = (0, -1);
    if !collision(&add(&rock, down), board) {
        rock = add(&rock, down);

        // Fall further
        return fall_rock(board, wind, wind_i + 1, &rock, height);
    } else {
        // landed
        let mut height = height;

        for p in rock.iter() {
            board.insert(p.clone());
            height = max(height, p.1);
        }

        return (wind_i + 1, height);
    }
}

pub fn day17() {
    let lines = fs::read_to_string("inputs/17.in").expect("Could not read file");
    let mut lines = lines.lines();

    let wind = lines.next().unwrap();
    let mut board = HashSet::new();

    // Add floor
    for i in 0..7 {
        board.insert((i, 0));
    }

    let shapes = vec![
        // ####
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        //  #
        // ###
        //  #
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        //   #
        //   #
        // ###
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        // #
        // #
        // #
        // #
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        // ##
        // ##
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];

    let mut wind_i: i64 = 0;
    let mut height = 0;

    for i in 0..2022 {
        let rock = i % shapes.len();

        let r = fall_rock(
            &mut board,
            wind.chars().collect(),
            wind_i,
            &add(&shapes[rock], (2, height + 4)),
            height,
        );
        wind_i = r.0;
        height = r.1;

        // print
        // println!("wind {}, height {}", wind_i, height);
        // for j in (0..height + 2).rev() {
        //     let line: Vec<&str> = (0..7)
        //         .map(|x| match board.contains(&(x, j)) {
        //             true => "#",
        //             false => ".",
        //         })
        //         .collect();
        //     println!("{}", line.join(""));
        // }

        // println!("=======================")
    }

    println!("{}", height);

    // part 2
    let mut board = HashSet::new();

    // Add floor
    for i in 0..7 {
        board.insert((i, 0));
    }

    let mut wind_i = 0;
    let mut height: i64 = 0;

    let target: i64 = 1000000000000;
    let mut i = 0;

    // find where this is reached
    let wind_target = 9;

    // Normal simulation (we have floor)
    loop {
        let rock = (i % (shapes.len() as i64)) as usize;

        let r = fall_rock(
            &mut board,
            wind.chars().collect(),
            wind_i,
            &add(&shapes[rock], (2, height + 4)),
            height,
        );
        wind_i = r.0;
        height = r.1;

        wind_i = wind_i % (wind.len() as i64);

        i += 1;

        // Loopable
        if wind_i == wind_target && rock == shapes.len() - 1 {
            break;
        }
    }

    // Generic situation (in the middle)
    let height_before = height;

    let mut j = 0;

    loop {
        let rock = (j % (shapes.len() as i64)) as usize;

        let r = fall_rock(
            &mut board,
            wind.chars().collect(),
            wind_i,
            &add(&shapes[rock], (2, height + 4)),
            height,
        );
        wind_i = r.0;
        height = r.1;

        wind_i = wind_i % (wind.len() as i64);

        j += 1;

        // Loopable
        if wind_i == wind_target && rock == shapes.len() - 1 {
            break;
        }
    }

    let height_after = height;

    // All others are the same
    let height_middle = (height - height_before) * ((target - i) / j);

    // Last part
    for k in 0..((target - i) % j) {
        let rock = (k % (shapes.len() as i64)) as usize;

        let r = fall_rock(
            &mut board,
            wind.chars().collect(),
            wind_i,
            &add(&shapes[rock], (2, height + 4)),
            height,
        );
        wind_i = r.0;
        height = r.1;
    }

    println!(
        "{}",
        height + height_middle - (height_after - height_before)
    );
}
