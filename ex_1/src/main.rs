// - Follow rotary dial instructions using numbers 0–99 and a starting position of 50
// - Apply each rotation: R increases position, L decreases; wrap 99→0→99 with click counting
// - Track every post-rotation position and count how many times it lands on 0
// - Starting at 11, apply R8 to land on 19, then L19 to land on 0.
// - Starting at 5, apply L10 to land on 95, then R5 to land on 0.

use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
enum Operation {
    L(u16),
    R(u16),
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::L(dist) => write!(f, "L{}", dist),
            Operation::R(dist) => write!(f, "R{}", dist),
        }
    }
}

#[derive(Debug)]
struct ParseOperationError;

impl FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = s.split_at(1);
        let dist: u16 = dist.parse().map_err(|_| ParseOperationError)?;
        match dir {
            "R" => Ok(Operation::R(dist)),
            "L" => Ok(Operation::L(dist)),
            _ => Err(ParseOperationError),
        }
    }
}

fn main_1() -> std::io::Result<()> {
    let mut password = 0;
    let mut starting_point = 50;
    let reader = file_read::read_to_buffer_lines("password_input_final")?;
    for line in reader {
        let line = line?;
        let operation: Operation = line.parse().expect("invalid operation");

        let result = calculate_part1(starting_point, &operation);
        println!("{} {}: {}", starting_point, operation, result);
        starting_point = result;
        if starting_point == 0 {
            password += 1
        }
    }

    println!("password: {}", password);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut password = 0;
    let mut starting_point = 50;
    let reader = file_read::read_to_buffer_lines("password_input_final")?;
    for line in reader {
        let line = line?;
        let operation: Operation = line.parse().expect("invalid operation");

        let result = calculate_part2(starting_point, &operation);
        println!(
            "{} {}: new starting point {} with password {}",
            starting_point, operation, result.0, result.1
        );
        starting_point = result.0;
        password += result.1
    }

    println!("password: {}", password);
    Ok(())
}

fn calculate_part1(starting_point: u8, operation: &Operation) -> u8 {
    let result = match *operation {
        Operation::L(dist) => {
            let mut res: i32 = starting_point as i32 - dist as i32 % 100;
            if res < 0 {
                res += 100;
            }
            res
        }
        Operation::R(dist) => {
            let mut res: i32 = starting_point as i32 + dist as i32 % 100;
            if res > 99 {
                res -= 100;
            }
            res
        }
    };
    result as u8
}

fn calculate_part2(starting_point: u8, operation: &Operation) -> (u8, u32) {
    let distance: i32 = match *operation {
        Operation::L(dist) => -(i32::from(dist)),
        Operation::R(dist) => i32::from(dist),
    };
    let result = i32::from(starting_point) + distance;
    let mut zeros: u32 = result.unsigned_abs().div_euclid(100);

    if result <= 0 && starting_point != 0 {
        zeros += 1;
    }

    // let mut position = result % 100;
    // if position < 0 {
    //     position += 100
    // }
    // rem_euclid always returns positive reminder, no need for +=100 fix;
    // -7.rem_euclid(100) = 93, since euclidean remainder keeps results in [0,100), -7-100*(-1)=93

    // learnt something new:
    // -7.rem_eu(3)=-7-3*(-3)=2? because -7/3=-2.xx, floor -3?
    // Answer:
    // - Real division: -7 / 3 ≈ -2.333.
    // - Euclidean division uses the floor (toward −∞), so floor(-2.333) = -3.
    // - Plug that into dividend = divisor * quotient + remainder:
    //     -7 = 3 * (-3) + remainder → remainder = -7 - (-9) = 2.

    // Since 2 is in [0, 3), that’s the result.

    let position = result.rem_euclid(100);
    (position as u8, zeros)
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

    #[test]
    fn calculate_part2_works() {
        assert_eq!((50, 1), calculate_part2(50, &Operation::R(100)));
        assert_eq!((50, 1), calculate_part2(50, &Operation::L(100)));
        assert_eq!((0, 1), calculate_part2(1, &Operation::L(1)));
        assert_eq!((0, 3), calculate_part2(1, &Operation::L(201)));

        // The dial starts by pointing at 50.
        // The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
        // The dial is rotated L30 to point at 52.
        // The dial is rotated R48 to point at 0.
        // The dial is rotated L5 to point at 95.
        // The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
        // The dial is rotated L55 to point at 0.
        // The dial is rotated L1 to point at 99.
        // The dial is rotated L99 to point at 0.
        // The dial is rotated R14 to point at 14.
        // The dial is rotated L82 to point at 32; during this rotation, it points at 0 once.
        // In this example, the dial points at 0 three times
        // at the end of a rotation, plus three more times during a rotation.
        // So, in this example, the new password would be 6.
        assert_eq!((82, 1), calculate_part2(50, &Operation::L(68)));
        assert_eq!((52, 0), calculate_part2(82, &Operation::L(30)));
        assert_eq!((0, 1), calculate_part2(52, &Operation::R(48)));
        assert_eq!((95, 0), calculate_part2(0, &Operation::L(5)));
        assert_eq!((55, 1), calculate_part2(95, &Operation::R(60)));
        assert_eq!((0, 1), calculate_part2(55, &Operation::L(55)));
        assert_eq!((99, 0), calculate_part2(0, &Operation::L(1)));
        assert_eq!((0, 1), calculate_part2(99, &Operation::L(99)));
        assert_eq!((14, 0), calculate_part2(0, &Operation::R(14)));
        assert_eq!((32, 1), calculate_part2(14, &Operation::L(82)));
    }
}
