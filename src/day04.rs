use std::fs;
use std::ops::Range;

fn parse_range(range: &str) -> Range<i32> {
    let mut splitted = range.split("-");
    let start = splitted.next().unwrap();
    let end = splitted.next().unwrap();
    let (start, end) = (
        i32::from_str_radix(start, 10).unwrap(),
        i32::from_str_radix(end, 10).unwrap(),
    );
    start..end
}

fn contained(range_1: &Range<i32>, range_2: &Range<i32>) -> bool {
    range_1.start <= range_2.start && range_1.end >= range_2.end
}

fn overlap(range_1: &Range<i32>, range_2: &Range<i32>) -> bool {
    if range_1.start <= range_2.start {
        range_2.start <= range_1.end
    } else {
        range_1.start <= range_2.end
    }
}

pub fn day04() {
    let lines = fs::read_to_string("inputs/04.in").expect("Could not read file");
    let lines = lines.lines();

    let mut count = 0;
    let mut count_2 = 0;

    for line in lines {
        let mut data = line.split(",");

        let range_1 = parse_range(data.next().unwrap());
        let range_2 = parse_range(data.next().unwrap());

        if contained(&range_1, &range_2) || contained(&range_2, &range_1) {
            count += 1;
        }

        if overlap(&range_1, &range_2) {
            count_2 += 1;
        }
    }

    println!("{}", count);

    println!("{}", count_2);
}
