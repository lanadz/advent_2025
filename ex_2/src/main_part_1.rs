// ID ranges -, separated by ,
// ie:
// 11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
// 1698522-1698528,446443-446449,38593856-38593862,565653-565659,
// 824824821-824824827,2121212118-2121212124
// invalid IDs: some sequence of digits repeated twice.
// 55 (5 twice), 6464 (64 twice), and 123123 (123 twice)
// invalid ID has leading zeroes; 0101 isn't an ID at all
// 101 is valid
//
// 11-22 has two invalid IDs, 11 and 22.
// 95-115 has one invalid ID, 99.
// 998-1012 has one invalid ID, 1010.
// 1188511880-1188511890 has one invalid ID, 1188511885.
// 222220-222224 has one invalid ID, 222222.
// 1698522-1698528 contains no invalid IDs.
// 446443-446449 has one invalid ID, 446446.
// 38593856-38593862 has one invalid ID, 38593859.
// The rest of the ranges contain no invalid IDs.
// Adding up all the invalid IDs in this example produces 1227775554.
// What do you get if you add up all of the invalid IDs?

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let file = File::open(input_path("ranges_example"))?;
    let reader = BufReader::new(file);
    let mut invalid_ids: Vec<u64> = vec![];
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
        let ranges: Vec<&str> = line.split_terminator(",").collect();
        for range in ranges {
            let (start_str, end_str) = range
                .split_once('-')
                .expect("must have start and end separated by -");
            invalid_ids.extend(collect_invalid_ids_for_range(start_str, end_str));
        }
    }
    dbg!(&invalid_ids);
    println!("Result: {}", invalid_ids.iter().sum::<u64>());
    Ok(())
}

fn input_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("input")
        .join(filename)
}

fn collect_invalid_ids_for_range(start_str: &str, end_str: &str) -> Vec<u64> {
    let start: u64 = start_str.parse().expect("not integer");
    let end: u64 = end_str.parse().expect("not integer");
    let mut invalid_ids: Vec<u64> = vec![];
    let mut current_number = start;
    if check_range(start_str, end_str) {
        while current_number <= end {
            current_number = bump_to_even_digits(current_number);
            if has_a_pattern(current_number) {
                invalid_ids.push(current_number)
            } else {
                current_number += 1;
                continue;
            }
            if let Some(val) = next_posible_patterned_number(current_number) { current_number = val }
            if current_number > end {
                return invalid_ids;
            }
        }
    }
    invalid_ids
}

fn has_a_pattern(number: u64) -> bool {
    let num_as_str = number.to_string();
    let (left, right) = num_as_str.split_at(num_as_str.len() / 2);
    left == right
}

fn next_posible_patterned_number(current_number: u64) -> Option<u64> {
    let current_number = current_number.to_string();
    if current_number.len() % 2 == 1 {
        return None;
    };

    let (current_pattern_part, _) = current_number.split_at(current_number.len() / 2);
    let next_pattern_part: u64 = current_pattern_part.parse::<u64>().expect("not integer") + 1;
    let new_number: u64 = next_pattern_part
        .to_string()
        .repeat(2)
        .parse()
        .expect("not integer");
    Some(new_number)
}

fn bump_to_even_digits(n: u64) -> u64 {
    let digits = n.to_string().len();
    if digits.is_multiple_of(2) {
        n
    } else {
        10u64.pow(digits as u32)
    }
}

fn check_range(start: &str, end: &str) -> bool {
    start.len().is_multiple_of(2) || end.len().is_multiple_of(2) || start.len() != end.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_posible_patterned_number_test() {
        assert_eq!(Some(33), next_posible_patterned_number(22));
        assert_eq!(Some(1212), next_posible_patterned_number(1122));
        assert_eq!(None, next_posible_patterned_number(112));
    }

    #[test]
    fn bump_to_even_digits_test() {
        assert_eq!(10, bump_to_even_digits(9));
        assert_eq!(49, bump_to_even_digits(49));
        assert_eq!(1000, bump_to_even_digits(495));
        assert_eq!(4958, bump_to_even_digits(4958));
        assert_eq!(100000, bump_to_even_digits(49586));
    }

    #[test]
    fn check_range_works() {
        assert!(check_range("11", "22"));
        assert!(check_range("11", "222"));
        assert_eq!(false, check_range("111", "222"));
        assert_eq!(true, check_range("111", "2222"));
        assert_eq!(true, check_range("1111", "22222"));
    }

    #[test]
    fn has_a_pattern_test() {
        assert_eq!(true, has_a_pattern(11));
        assert_eq!(false, has_a_pattern(1122));
        assert_eq!(false, has_a_pattern(21122));
        assert_eq!(true, has_a_pattern(211211));
    }

    #[test]
    fn collect_invalid_ids_for_range_test() {
        assert_eq!(vec![11, 22], collect_invalid_ids_for_range("11", "22"));
        assert_eq!(vec![99], collect_invalid_ids_for_range("95", "115"));
        assert_eq!(vec![1010], collect_invalid_ids_for_range("998", "1012"));
        assert_eq!(
            vec![1188511885],
            collect_invalid_ids_for_range("1188511880", "1188511890")
        );
        assert_eq!(
            vec![222222],
            collect_invalid_ids_for_range("222220", "222224")
        );
        assert_eq!(
            Vec::<u64>::new(),
            collect_invalid_ids_for_range("1698522", "1698528")
        );
        assert_eq!(
            vec![446446],
            collect_invalid_ids_for_range("446443", "446449")
        );
        assert_eq!(
            vec![38593859],
            collect_invalid_ids_for_range("38593856", "38593862")
        );
        assert_eq!(vec![11, 22], collect_invalid_ids_for_range("1", "22"));
    }
}
