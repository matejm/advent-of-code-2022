use std::fs;

fn check(c: i32, x: i32) -> i32 {
    if (20..221).step_by(40).find(|t| t == &c).is_some() {
        c * x
    } else {
        0
    }
}

fn draw(c: i32, x: i32, display: &mut Vec<char>) {
    if (((c - 1) % 40) - x).abs() <= 1 {
        display.push('#')
    } else {
        display.push('.')
    }
}

pub fn day10() {
    let lines = fs::read_to_string("inputs/10.in").expect("Could not read file");
    let lines = lines.lines();

    let mut x = 1;
    let mut c = 1;
    let mut score = 0;
    let mut display = vec![];

    for line in lines {
        draw(c, x, &mut display);

        if line.starts_with("noop") {
            c += 1;
        } else {
            let add = line.split(" ").last().unwrap().parse::<i32>().unwrap();
            c += 1;
            score += check(c, x);
            draw(c, x, &mut display);
            c += 1;
            x += add;
        }

        score += check(c, x);
    }

    println!("{}", score);

    for i in (0..220).step_by(40) {
        let s: String = display[i..(i + 40)].iter().collect();
        println!("{}", s);
    }
}
