use std::collections::VecDeque;
use std::fs;

fn eval(expr: &str, old: i64) -> i64 {
    let expr = expr.replace("old", old.to_string().as_str());
    let mut expr = expr.split(" ");
    let a = expr.next().unwrap();
    let op = expr.next().unwrap();
    let b = expr.next().unwrap();

    let a = a.parse::<i64>().unwrap();
    let b = b.parse::<i64>().unwrap();

    match op {
        "+" => a + b,
        "*" => a * b,
        _ => panic!(),
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn lcm_vec(v: &Vec<i64>) -> i64 {
    let mut l = 1;
    for &a in v {
        l = lcm(a, l);
    }
    l
}

pub fn day11() {
    let lines = fs::read_to_string("inputs/11.in").expect("Could not read file");
    let mut lines = lines.lines();

    let mut monkeys: Vec<VecDeque<i64>> = Vec::new();
    let mut operations = Vec::new();
    let mut tests = vec![];
    let mut if_true = vec![];
    let mut if_false = vec![];

    lines.next();

    loop {
        // Parse items
        let items = lines.next().unwrap().split(": ");
        let items = items.last().unwrap().split(", ");
        let items = items.map(|i| i.parse::<i64>().unwrap());
        let items: VecDeque<i64> = items.collect();
        monkeys.push(items);

        // Parse operation
        let operation = lines.next().unwrap().split(" = ");
        let operation = operation.last().unwrap();
        operations.push(operation);

        // Parse test
        let test = lines.next().unwrap().split("by ");
        let test = test.last().unwrap().parse::<i64>().unwrap();
        tests.push(test);

        // Parse branches
        let t = lines
            .next()
            .unwrap()
            .split("monkey ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        if_true.push(t);

        let f = lines
            .next()
            .unwrap()
            .split("monkey ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        if_false.push(f);

        // Read next two lines
        if lines.next().is_none() || lines.next().is_none() {
            break;
        }
    }

    let mut count = vec![0; monkeys.len()];
    let monkeys_lcm = lcm_vec(&tests);

    let monkeys_tmp = monkeys.clone();

    // Play game
    for _ in 0..20 {
        for monkey in 0..monkeys.len() {
            let n_items = monkeys[monkey].len();
            count[monkey] += n_items;

            for _ in 0..n_items {
                let mut item = monkeys[monkey].pop_front().unwrap();

                // Inspect item
                item = eval(operations[monkey], item);

                // Relief
                item /= 3;

                // Check
                if item % tests[monkey] == 0 {
                    monkeys[if_true[monkey]].push_back(item);
                } else {
                    monkeys[if_false[monkey]].push_back(item);
                }
            }
        }
    }

    count.sort();

    let best_monkey = count.pop().unwrap();
    let second_best_monkey = count.pop().unwrap();

    println!("{}", best_monkey * second_best_monkey);

    // Second star
    count = vec![0; monkeys.len()];
    monkeys = monkeys_tmp;

    // Play game
    for _ in 0..10000 {
        for monkey in 0..monkeys.len() {
            let n_items = monkeys[monkey].len();
            count[monkey] += n_items;

            for _ in 0..n_items {
                let mut item = monkeys[monkey].pop_front().unwrap();

                // Inspect item
                item = eval(operations[monkey], item);

                // Keep manageable
                item %= monkeys_lcm;

                // Check
                if item % tests[monkey] == 0 {
                    monkeys[if_true[monkey]].push_back(item);
                } else {
                    monkeys[if_false[monkey]].push_back(item);
                }
            }
        }
    }

    count.sort();

    let best_monkey = count.pop().unwrap();
    let second_best_monkey = count.pop().unwrap();

    println!("{}", best_monkey * second_best_monkey);
}
