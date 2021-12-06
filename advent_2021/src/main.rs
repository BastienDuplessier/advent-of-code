#![feature(drain_filter)]
use std::io::prelude::*;
use std::str::Chars;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The day to run
    day: String,
    /// Part of the exercice to run
    part: String,
}

fn main() {
    let args = Cli::from_args();

    // println!("Hello, World!");

    // let filename = "inputs/1";
    // let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let day = args.day.as_str();
    let part = args.part.as_str();

    let result = match (args.day.as_str(), args.part.as_str()) {
        ("day1", "1") => ex1_1(),
        ("day1", "2") => ex1_2(),
        ("day2", "1") => ex2_1(),
        ("day2", "2") => ex2_2(),
        ("day3", "1") => ex3_1(),
        ("day3", "2") => ex3_2(),
        _ => Err(format!("could not get `{} {}`", day, part)),
    };

    println!("{}", result.expect("no result"));
}

fn ex1_1() -> Result<i32, String> {
    let stdin = std::io::stdin();
    let vec: Vec<i32> = stdin
        .lock()
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();
    let (_, res) = vec
        .iter()
        .fold((0, 0), |(current_depth, depth_increased), &depth| {
            if current_depth == 0 {
                // println!("{} (N/A - no previous measurement)", depth);
                (depth, 0)
            } else if depth > current_depth {
                // println!("{} (increased)", depth);
                (depth, depth_increased + 1)
            } else {
                // println!("{} (decreased)", depth);
                (depth, depth_increased)
            }
        });

    Ok(res)
}

fn ex1_2() -> Result<i32, String> {
    let stdin = std::io::stdin();
    let vec: Vec<i32> = stdin
        .lock()
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();
    let (_, res) = vec
        .windows(3)
        .fold((0, 0), |(current_depth, depth_increased), depths| {
            let combined_depth = depths.iter().sum();
            if current_depth == 0 {
                // println!("{} (N/A - no previous measurement)", combined_depth);
                (combined_depth, 0)
            } else if combined_depth > current_depth {
                // println!("{} (increased)", combined_depth);
                (combined_depth, depth_increased + 1)
            } else {
                // println!("{} (decreased)", combined_depth);
                (combined_depth, depth_increased)
            }
        });

    Ok(res)
}

fn ex2_1() -> Result<i32, String> {
    // let mut h_dist = 0;
    // let mut v_dist = 0;

    let stdin = std::io::stdin();
    let (h, v) = stdin
        .lock()
        .lines()
        .map(|x| ex2_parse(x.unwrap()))
        .fold((0, 0), {
            |(h, v), (dir, val)| match dir {
                'f' => (h, v + val),
                'u' => (h - val, v),
                _ => (h + val, v),
            }
        });

    println!("h:{},v:{} = **{}", h, v, h * v);

    Ok(h * v)
}

fn ex2_2() -> Result<i32, String> {
    let stdin = std::io::stdin();
    let (h, v, _) = stdin
        .lock()
        .lines()
        .map(|x| ex2_parse(x.unwrap()))
        .fold((0, 0, 0), {
            |(h, v, a), (dir, val)| match dir {
                // down X increases your aim by X units.
                // up X decreases your aim by X units.
                // forward X does two things:
                //     It increases your horizontal position by X units.
                //     It increases your depth by your aim multiplied by X.
                'd' => (h, v, a + val),
                'u' => (h, v, a - val),
                _ => (h + val, v + (val * a), a),
            }
        });

    println!("h:{},v:{} = **{}", h, v, h * v);

    Ok(h * v)
}

fn ex2_parse(line: String) -> (char, i32) {
    let mut split = line.trim().split(" ");
    let command = split.next().unwrap();
    let value: i32 = split.next().unwrap().parse().unwrap();

    match command {
        "forward" => ('f', value),
        "up" => ('u', value),
        "down" => ('d', value),
        _ => ('e', value),
    }
}

fn ex3_1() -> Result<i32, String> {
    let stdin = std::io::stdin();
    let mut input = stdin.lock().lines().map(|x| x.unwrap());

    let size = input.nth(0).unwrap().len();
    let result = input.fold(Vec::new(), |vec, line| {
        ex3_update_vec(size, vec, line.chars())
    });

    let (mcb, lcb) = ex3_count_bits(result);

    Ok(ex3_vec_to_decimal(mcb) * ex3_vec_to_decimal(lcb))
}

fn ex3_2() -> Result<i32, String> {
    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map(|x| x.unwrap()).collect();

    let ogr = ex3_filter_most_values(input.clone());
    let cgr = ex3_filter_least_values(input);

    println!("{} -- {}", ogr, cgr);
    Ok(ogr * cgr)
}

fn ex3_find_most_least_bits_at(vec: &Vec<String>, index: usize) -> (i32, i32) {
    let (zero, one) = vec
        .iter()
        .fold((0, 0), |(zero, one), line| match line.chars().nth(index) {
            Some('0') => (zero + 1, one),
            Some('1') => (zero, one + 1),
            _ => (zero, one),
        });
    if zero > one {
        (0, 1)
    } else {
        (1, 0)
    }
}

fn ex3_update_vec(size: usize, vec: Vec<(i32, i32)>, mut chars: Chars<'_>) -> Vec<(i32, i32)> {
    let mut v: Vec<(i32, i32)> = Vec::new();

    for i in 0..size {
        let (zero, one) = match vec.get(i) {
            Some((zero, one)) => (*zero, *one),
            None => (0, 0),
        };

        match chars.next() {
            Some('0') => v.push((zero + 1, one)),
            Some('1') => v.push((zero, one + 1)),
            _ => (),
        }
    }

    v
}

fn ex3_count_bits(vec: Vec<(i32, i32)>) -> (Vec<i32>, Vec<i32>) {
    vec.iter().fold((Vec::new(), Vec::new()), {
        |(mut mcb, mut lcb), (zero, one)| {
            if zero > one {
                mcb.push(0);
                lcb.push(1);
                (mcb, lcb)
            } else {
                mcb.push(1);
                lcb.push(0);
                (mcb, lcb)
            }
        }
    })
}

fn ex3_vec_to_decimal(vec: Vec<i32>) -> i32 {
    vec.iter()
        .skip_while(|x| **x < 0)
        .fold(0, |acc, x| acc * 2 + x)
}

fn ex3_filter_most_values(mut vec: Vec<String>) -> i32 {
    let mut start = String::from("");
    let mut i = 0;

    while vec.len() > 1 {
        let (mcb, _lcb) = ex3_find_most_least_bits_at(&vec, i);
        start = format!("{}{}", &start, mcb);
        vec = vec.drain_filter(|line| line.starts_with(&start)).collect::<Vec<_>>();

        i = i + 1;
    }

    let res_vec = ex3_string_to_vec(&vec[0]);
    ex3_vec_to_decimal(res_vec)
}

fn ex3_filter_least_values(mut vec: Vec<String>) -> i32 {
    let mut start = String::from("");
    let mut i = 0;

    while vec.len() > 1 {
        let (_mcb, lcb) = ex3_find_most_least_bits_at(&vec, i);
        start = format!("{}{}", &start, lcb);
        vec = vec.drain_filter(|line| line.starts_with(&start)).collect::<Vec<_>>();

        i = i + 1;
    }

    let res_vec = ex3_string_to_vec(&vec[0]);
    ex3_vec_to_decimal(res_vec)
}

fn ex3_string_to_vec(list: &String) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    let size = list.len();
    let mut chars = list.chars();

    for _i in 0..size {
        match chars.next() {
            Some('0') => vec.push(0),
            Some('1') => vec.push(1),
            _ => (),
        }
    }

    vec
}

