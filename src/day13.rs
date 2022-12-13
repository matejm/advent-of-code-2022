use std::cmp::min;
use std::fs;

#[derive(Debug, Clone)]
enum Item {
    List(Vec<Item>),
    Number(i32),
}

impl Item {
    fn parse(line: &str, from: usize) -> Option<(Item, usize)> {
        let mut v = vec![];
        let mut i = from + 1;

        while i < line.len() - 1 {
            let c = line.get(i..i + 1).unwrap();

            if c == "[" {
                let (item, continue_from) = Item::parse(line, i).unwrap();
                v.push(item);
                i = continue_from;
            } else if c == "]" {
                break;
            } else if c == "," {
                // pass
            } else {
                // Parse number
                let mut j = i + 1;
                while line
                    .get(j..j + 1)
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap()
                    .is_numeric()
                {
                    j += 1;
                }
                let num = line.get(i..j).unwrap().parse::<i32>().unwrap();
                v.push(Item::Number(num));

                i = j - 1;
            }
            i += 1;
        }

        return Some((Item::List(v), i));
    }
}

fn validate(v1: &Item, v2: &Item) -> Option<bool> {
    // println!("{:?} {:?}", v1, v2);

    match (v1, v2) {
        (Item::Number(n1), Item::Number(n2)) => {
            if n1 < n2 {
                Some(true)
            } else if n1 > n2 {
                Some(false)
            } else {
                None
            }
        }
        (Item::Number(n1), Item::List(v2)) => {
            let l1 = Item::List(vec![Item::Number(*n1)]);
            return validate(&l1, &Item::List(v2.clone()));
        }
        (Item::List(v1), Item::Number(n2)) => validate(
            &Item::List(v1.clone()),
            &Item::List(vec![Item::Number(*n2)]),
        ),
        (Item::List(v1), Item::List(v2)) => {
            let mut res = None;

            for i in 0..min(v1.len(), v2.len()) {
                let v = validate(&v1[i], &v2[i]);
                if v.is_some() {
                    res = v;
                    break;
                }
            }
            if res.is_none() {
                if v1.len() < v2.len() {
                    res = Some(true);
                } else if v1.len() > v2.len() {
                    res = Some(false);
                }
            }
            res
        }
    }
}

pub fn day13() {
    let lines = fs::read_to_string("inputs/13.in").expect("Could not read file");
    let lines: Vec<&str> = lines.lines().collect();
    let mut l = lines.iter();

    let mut sum = 0;
    let mut index = 1;

    loop {
        let first = l.next();
        if first.is_none() {
            break;
        }

        let first = first.unwrap();
        let second = l.next().unwrap();
        l.next();

        let v1 = Item::parse(first, 0).unwrap();
        let v2 = Item::parse(second, 0).unwrap();

        let val = validate(&v1.0, &v2.0);
        if val.unwrap() {
            sum += index;
        }

        index += 1;
    }

    println!("{}", sum);

    let mut l = lines.iter();

    let mut ordered = vec![];

    let mut index = 1;

    let a = Item::parse("[[2]]", 0).unwrap().0;
    let b = Item::parse("[[6]]", 0).unwrap().0;

    ordered.push((a, -1));
    ordered.push((b, -2));

    loop {
        let first = l.next();
        if first.is_none() {
            break;
        }

        let first = *first.unwrap();
        if first == "" {
            continue;
        }

        let v1 = Item::parse(first, 0).unwrap().0;
        let v1_copy = v1.clone();

        // Insert
        let mut to_insert: i32 = -1;

        // Insertion sort
        for i in 0..ordered.len() {
            let v = validate(&ordered[i].0, &v1_copy);
            if v.is_none() || !v.unwrap() {
                to_insert = i as i32;
                break;
            }
        }
        if to_insert != -1 {
            ordered.insert(to_insert as usize, (v1, index));
        } else {
            ordered.push((v1, index));
        }

        index += 1;
    }

    let a = ordered.iter().position(|x| x.1 == -1).unwrap();
    let b = ordered.iter().position(|x| x.1 == -2).unwrap();

    println!("{}", (a + 1) * (b + 1));
}
