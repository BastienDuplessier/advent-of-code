#![feature(drain_filter)]
use std::fmt;
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
        ("day4", "1") => ex4_1(),
        ("day4", "2") => ex4_2(),
        // ("day4", "2") => ex4_2(),
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
        vec = vec
            .drain_filter(|line| line.starts_with(&start))
            .collect::<Vec<_>>();

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
        vec = vec
            .drain_filter(|line| line.starts_with(&start))
            .collect::<Vec<_>>();

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

#[derive(Debug)]
struct GridCell(i32, bool);
type GridLine = Vec<GridCell>;
type Grid = Vec<GridLine>;

fn ex4_1() -> Result<i32, String> {
    let numbers = ex4_get_numbers();
    // println!("{:?}", numbers);

    let mut grids = ex4_get_grids();

    for i in numbers {    
        // println!("\n\n");
        // println!("=================================");
        // println!("Current number : {}", i);
        let mut new_grids = Vec::new();
        for grid in grids {
            let new_grid = ex4_check_number(grid, i);
            if ex4_check_grid(&new_grid) {
                println!("youpi");
                ex4_println_grid(&new_grid);
                let sum = ex4_sum_unmarked_numbers(&new_grid);
                return Ok(sum * i);
            }
            // ex4_println_grid(&new_grid);
            // println!("---------------------------");
            new_grids.push(new_grid);
        }
        grids = new_grids;
    }

    Err("Failed".to_string())
}
fn ex4_2() -> Result<i32, String> {
    let numbers = ex4_get_numbers();
    // println!("{:?}", numbers);

    let mut grids = ex4_get_grids();
    let mut last_result: i32 = 0;

    for i in numbers {    
        // println!("\n\n");
        // println!("=================================");
        // println!("Current number : {}", i);
        let mut new_grids = Vec::new();
        for grid in grids {
            let new_grid = ex4_check_number(grid, i);
            if ex4_check_grid(&new_grid) {
                println!("youpi");
                ex4_println_grid(&new_grid);
                last_result = ex4_sum_unmarked_numbers(&new_grid) * i;
            } else {
                new_grids.push(new_grid);
            }
        }
        grids = new_grids;
    }

    Ok(last_result)
}

fn ex4_get_numbers() -> Vec<i32> {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input
            .trim()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect(),
        _ => panic!("ouch"),
    }
}

fn ex4_get_grids() -> Vec<Grid> {
    let mut grids = Vec::new();

    loop {
        match ex4_get_grid() {
            Some(grid) => grids.push(grid),
            _ => return grids,
        }
    }
}

// fn ex4_get_grid() -> Option<Vec<(bool, i32)>> {
fn ex4_get_grid() -> Option<Grid> {
    let mut input = String::new();

    // Here, find next non empty line
    while input == String::from("") {
        match std::io::stdin().read_line(&mut input) {
            Ok(0) => return None,
            Ok(_) => input = input.trim().to_string(),
            Err(_) => return None,
        }
    }

    // Build grid
    let mut grid: Grid = Vec::new();
    while input != String::from("") {
        grid.push(ex4_build_line(input.clone()));

        input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => input = input.trim().to_string(),
            Err(_) => return None,
        }
    }

    Some(grid)
}

fn ex4_build_line(input: String) -> GridLine {
    let mut line: GridLine = Vec::new();

    for s in input.split_whitespace() {
        let cell = GridCell(s.parse().unwrap(), false);
        line.push(cell)
    }

    line
}

fn ex4_println_grid(grid: &Grid) -> () {
    for line in grid {
        for cell in line {
            print!("{}{}", cell, '\t');
        }
        println!("");
    }

    ()
}

fn ex4_check_number(grid: Grid, number: i32) -> Grid {
    let mut new_grid = Vec::new();

    for line in grid {
        let mut new_line = Vec::new();
        for cell in line {
            if number == cell.0 {
                new_line.push(GridCell(cell.0, true))
            } else {
                new_line.push(GridCell(cell.0, cell.1))
            }
        }
        new_grid.push(new_line);
    }

    new_grid
}

impl fmt::Display for GridCell {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        if self.1 {
            write!(f, "_{}_", self.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

fn ex4_check_grid(grid: &Grid) -> bool {
    for i in 0..5 {
        let mut hcheck = true;
        let mut vcheck = true;
        for j in 0..5 {
            // println!("CELL ({},{}) = {}  [{} - {:?}]", j, i, grid[j][i], grid[j][i].0, grid[j][i].1);
            // println!("vcheck was {:?}, now is {:?}", vcheck, vcheck && grid[j][i].1);
            // println!("CELL ({},{}) = {}  [{} - {:?}]", i, j, grid[i][j], grid[i][j].0, grid[i][j].1);
            // println!("hcheck was {:?}, now is {:?}", hcheck, hcheck && grid[i][j].1);
       
            hcheck = hcheck && grid[i][j].1;
            vcheck = vcheck && grid[j][i].1;
        }
        if hcheck || vcheck {
            return true;
        }
    }

    false
}

fn ex4_sum_unmarked_numbers(grid: &Grid) -> i32 {
    let mut sum = 0;
    for line in grid {
        for cell in line {
            if !cell.1 {
                sum = sum + cell.0
            }
        }
    }

    sum
}
