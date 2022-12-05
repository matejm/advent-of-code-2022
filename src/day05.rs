use std::fs;

pub fn day05() {
    let lines = fs::read_to_string("inputs/05.in").expect("Could not read file");
    let mut lines = lines.lines();

    let mut crates = vec![];

    let mut parsed = false;

    while !parsed {
        let line = lines.next().unwrap();

        let mut cs = vec![];

        let mut line = line.chars();
        line.next();

        for i in line.step_by(4) {
            if i.is_numeric() {
                parsed = true;
                break;
            }
            cs.push(i);
        }

        crates.push(cs);
    }

    // Transpose crates vector.
    let mut cs = vec![];

    for (_i, ci) in crates.iter().enumerate() {
        for (j, c) in ci.iter().enumerate() {
            if cs.len() <= j {
                cs.push(vec![]);
            }

            if !c.is_whitespace() {
                cs[j].push(*c)
            }
        }
    }

    // reverse each line
    for v in cs.iter_mut() {
        v.reverse();
    }

    lines.next();

    let mut cs2 = cs.to_vec();

    for line in lines {
        let line = line.replace("move ", "");
        let line = line.replace(" from ", " ");
        let line = line.replace(" to ", " ");
        let mut res = line.split(" ");

        let n = usize::from_str_radix(res.next().unwrap(), 10).unwrap();
        let from = usize::from_str_radix(res.next().unwrap(), 10).unwrap();
        let to = usize::from_str_radix(res.next().unwrap(), 10).unwrap();

        for _ in 0..n {
            // Move
            let a = cs[from - 1].pop().unwrap();
            cs[to - 1].push(a);
        }

        let mut a = vec![];
        for _ in 0..n {
            a.push(cs2[from - 1].pop().unwrap());
        }
        for _ in 0..n {
            cs2[to - 1].push(a.pop().unwrap());
        }
    }

    for v in cs.iter() {
        print!("{}", v.last().unwrap());
    }

    println!();

    for v in cs2.iter() {
        print!("{}", v.last().unwrap());
    }

    println!()
}
