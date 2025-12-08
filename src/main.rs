// - Follow rotary dial instructions using numbers 0–99 and a starting position of 50
// - Apply each rotation: R increases position, L decreases; wrap 99→0→99 with click counting
// - Track every post-rotation position and count how many times it lands on 0
// - Starting at 11, apply R8 to land on 19, then L19 to land on 0.
// - Starting at 5, apply L10 to land on 95, then R5 to land on 0.

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Operation {
    L(u16),
    R(u16),
    Skip,
}

impl Operation {
    fn to_string(&self) -> String {
        match self {
            Operation::L(dist) => format!("L{}", dist),
            Operation::R(dist) => format!("R{}", dist),
            Operation::Skip => "invalid".to_string(),
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("src/input/password_input_final")?;
    let reader = BufReader::new(file);
    let mut password = 0;
    let mut starting_point = 50;
    for line in reader.lines() {
        let line = line?;
        let (dir, dist) = line.split_at(1);
        let dist: u16 = dist.parse().unwrap(); // add error handling
        let operation: Operation = match dir {
            "R" => Operation::R(dist),
            "L" => Operation::L(dist),
            _ => Operation::Skip,
        };

        let result = calculate_part1(starting_point, &operation); // add error handling
        println!("{} {}: {}", starting_point, operation.to_string(), result);
        starting_point = result;
        if starting_point == 0 {
            password += 1
        }
    }

    println!("password: {}", password);
    Ok(())
}

fn calculate_part1(starting_point: u8, operation: &Operation) -> u8 {
    let result = match *operation {
        Operation::L(dist) => {
            let mut res: i32 = starting_point as i32 - dist as i32 % 100;
            if res < 0 {
                res = res + 100
            }
            res
        }
        Operation::R(dist) => {
            let mut res: i32 = starting_point as i32 + dist as i32 % 100;
            if res > 99 {
                res = res - 100;
            }
            res
        }
        Operation::Skip => 0,
    };
    result as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_part1_works() {
        assert_eq!(calculate_part1(50, &Operation::L(5)), 45);
        assert_eq!(calculate_part1(50, &Operation::R(5)), 55);
        assert_eq!(calculate_part1(99, &Operation::R(5)), 4);
        assert_eq!(calculate_part1(2, &Operation::L(5)), 97);
        assert_eq!(calculate_part1(2, &Operation::L(2)), 0);
        assert_eq!(calculate_part1(98, &Operation::R(2)), 0);
        // 100 rotations are from 0-99, so it doesnt matter 45, or 145 or 445 -> it's as if you have only 45
        assert_eq!(calculate_part1(2, &Operation::R(443)), 45);
    }

    #[test]
    fn calculate_part1_from_example_tests() {
        // The dial starts by pointing at 50.
        // The dial is rotated L68 to point at 82.
        // The dial is rotated L30 to point at 52.
        // The dial is rotated R48 to point at 0.
        // The dial is rotated L5 to point at 95.
        // The dial is rotated R60 to point at 55.
        // The dial is rotated L55 to point at 0.
        // The dial is rotated L1 to point at 99.
        // The dial is rotated L99 to point at 0.
        // The dial is rotated R14 to point at 14.
        // The dial is rotated L82 to point at 32.
        assert_eq!(calculate_part1(50, &Operation::L(68)), 82);
        assert_eq!(calculate_part1(82, &Operation::L(30)), 52);
        assert_eq!(calculate_part1(52, &Operation::R(48)), 0);
        assert_eq!(calculate_part1(0, &Operation::L(5)), 95);
        assert_eq!(calculate_part1(95, &Operation::R(60)), 55);
        assert_eq!(calculate_part1(55, &Operation::L(55)), 0);
        assert_eq!(calculate_part1(0, &Operation::L(1)), 99);
        assert_eq!(calculate_part1(99, &Operation::L(99)), 0);
        assert_eq!(calculate_part1(0, &Operation::R(14)), 14);
        assert_eq!(calculate_part1(14, &Operation::L(82)), 32);
    }
}
