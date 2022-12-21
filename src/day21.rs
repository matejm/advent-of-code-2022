use num::{self, Complex};
use std::{collections::HashMap, fs};

enum Monkey {
    Expr((String, char, String)),
    Number(i64),
}

fn get_value(
    monkey: &String,
    store: &HashMap<String, Monkey>,
    memo: &mut HashMap<String, i64>,
) -> i64 {
    if memo.contains_key(monkey) {
        return memo[monkey];
    }

    let v = match &store[monkey] {
        Monkey::Number(n) => *n,
        Monkey::Expr((l, o, r)) => match o {
            '*' => get_value(l, store, memo) * get_value(r, store, memo),
            '+' => get_value(l, store, memo) + get_value(r, store, memo),
            '/' => get_value(l, store, memo) / get_value(r, store, memo),
            '-' => get_value(l, store, memo) - get_value(r, store, memo),
            _ => panic!("Invalid operator"),
        },
    };

    memo.insert(monkey.clone(), v);
    return v;
}

fn get_value2(
    monkey: &String,
    store: &HashMap<String, Monkey>,
    memo: &mut HashMap<String, Complex<f64>>,
) -> Complex<f64> {
    if memo.contains_key(monkey) {
        return memo[monkey].clone();
    }

    let v = match &store[monkey] {
        Monkey::Number(n) => {
            if monkey == "humn" {
                Complex::i() as Complex<f64>
            } else {
                Complex::from(*n as f64)
            }
        }
        Monkey::Expr((l, o, r)) => {
            let a = get_value2(l, store, memo);
            let b = get_value2(r, store, memo);

            if monkey == "root" {
                Complex::from((b.re - a.re) / (a.im - b.im))
            } else {
                match o {
                    '*' => a * b,
                    '+' => a + b,
                    '/' => a / b,
                    '-' => a - b,
                    _ => panic!("Invalid operator"),
                }
            }
        }
    };

    memo.insert(monkey.clone(), v.clone());
    return v;
}

pub fn day21() {
    let lines = fs::read_to_string("inputs/21.in").expect("Could not read file");
    let lines = lines.lines();

    let mut store = HashMap::new();

    for line in lines {
        let mut l = line.split(": ");
        let monkey = l.next().unwrap().to_string();

        let expression = l.next().unwrap();

        if expression.contains(" ") {
            let mut expression = expression.split(" ");
            let expression = (
                expression.next().unwrap().to_string(),
                expression.next().unwrap().chars().next().unwrap(),
                expression.next().unwrap().to_string(),
            );
            store.insert(monkey, Monkey::Expr(expression));
        } else {
            store.insert(monkey, Monkey::Number(expression.parse::<i64>().unwrap()));
        }
    }

    let v = get_value(&"root".to_string(), &store, &mut HashMap::new());

    println!("{}", v);

    let v2 = get_value2(&"root".to_string(), &store, &mut HashMap::new());

    println!("{}", v2.re.round());
}
