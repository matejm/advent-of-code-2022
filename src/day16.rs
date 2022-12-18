use std::{cmp::max, collections::HashMap, fs};

// custom hash, makes everything 5x faster.
fn set_opened(opened: u64, i: usize) -> u64 {
    opened | (1 << i)
}

fn is_opened(opened: u64, i: usize) -> bool {
    (opened & (1 << i)) > 0
}

fn all_opened(opened: u64, n_valves: usize, flow_rate: &Vec<i32>) -> bool {
    let mut opened = opened;
    for i in 0..n_valves {
        if (opened & 1) == 0 && flow_rate[i] > 0 {
            return false;
        }
        opened >>= 1;
    }
    return true;
}

fn get_max_pressure_release(
    g: &Vec<Vec<usize>>,
    flow_rate: &Vec<i32>,
    opened: u64,
    time: i32,
    v: usize,
    memo: &mut HashMap<(i32, usize, u64), i32>,
    last: usize,
) -> i32 {
    if time == 0 {
        return 0;
    }

    let k = (time, v, opened);

    if memo.contains_key(&k) {
        return *memo.get(&k).unwrap();
    }

    let mut score = 0;

    // open valve
    if !is_opened(opened, v) && flow_rate[v] > 0 {
        let opened = set_opened(opened, v);
        score = get_max_pressure_release(g, flow_rate, opened, time - 1, v, memo, 1000)
            + (time - 1) * flow_rate[v];
    }

    // move to one of the neighbors
    for n in g[v].iter() {
        // don't blindlessly go back without doing anything.
        if last == *n {
            continue;
        }
        score = max(
            score,
            get_max_pressure_release(g, flow_rate, opened, time - 1, *n, memo, v),
        );
    }

    memo.insert(k, score);
    return score;
}

fn get_max_pressure_release2(
    g: &Vec<Vec<usize>>,
    flow_rate: &Vec<i32>,
    opened: u64,
    time: i32,
    v: usize,
    u: usize,
    memo: &mut HashMap<(i32, usize, usize, u64, bool), i32>,
    last_v: usize,
    last_u: usize,
    elephant_turn: bool,
    n_valves: usize,
) -> i32 {
    if time == 0 {
        return 0;
    }

    if all_opened(opened, n_valves, flow_rate) {
        return 0;
    }

    let k = (time, v, u, opened, elephant_turn);

    if memo.contains_key(&k) {
        return *memo.get(&k).unwrap();
    }

    let mut score = 0;

    if elephant_turn {
        // open valve
        if !is_opened(opened, u) && flow_rate[u] > 0 {
            let opened = set_opened(opened, u);
            score = get_max_pressure_release2(
                g,
                flow_rate,
                opened,
                time - 1,
                v,
                u,
                memo,
                last_v,
                1000,
                false,
                n_valves,
            ) + (time - 1) * flow_rate[u];
        }

        // move to one of the neighbors
        for n in g[u].iter() {
            // don't blindlessly go back without doing anything.
            if last_u == *n {
                continue;
            }
            score = max(
                score,
                get_max_pressure_release2(
                    g,
                    flow_rate,
                    opened,
                    time - 1,
                    v,
                    *n,
                    memo,
                    last_v,
                    u,
                    false,
                    n_valves,
                ),
            );
        }
    } else {
        // open valve
        if !is_opened(opened, v) && flow_rate[v] > 0 {
            let opened = set_opened(opened, v);
            score = get_max_pressure_release2(
                g, flow_rate, opened, time, v, u, memo, 1000, last_u, true, n_valves,
            ) + (time - 1) * flow_rate[v];
        }

        // move to one of the neighbors
        for n in g[v].iter() {
            // don't blindlessly go back without doing anything.
            if last_v == *n {
                continue;
            }
            score = max(
                score,
                get_max_pressure_release2(
                    g, flow_rate, opened, time, *n, u, memo, v, last_u, true, n_valves,
                ),
            );
        }
    }

    memo.insert(k, score);
    return score;
}

pub fn day16() {
    let lines = fs::read_to_string("inputs/16.in").expect("Could not read file");
    let lines = lines.lines();

    let mut convert = HashMap::new();

    for (i, line) in lines.clone().enumerate() {
        let l = line.replace("Valve ", "");
        let mut l = l.split(" ");
        let a = l.next().unwrap();

        convert.insert(a.to_string(), i);
    }

    let mut g: Vec<Vec<usize>> = vec![vec![]; 64];
    let mut flow_rate: Vec<i32> = vec![0; 64];

    for line in lines {
        let line = line.replace("Valve ", "").replace("has flow rate=", "");
        let line = line
            .replace("; tunnels lead to valves", "")
            .replace("; tunnel leads to valve", "");
        let line = line.replace(",", "");
        let line: Vec<String> = line.split(" ").map(|x| x.to_owned()).collect();

        let v = convert.get(&line[0].clone()).unwrap();
        let f = line[1].parse::<i32>().unwrap();
        let neighbors: Vec<usize> = line[2..].iter().map(|x| *convert.get(x).unwrap()).collect();

        g[*v] = neighbors;
        flow_rate[*v] = f;
    }

    let start = *convert.get(&"AA".to_string()).unwrap();

    let mut memo: HashMap<(i32, usize, u64), i32> = HashMap::new();
    let mut memo2: HashMap<(i32, usize, usize, u64, bool), i32> = HashMap::new();
    let n_valves = convert.len();

    println!(
        "{}",
        get_max_pressure_release(&g, &flow_rate, 0, 30, start, &mut memo, 1000)
    );

    println!(
        "{}",
        get_max_pressure_release2(
            &g, &flow_rate, 0, 26, start, start, &mut memo2, 1000, 1000, false, n_valves
        )
    );
}
