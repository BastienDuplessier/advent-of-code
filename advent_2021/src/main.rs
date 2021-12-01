use std::fs;

fn main() {
    let filename = "inputs/1";

    // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // println!("ex1_1 is {}", ex1_1(&contents));
    println!("ex1_2 is {}", ex1_2(&contents));
}

fn ex1_1(contents: &str) -> i32 {
    let split = contents.trim().split("\n").map(|x| x.parse::<i32>().unwrap());
    let (_, res) = split.fold((0, 0),|(current_depth, depth_increased), depth| {
        if current_depth == 0 {
            // println!("{} (N/A - no previous measurement)", depth);
            (depth, 0)
        } else if depth > current_depth {
            // println!("{} (increased)", depth);
            (depth, depth_increased+1)
        } else {
            // println!("{} (decreased)", depth);
            (depth, depth_increased)
        }
    });

    res
}

fn ex1_2(contents: &str) -> i32 {
    let vec: Vec<i32> = contents.trim().split("\n").map(|x| x.parse::<i32>().unwrap()).collect();
    let (_, res) = vec.windows(3).fold((0, 0),|(current_depth, depth_increased), depths| {
        let combined_depth = depths.iter().sum();
        if current_depth == 0 {
            println!("{} (N/A - no previous measurement)", combined_depth);
            (combined_depth, 0)
        } else if combined_depth > current_depth {
            println!("{} (increased)", combined_depth);
            (combined_depth, depth_increased+1)
        } else {
            println!("{} (decreased)", combined_depth);
            (combined_depth, depth_increased)
        }
    });

    res
}
