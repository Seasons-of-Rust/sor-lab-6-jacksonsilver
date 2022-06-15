use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut numbers: Vec<i32> = BufReader::new(File::open("input.txt").expect("file not found"))
        .lines() // Go through each line
        .next() // Only take the first line
        .unwrap() // Unwrap the option of whether there was a next line
        .unwrap() // Unwrap the string result of the lines
        .split(',') // Split by commas
        .map(|number| {
            number
                .parse() // Parse the number
                .expect("could not parse number") // Panic with a message if it fails
        })
        .collect();

    for noun in 0..=99 {
        numbers[1] = noun;
        for verb in 0..=99 {
            numbers[2] = verb;
            let mut numbers2 = numbers.clone();

            for i in (0..numbers2.len()).step_by(4) {
                match numbers2[i as usize] {
                    1 => {
                        let index = numbers2[(i + 3) as usize];
                        numbers2[index as usize] = numbers2[numbers2[(i + 1) as usize] as usize]
                            + numbers2[numbers2[(i + 2) as usize] as usize];
                        numbers2[i as usize] += 4;
                    }
                    2 => {
                        let index = numbers2[(i + 3) as usize];
                        numbers2[index as usize] = numbers2[numbers2[(i + 1) as usize] as usize]
                            * numbers2[numbers2[(i + 2) as usize] as usize];
                        numbers2[i as usize] += 4;
                    }
                    99 => {
                        break;
                    }
                    _ => {
                        panic!("Something went wrong");
                    }
                }
            }
            if numbers2[0] == 19690720 {
                println!("verb: {} and noun: {}", verb, noun)
            }
        }
    }
}
