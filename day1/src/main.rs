use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_fuel_required(module: i32) -> i32 {
    let fuel = ((module / 3) as f64).floor() as i32 - 2;
    match fuel < 0 {
        true => 0,
        false => fuel,
    }
}

fn main() {
    let numbers: Vec<i32> = BufReader::new(File::open("input.txt").expect("file not found"))
        .lines() // Go through each line
        .map(|line| {
            line.expect("could not parse line") // Unwrap the result of the line
                .parse() // Try to parse it to what we expect (i32 from the annotation)
                .expect("could not parse number") // Unwrap the result of the parse
        })
        .collect();

    let mut total_fuel_required: i32 = 0;
    let mut fuel_added: i32;

    for module in numbers {
        fuel_added = calculate_fuel_required(module);
        loop {
            match fuel_added <= 0 {
                true => break,
                false => total_fuel_required += fuel_added,
            }
            fuel_added = calculate_fuel_required(fuel_added);
        }
    }

    println!("{}", total_fuel_required);
}
