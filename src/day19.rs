use std::cmp::max;
use std::collections::HashMap;
use std::{fs, vec};

type Prices = (i32, i32, i32);

fn solve(
    // values
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
    // production
    ore_p: i32,
    clay_p: i32,
    obsidian_p: i32,
    geode_p: i32,
    // prices
    ore_price: &Prices,
    clay_price: &Prices,
    obsidian_price: &Prices,
    geode_price: &Prices,
    time: i32,
    memo: &mut HashMap<(i32, i32, i32, i32, i32, i32, i32, i32, i32), i32>,
) -> i32 {
    if time == 0 {
        return geode;
    }

    let k = (
        ore, clay, obsidian, geode, ore_p, clay_p, obsidian_p, geode_p, time,
    );
    if memo.contains_key(&k) {
        return memo[&k];
    }

    // produce
    let ore1 = ore + ore_p;
    let clay1 = clay + clay_p;
    let obsidian1 = obsidian + obsidian_p;
    let geode1 = geode + geode_p;

    let time = time - 1;

    let mut score = 0;

    // try to buy ore
    if ore >= ore_price.0 {
        // check if we need more ore
        if ore_p
            < max(
                ore_price.0,
                max(clay_price.0, max(obsidian_price.0, geode_price.0)),
            )
        {
            score = max(
                score,
                solve(
                    ore1 - ore_price.0,
                    clay1,
                    obsidian1,
                    geode1,
                    ore_p + 1,
                    clay_p,
                    obsidian_p,
                    geode_p,
                    ore_price,
                    clay_price,
                    obsidian_price,
                    geode_price,
                    time,
                    memo,
                ),
            );
        }
    }

    // try to buy clay
    if ore >= clay_price.0 {
        // check if we need more clay production
        if clay_p < obsidian_price.1 {
            score = max(
                score,
                solve(
                    ore1 - clay_price.0,
                    clay1,
                    obsidian1,
                    geode1,
                    ore_p,
                    clay_p + 1,
                    obsidian_p,
                    geode_p,
                    ore_price,
                    clay_price,
                    obsidian_price,
                    geode_price,
                    time,
                    memo,
                ),
            );
        }
    }

    // try to buy obsidian
    if ore >= obsidian_price.0 && clay >= obsidian_price.1 {
        // check if we need more obsidian production
        if obsidian_p < geode_price.2 {
            score = max(
                score,
                solve(
                    ore1 - obsidian_price.0,
                    clay1 - obsidian_price.1,
                    obsidian1,
                    geode1,
                    ore_p,
                    clay_p,
                    obsidian_p + 1,
                    geode_p,
                    ore_price,
                    clay_price,
                    obsidian_price,
                    geode_price,
                    time,
                    memo,
                ),
            );
        }
    }

    // try to buy geode
    if ore >= geode_price.0 && obsidian >= geode_price.2 {
        score = max(
            score,
            solve(
                ore1 - geode_price.0,
                clay1,
                obsidian1 - geode_price.2,
                geode1,
                ore_p,
                clay_p,
                obsidian_p,
                geode_p + 1,
                ore_price,
                clay_price,
                obsidian_price,
                geode_price,
                time,
                memo,
            ),
        );
    }

    // do not buy anything
    score = max(
        score,
        solve(
            ore1,
            clay1,
            obsidian1,
            geode1,
            ore_p,
            clay_p,
            obsidian_p,
            geode_p,
            ore_price,
            clay_price,
            obsidian_price,
            geode_price,
            time,
            memo,
        ),
    );

    memo.insert(k, score);

    return score;
}

pub fn day19() {
    let lines = fs::read_to_string("inputs/19.in").expect("Could not read file");
    let lines = lines.lines();

    let mut ore_pricing = vec![];
    let mut clay_pricing = vec![];
    let mut obsidian_pricing = vec![];
    let mut geode_pricing = vec![];

    for line in lines {
        let mut line = line.split("costs ");
        line.next();

        ore_pricing.push((
            line.next()
                .unwrap()
                .split(" ")
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap(),
            0,
            0,
        ));

        clay_pricing.push((
            line.next()
                .unwrap()
                .split(" ")
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap(),
            0,
            0,
        ));

        let s: Vec<&str> = line.next().unwrap().split(" ").collect();
        let ore = s[0].parse::<i32>().unwrap();
        let clay = s[3].parse::<i32>().unwrap();

        obsidian_pricing.push((ore, clay, 0));

        let s: Vec<&str> = line.next().unwrap().split(" ").collect();
        let ore = s[0].parse::<i32>().unwrap();
        let obsidian = s[3].parse::<i32>().unwrap();

        geode_pricing.push((ore, 0, obsidian));
    }

    let mut score = 0;
    for i in 0..ore_pricing.len() {
        let mut memo: HashMap<(i32, i32, i32, i32, i32, i32, i32, i32, i32), i32> = HashMap::new();

        score += solve(
            0,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            &ore_pricing[i],
            &clay_pricing[i],
            &obsidian_pricing[i],
            &geode_pricing[i],
            24,
            &mut memo,
        ) * ((i + 1) as i32);
    }

    println!("{}", score);

    let mut score = 1;

    for i in 0..3 {
        let mut memo: HashMap<(i32, i32, i32, i32, i32, i32, i32, i32, i32), i32> = HashMap::new();

        score *= solve(
            0,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            &ore_pricing[i],
            &clay_pricing[i],
            &obsidian_pricing[i],
            &geode_pricing[i],
            32,
            &mut memo,
        );
    }

    println!("{}", score)
}
