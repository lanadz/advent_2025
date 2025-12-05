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
}

fn main() -> std::io::Result<()> {
    let file = File::open("src/input/password_input_short")?;
    let reader = BufReader::new(file);
    let mut password = 0;
    let mut starting_point = 50;
    for line in reader.lines() {
        let line = line?;
        let (dir, dist) = line.split_at(1);
        let dist: u16 = dist.parse().unwrap(); // add error handling
        let operation: Option<Operation> = match dir {
            "R" => Some(Operation::R(dist)),
            "L" => Some(Operation::L(dist)),
            _ => None,
        };
        println!("{}", starting_point);
        starting_point = calculate(starting_point, operation.unwrap()); // add error handling
        if starting_point == 0 {
            password += 1
        }
    }

    println!("password: {}", password);
    Ok(())
}

fn calculate(starting_point: u8, operation: Operation) -> u8 {
    let result = match operation {
        Operation::L(dist) => {
            let mut res: i32 = starting_point as i32 - dist as i32;
            if res < 0 {
                res = res + 100
            }
            res
        }
        Operation::R(dist) => {
            let mut res: i32 = starting_point as i32 + dist as i32;
            if res > 99 {
                res = res - 100;
            }
            res
        }
    };
    result as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_works() {
        assert_eq!(calculate(50, Operation::L(5)), 45);
        assert_eq!(calculate(50, Operation::R(5)), 55);
        assert_eq!(calculate(99, Operation::R(5)), 4);
        assert_eq!(calculate(2, Operation::L(5)), 97);
        assert_eq!(calculate(2, Operation::L(2)), 0);
        assert_eq!(calculate(98, Operation::R(2)), 0);
        assert_eq!(calculate(98, Operation::R(443)), 2);
    }
}
