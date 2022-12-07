use std::collections::HashMap;
use std::{fs, vec};

enum Cmd {
    Cd { dir: String },
    Ls { entry: Entry },
    Ignore,
}

enum Entry {
    Dir { name: String },
    File { size: u64, name: String },
}

fn parse_command(line: &str) -> Cmd {
    if line.starts_with("$") {
        let mut res = line.split(" ");
        let _ = res.next();
        let cmd = res.next().unwrap();

        if cmd == "cd" {
            Cmd::Cd {
                dir: res.next().unwrap().to_string(),
            }
        } else {
            Cmd::Ignore
        }
    } else {
        let mut res = line.split(" ");

        let first = res.next().unwrap();
        let name = res.next().unwrap().to_string();
        if first == "dir" {
            Cmd::Ls {
                entry: Entry::Dir { name: name },
            }
        } else {
            Cmd::Ls {
                entry: Entry::File {
                    size: first.parse::<u64>().unwrap(),
                    name: name,
                },
            }
        }
    }
}

fn join(v: &Vec<String>) -> String {
    let mut s = "".to_string();
    for p in v {
        s.push_str("/");
        s.push_str(&p);
    }
    s
}

pub fn day07() {
    let lines = fs::read_to_string("inputs/07.in").expect("Could not read file");
    let mut lines = lines.lines();

    let start = lines.next().unwrap();
    assert_eq!(start, "$ cd /");

    let mut sizes: HashMap<String, u64> = HashMap::new();
    let mut children_map = HashMap::new();
    let mut files = vec![];

    let mut path = vec!["_".to_string()];

    for line in lines {
        match parse_command(line) {
            Cmd::Cd { dir } => {
                if dir == "..".to_string() {
                    path.pop();
                } else {
                    path.push(dir);
                }
            }
            Cmd::Ls { entry } => match entry {
                Entry::Dir { name, .. } => {
                    let joined_path = join(&path);
                    let joined_total = join(&vec![joined_path.to_string(), name]);

                    if !children_map.contains_key(&joined_path) {
                        children_map.insert(joined_path, vec![joined_total]);
                    } else {
                        let mut in_there = children_map.remove(&joined_path).unwrap();
                        in_there.push(joined_total);
                        children_map.insert(joined_path, in_there);
                    }
                }
                Entry::File { size, name } => files.push((join(&path), size, name)),
            },
            Cmd::Ignore => {}
        }
    }

    for file in files {
        // add size to all parents
        let (path, size, _name) = file;

        let mut s = "".to_string();
        for part in path.split("/") {
            s.push_str(&part.to_string());

            *sizes.entry(s.clone()).or_insert(0) += size;

            s.push_str("/");
        }
    }

    // Sum of all directories with size at most 100000
    let mut star1 = 0;

    for size in sizes.values() {
        if size <= &100000 {
            star1 += size;
        }
    }

    println!("{}", star1);

    let free_space = 70000000 - sizes.get("/_").unwrap();
    let needed_space = 30000000 - free_space;
    assert!(needed_space > 0);

    let mut min_size = 70000000;
    for &size in sizes.values() {
        if min_size > size && size >= needed_space {
            min_size = size;
        }
    }

    println!("{}", min_size);
}
