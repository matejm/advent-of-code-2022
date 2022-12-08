use std::{fs, vec};

fn mark_visible(
    grid: &Vec<Vec<char>>,
    visible: &mut Vec<Vec<bool>>,
    start_x: i32,
    start_y: i32,
    direction_x: i32,
    direction_y: i32,
    steps_x: i32,
    steps_y: i32,
) {
    let mut height = -1;

    for dx in 0..steps_x {
        for dy in 0..steps_y {
            let y = (start_y + dy * direction_y) as usize;
            let x = (start_x + dx * direction_x) as usize;

            let h = grid[y][x];
            let h = h.to_string().parse::<i32>().unwrap();

            if h > height {
                height = h;
                // Mark visible
                visible[y][x] = true;
            }
        }
    }
}

fn count_row(
    grid: &Vec<Vec<char>>,
    start_x: i32,
    start_y: i32,
    height: i32,
    direction_x: i32,
    direction_y: i32,
    steps_x: i32,
    steps_y: i32,
) -> i32 {
    let mut count = 0;
    for dx in 1..steps_x {
        for dy in 1..steps_y {
            let y = (start_y + dy * direction_y) as usize;
            let x = (start_x + dx * direction_x) as usize;

            let h = grid[y][x];
            let h = h.to_string().parse::<i32>().unwrap();

            count += 1;

            if h >= height {
                return count;
            }
        }
    }
    return count;
}

fn tree_score(grid: &Vec<Vec<char>>, tree_x: i32, tree_y: i32) -> i32 {
    let size_y = grid.len() as i32;
    let size_x = grid.first().unwrap().len() as i32;

    let height = grid[tree_y as usize][tree_x as usize];
    let height = height.to_string().parse::<i32>().unwrap();

    let right = count_row(grid, tree_x, tree_y, height, 1, 0, size_x - tree_x, 2);
    let left = count_row(grid, tree_x, tree_y, height, -1, 0, tree_x + 1, 2);
    let down = count_row(grid, tree_x, tree_y, height, 0, 1, 2, size_y - tree_y);
    let up = count_row(grid, tree_x, tree_y, height, 0, -1, 2, tree_y + 1);

    right * left * down * up
}

pub fn day08() {
    let lines = fs::read_to_string("inputs/08.in").expect("Could not read file");
    let lines = lines.lines();

    let mut grid: Vec<Vec<char>> = vec![];
    let mut visible: Vec<Vec<bool>> = vec![];

    for line in lines {
        let line_chars: Vec<char> = line.chars().collect();
        let falses = (0..line_chars.len()).map(|_| false).collect();
        visible.push(falses);
        grid.push(line_chars)
    }

    let size_y = grid.len() as i32;
    let size_x = grid.first().unwrap().len() as i32;

    for y in 0..size_y {
        mark_visible(&grid, &mut visible, 0, y, 1, 0, size_x, 1);
        mark_visible(&grid, &mut visible, size_x - 1, y, -1, 0, size_x, 1);
    }

    for x in 0..size_x {
        mark_visible(&grid, &mut visible, x, 0, 0, 1, 1, size_y);
        mark_visible(&grid, &mut visible, x, size_y - 1, 0, -1, 1, size_y);
    }

    // Count
    let mut count = 0;
    for y in 0..size_y {
        for x in 0..size_x {
            if visible[y as usize][x as usize] {
                count += 1;
            }
        }
    }

    println!("{}", count);

    let mut max_score = 0;
    for y in 0..size_y {
        for x in 0..size_x {
            let score = tree_score(&grid, x, y);
            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("{}", max_score);
}
