#![feature(drain_filter)]
use std::cmp;
use std::collections::HashMap;
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
        ("day5", "1") => ex5_1(),
        ("day5", "2") => ex5_2(),
        ("day6", "1") => ex6_1(),
        ("day6", "2") => ex6_2(),
        ("day7", "1") => ex7_1(),
        ("day7", "2") => ex7_2(),
        ("day8", "1") => ex8_1(),
        ("day8", "2") => ex8_2(),
        ("day9", "1") => ex9_1(),
        ("day9", "2") => ex9_2(),
        _ => Err(format!("could not get `{} {}`", day, part)),
    };

    println!("{}", result.expect("no result"));
}

fn ex1_1() -> Result<i64, String> {
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

    Ok(res as i64)
}

fn ex1_2() -> Result<i64, String> {
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

    Ok(res as i64)
}

fn ex2_1() -> Result<i64, String> {
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

    Ok((h * v) as i64)
}

fn ex2_2() -> Result<i64, String> {
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

    Ok((h * v) as i64)
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

fn ex3_1() -> Result<i64, String> {
    let stdin = std::io::stdin();
    let mut input = stdin.lock().lines().map(|x| x.unwrap());

    let size = input.nth(0).unwrap().len();
    let result = input.fold(Vec::new(), |vec, line| {
        ex3_update_vec(size, vec, line.chars())
    });

    let (mcb, lcb) = ex3_count_bits(result);

    Ok((ex3_vec_to_decimal(mcb) * ex3_vec_to_decimal(lcb)) as i64)
}

fn ex3_2() -> Result<i64, String> {
    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map(|x| x.unwrap()).collect();

    let ogr = ex3_filter_most_values(input.clone());
    let cgr = ex3_filter_least_values(input);

    println!("{} -- {}", ogr, cgr);
    Ok((ogr * cgr) as i64)
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

fn ex4_1() -> Result<i64, String> {
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
                return Ok((sum * i) as i64);
            }
            // ex4_println_grid(&new_grid);
            // println!("---------------------------");
            new_grids.push(new_grid);
        }
        grids = new_grids;
    }

    Err("Failed".to_string())
}
fn ex4_2() -> Result<i64, String> {
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

    Ok(last_result as i64)
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

type DiagramLine = Vec<i32>;
type Diagram = Vec<DiagramLine>;

fn ex5_1() -> Result<i64, String> {
    let diagram = ex5_build_diagram();
    // ex5_print_diagram(&diagram);

    Ok(ex5_count_overlap(&diagram) as i64)
}

fn ex5_2() -> Result<i64, String> {
    let diagram = ex5_build_diagram2();
    // ex5_print_diagram(&diagram);

    Ok(ex5_count_overlap(&diagram) as i64)
}

// fn ex5_print_diagram(diagram: &Diagram) -> () {
//     for line in diagram {
//         for i in line {
//             if i == &0 {
//                 print!(".");
//             } else {
//                 print!("{}", i);
//             }
//         }
//         println!("");
//     }
// }

fn ex5_build_diagram() -> Diagram {
    let mut diagram: Diagram = Vec::new();
    diagram.push(Vec::new());
    let mut input = String::new();

    loop {
        match std::io::stdin().read_line(&mut input) {
            Ok(0) => return diagram,
            Ok(_) => diagram = ex5_update_diagram(diagram, input.trim().to_string()),
            Err(_) => return diagram,
        };
        input = String::new();
    }
}

fn ex5_update_diagram(mut diagram: Diagram, input: String) -> Diagram {
    let (x1, y1, x2, y2) = ex5_parse_input(input);
    let mut new_diagram: Diagram = Vec::new();

    if x1 == x2 || y1 == y2 {
        // Resize diagram
        let max_y = cmp::max(cmp::max(y1, y2) + 1, diagram.len());
        let max_x = if diagram.len() > 0 {
            cmp::max(cmp::max(x1, x2) + 1, diagram[0].len())
        } else {
            cmp::max(x1, x2)
        };

        diagram.resize_with(max_y, || Vec::new());
        for mut line in diagram {
            line.resize(max_x, 0);
            new_diagram.push(line);
        }

        // println!("{},{} -> {},{}", x1,y1, x2, y2);
        if x1 == x2 {
            let (a, b) = (cmp::min(y1, y2), cmp::max(y1, y2));
            for i in a..b + 1 {
                // println!("{}:{} == {}", i, x1, new_diagram[i][x1]);
                new_diagram[i][x1] = new_diagram[i][x1] + 1;
            }
        } else {
            let (a, b) = (cmp::min(x1, x2), cmp::max(x1, x2));
            for j in a..b + 1 {
                // println!("{}:{} == {}", y2, j, new_diagram[y2][j]);
                new_diagram[y2][j] = new_diagram[y2][j] + 1;
            }
        }
    } else {
        return diagram;
    }
    // println!("{:?}", new_diagram);

    new_diagram
}

fn ex5_build_diagram2() -> Diagram {
    let mut diagram: Diagram = Vec::new();
    diagram.push(Vec::new());
    let mut input = String::new();

    loop {
        match std::io::stdin().read_line(&mut input) {
            Ok(0) => return diagram,
            Ok(_) => diagram = ex5_update_diagram2(diagram, input.trim().to_string()),
            Err(_) => return diagram,
        };
        input = String::new();
    }
}

fn ex5_update_diagram2(mut diagram: Diagram, input: String) -> Diagram {
    let (x1, y1, x2, y2) = ex5_parse_input(input);
    let mut new_diagram: Diagram = Vec::new();

    // println!("{},{} -> {},{}", x1,y1, x2, y2);
    let max_y = cmp::max(cmp::max(y1, y2) + 1, diagram.len());
    let max_x = if diagram.len() > 0 {
        cmp::max(cmp::max(x1, x2) + 1, diagram[0].len())
    } else {
        cmp::max(x1, x2)
    };
    diagram.resize_with(max_y, || Vec::new());
    for mut line in diagram {
        line.resize(max_x, 0);
        new_diagram.push(line);
    }

    if x1 == x2 || y1 == y2 {
        if x1 == x2 {
            let (a, b) = (cmp::min(y1, y2), cmp::max(y1, y2));
            for i in a..b + 1 {
                // println!("{}:{} == {}", i, x1, new_diagram[i][x1]);
                new_diagram[i][x1] = new_diagram[i][x1] + 1;
            }
        } else {
            let (a, b) = (cmp::min(x1, x2), cmp::max(x1, x2));
            for j in a..b + 1 {
                // println!("{}:{} == {}", y2, j, new_diagram[y2][j]);
                new_diagram[y2][j] = new_diagram[y2][j] + 1;
            }
        }
    } else {
        let range = ex5_build_range(x1, y1, x2, y2);

        for (x, y) in range {
            new_diagram[y][x] = new_diagram[y][x] + 1;
        }
    }
    // println!("{:?}", new_diagram);

    new_diagram
}

fn ex5_parse_input(input: String) -> (usize, usize, usize, usize) {
    let result: Vec<usize> = input
        .split(" -> ")
        .map(|xy| {
            xy.split(",")
                .map(|i| i.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
        .concat();

    (result[0], result[1], result[2], result[3])
}

fn ex5_count_overlap(diagram: &Diagram) -> i32 {
    diagram.iter().fold(0, |acc, line| {
        line.iter()
            .fold(acc, |acc, cell| if cell > &1 { acc + 1 } else { acc })
    })
}

fn ex5_build_range(x1: usize, y1: usize, x2: usize, y2: usize) -> Vec<(usize, usize)> {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let dir_x: i32 = if x1 > x2 { -1 } else { 1 };
    let dir_y: i32 = if y1 > y2 { -1 } else { 1 };

    let mut vec = vec![(x1, y1)];
    loop {
        i = i + dir_x;
        j = j + dir_y;
        let x = (x1 as i32 + i) as usize;
        let y = (y1 as i32 + j) as usize;

        vec.push((x, y));

        if (x as usize == x2) && (y == y2) {
            return vec;
        }
    }
}

fn ex6_1() -> Result<i64, String> {
    let mut vec = ex6_read_input();
    for _ in 0..80 {
        let mut new_vec = Vec::new();
        for fish in vec {
            if fish == 0 {
                new_vec.push(6);
                new_vec.push(8);
            } else {
                new_vec.push(fish - 1);
            }
        }
        vec = new_vec;
    }
    Ok(vec.len() as i64)
}

fn ex6_2() -> Result<i64, String> {
    let mut vec = ex6_convert_input(ex6_read_input());
    for _ in 0..256 {
        vec = ex6_update_vec(vec);
    }

    let mut sum = 0;
    for i in vec {
        sum = sum + i;
    }

    Ok(sum)
}

fn ex6_read_input() -> Vec<usize> {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input
            .trim()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn ex6_convert_input(vec: Vec<usize>) -> Vec<i64> {
    let mut new_vec = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];

    for i in vec {
        new_vec[i] = new_vec[i] + 1
    }

    new_vec
}

fn ex6_update_vec(vec: Vec<i64>) -> Vec<i64> {
    let mut new_vec = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];

    // Birth
    new_vec[6] = vec[0];
    new_vec[8] = vec[0];

    // Decrease timer
    for i in 1..9 {
        new_vec[i - 1] = new_vec[i - 1] + vec[i]
    }

    new_vec
}

fn ex7_1() -> Result<i64, String> {
    let vec = ex7_read_input();
    let max = &vec.iter().fold(0, |acc, x| cmp::max(acc, *x));

    let mut best_cost = 1000000;

    for i in 0..*max {
        let mut cost = 0;
        for pos in &vec {
            cost = cost + (pos - i).abs();
        }
        // println!("best cost {} , cost is {}", i ,cost);
        if best_cost > cost {
            best_cost = cost;
        }
    }

    Ok(best_cost as i64)
}

fn ex7_2() -> Result<i64, String> {
    let vec = ex7_read_input();
    let max = &vec.iter().fold(0, |acc, x| cmp::max(acc, *x));

    let mut best_cost: i64 = 10000000000000000;

    for i in 0..*max {
        let mut cost = 0;
        for pos in &vec {
            let diff = (pos - i).abs();
            cost = cost + (diff * (diff + 1) / 2);
        }
        // println!("best cost {} , cost is {}", i ,cost);
        if best_cost > cost {
            best_cost = cost;
        }
    }

    Ok(best_cost as i64)
}

fn ex7_read_input() -> Vec<i64> {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input
            .trim()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn ex8_1() -> Result<i64, String> {
    let input = ex8_parse_input1();

    let mut count = 0;
    for line in input {
        count = count
            + line.split(' ').fold(0, |acc, s| match s.len() {
                2 | 3 | 4 | 7 => acc + 1,
                _ => acc,
            })
    }

    Ok(count)
}

fn ex8_2() -> Result<i64, String> {
    let input = ex8_parse_input2();

    let mut total = 0;
    for (pattern, number) in input {
        // println!("To find: {:?}", number);
        let numbers = ex8_compute_pattern(pattern);
        // println!("Numbers: {:?}", numbers);
        let digits = ex8_to_digit(number, &numbers);
        // println!("Digits: {:?}", digits);

        total = total + digits.iter().fold(0, |acc, x| acc * 10 + x);
        // println!("Total: {:?}", digits);
        // count = count + line.split(' ').fold(0, |acc, s|
        //                      match s.len() {
        //                          2 | 3 | 4 | 7 => acc + 1,
        //                              _ => acc
        //                      }
        // )
    }

    Ok(total as i64)
}

fn ex8_parse_input1() -> Vec<String> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|x| String::from(x.unwrap().split('|').last().unwrap().trim()))
        .collect()
}

fn ex8_parse_input2() -> Vec<(String, String)> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|x| ex8_parse_line(x.unwrap()))
        .collect()
}

fn ex8_parse_line(x: String) -> (String, String) {
    let mut split = x.split('|');
    let pattern = split.nth(0).unwrap().trim();
    let number = split.last().unwrap().trim();
    (String::from(pattern), String::from(number))
}

fn ex8_compute_pattern(pattern: String) -> HashMap<String, i64> {
    let mut result = HashMap::new();
    let mut reverse = HashMap::new();
    let mut fivers = Vec::new();
    let mut sixers = Vec::new();
    let split = pattern.split(' ').map(|x| ex8_order_digit(x));

    // Get basic digits
    for digit in split {
        match digit.len() {
            2 => {
                result.insert(1, digit.clone());
                reverse.insert(digit, 1)
            }
            3 => {
                result.insert(7, digit.clone());
                reverse.insert(digit, 7)
            }
            4 => {
                result.insert(4, digit.clone());
                reverse.insert(digit, 4)
            }
            5 => {
                fivers.push(digit);
                None
            }
            6 => {
                sixers.push(digit);
                None
            }
            _ => {
                result.insert(8, digit.clone());
                reverse.insert(digit, 8)
            }
        };
    }

    // Find 0/6/9
    for digit in sixers {
        if ex8_matches(&digit, 1, &result) {
            if ex8_matches(&digit, 4, &result) {
                result.insert(9, digit.clone());
                reverse.insert(digit, 9);
            } else {
                result.insert(0, digit.clone());
                reverse.insert(digit, 0);
            }
        } else {
            result.insert(6, digit.clone());
            reverse.insert(digit, 6);
        }
    }

    for digit in fivers {
        if ex8_matches(&digit, 1, &result) {
            reverse.insert(digit, 3);
        } else {
            if ex8_reverse_matches(&digit, 6, &result) {
                reverse.insert(digit, 5);
            } else {
                reverse.insert(digit, 2);
            }
        }
    }

    reverse
}

fn ex8_order_digit(s: &str) -> String {
    let mut vec: Vec<char> = s.chars().collect();
    vec.sort();

    String::from_iter(vec)
}

fn ex8_matches(digit: &String, i: i64, numbers: &HashMap<i64, String>) -> bool {
    numbers[&i].chars().all(|c| ex8_char_match(digit, c))
}

fn ex8_reverse_matches(digit: &String, i: i64, numbers: &HashMap<i64, String>) -> bool {
    digit.chars().all(|c| ex8_char_match(&numbers[&i], c))
}

fn ex8_char_match(a: &String, b: char) -> bool {
    let v: Vec<&str> = a.matches(b).collect();
    !v.is_empty()
}

fn ex8_to_digit(str: String, numbers: &HashMap<String, i64>) -> Vec<i64> {
    str.split(' ')
        .map(|x| numbers[&ex8_order_digit(x)])
        .collect()
}

type MapLine = Vec<usize>;
type Map = Vec<MapLine>;

#[derive(Debug)]
struct RichMap {
    map: Map,
    hsize: usize,
    vsize: usize,
}
impl RichMap {
    fn get(&self, i: &usize, j: &usize) -> Option<usize> {
        if *i < self.hsize && *j < self.vsize {
            Some(self.map[*i][*j])
        } else {
            None
        }
    }
}

type Coords = (usize, usize);
type TraverseMap = HashMap<Coords, bool>;

fn ex9_1() -> Result<i64, String> {
    let map = ex9_parse_map();
    let (hsize, vsize) = ex9_calc_sizes(&map);

    let mut risk_level = 0;
    for i in 0..hsize {
        for j in 0..vsize {
            if ex9_find_if_lowest(&map, &i, &j, &hsize, &vsize) {
                risk_level = risk_level + 1 + map[i][j] as i64;
            }
        }
    }

    Ok(risk_level)
}
fn ex9_2() -> Result<i64, String> {
    let map = ex9_build_rich_map(ex9_parse_map());
    let (hsize, vsize) = (map.hsize, map.vsize);
    let mut lowest = Vec::new();

    for i in 0..hsize {
        for j in 0..vsize {
            if ex9_find_if_lowest(&map.map, &i, &j, &hsize, &vsize) {
                let size = ex9_compute_size(&map, &i, &j);
                lowest.push((i, j, size));
            }
        }
    }

    lowest.sort_by(|(_, _, a), (_, _, b)| b.cmp(a));

    Ok(lowest[0].2 * lowest[1].2 * lowest[2].2)
}

fn ex9_parse_map() -> Map {
    std::io::stdin()
        .lock()
        .lines()
        .map(|x| {
            x.unwrap()
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn ex9_calc_sizes(map: &Map) -> (usize, usize) {
    (map.len(), map[0].len())
}

fn ex9_find_if_lowest(map: &Map, i: &usize, j: &usize, maxi: &usize, maxj: &usize) -> bool {
    let result = true;
    // Up
    if *i >= 1 {
        if map[*i][*j] >= map[*i - 1][*j] {
            return false;
        }
    }
    // Left
    if *j >= 1 {
        if map[*i][*j] >= map[*i][*j - 1] {
            return false;
        }
    }
    // Down
    if *i + 1 < *maxi {
        if map[*i][*j] >= map[*i + 1][*j] {
            return false;
        }
    }
    // Right
    if *j + 1 < *maxj {
        if map[*i][*j] >= map[*i][*j + 1] {
            return false;
        }
    }

    result
}

fn ex9_compute_size(map: &RichMap, i: &usize, j: &usize) -> i64 {
    let mut traversed: TraverseMap = HashMap::new();
    traversed = ex9_traverse_map(map, *i, *j, traversed);
    traversed
        .values()
        .fold(0, |a, x| if *x { a + 1 } else { a })
}

fn ex9_traverse_map(map: &RichMap, i: usize, j: usize, mut traversed: TraverseMap) -> TraverseMap {
    if map.get(&i, &j) == None {
        return traversed;
    }
    match traversed.get(&(i, j)) {
        None => {
            match map.get(&i, &j) {
                Some(9) => {
                    traversed.insert((i, j), false);
                    return traversed;
                }
                Some(_) => traversed.insert((i, j), true),
                _ => {
                    traversed.insert((i, j), false);
                    return traversed;
                }
            };
            if i > 0 {
                traversed = ex9_traverse_map(map, i - 1, j, traversed);
            }
            if j > 0 {
                traversed = ex9_traverse_map(map, i, j - 1, traversed);
            }
            traversed = ex9_traverse_map(map, i + 1, j, traversed);
            ex9_traverse_map(map, i, j + 1, traversed)
        }
        _ => traversed,
    }
}

fn ex9_build_rich_map(map: Map) -> RichMap {
    let (hsize, vsize) = ex9_calc_sizes(&map);
    RichMap {
        map: map,
        hsize: hsize,
        vsize: vsize,
    }
}
