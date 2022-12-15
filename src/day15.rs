use std::{
    cmp::{max, min},
    collections::HashSet,
    fs,
    iter::zip,
};

fn dist((x, y): &(i32, i32), (a, b): &(i32, i32)) -> i32 {
    (x - a).abs() + (y - b).abs()
}

fn search_square(
    sensors: &Vec<(i32, i32)>,
    closest: &Vec<(i32, i32)>,
    (x, y): (i32, i32),
    (w, h): (i32, i32),
) {
    let h = max(h, 1);
    let w = max(w, 1);

    // Check if fully in
    for (sensor, beacon) in zip(sensors.iter(), closest.iter()) {
        let d = dist(sensor, beacon);
        if dist(&(x, y), sensor) <= d
            && dist(&(x + w - 1, y), sensor) <= d
            && dist(&(x, y + h - 1), sensor) <= d
            && dist(&(x + w - 1, y + h - 1), sensor) <= d
        {
            // fully contained
            return;
        }
    }

    // else check subsquares
    if w > 1 || h > 1 {
        search_square(sensors, closest, (x, y), (w / 2, h / 2));
        search_square(sensors, closest, (x + w / 2, y), (w - w / 2, h / 2));
        search_square(sensors, closest, (x, y + h / 2), (w / 2, h - h / 2));
        search_square(
            sensors,
            closest,
            (x + w / 2, y + h / 2),
            (w - w / 2, h - h / 2),
        );
        return;
    }
    println!("{}", 4000000 * (x as i64) + (y as i64),);
}

pub fn day15() {
    let lines = fs::read_to_string("inputs/15.in").expect("Could not read file");
    let lines = lines.lines();

    let mut sensors = vec![];
    let mut beacons = HashSet::new();
    let mut closest = vec![];

    let mut min_x = 0;
    let mut max_x = 0;

    for line in lines {
        let line = line.replace(",", "");
        let line = line.replace(":", "");
        let line: Vec<&str> = line.split([' ', '=']).collect();

        let sensor = (
            line[3].parse::<i32>().unwrap(),
            line[5].parse::<i32>().unwrap(),
        );
        sensors.push(sensor);

        let beacon = (
            line[11].parse::<i32>().unwrap(),
            line[13].parse::<i32>().unwrap(),
        );
        beacons.insert(beacon);
        closest.push(beacon);

        min_x = min(min_x, min(sensor.0, beacon.0));
        max_x = max(max_x, max(sensor.0, beacon.0));
    }

    // Check free y values
    let mut blocked = 0;
    let y = 2000000;

    for x in min_x - (max_x - min_x)..max_x + (max_x - min_x) {
        let mut possible = true;
        // check all sensors
        for (i, sensor) in sensors.iter().enumerate() {
            if dist(&(x, y), sensor) <= dist(sensor, &closest[i]) {
                possible = false;
                break;
            }
        }

        if !possible && !beacons.contains(&(x, y)) {
            blocked += 1;
        }
    }

    println!("{:?}", blocked);

    search_square(&sensors, &closest, (0, 0), (4000000, 4000000));
}
