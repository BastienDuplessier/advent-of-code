use std::io::prelude::*;
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

