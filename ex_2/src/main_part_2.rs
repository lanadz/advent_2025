// Now, an ID is invalid if it is made only of some sequence of digits
// repeated at least twice. So, 12341234 (1234 two times),
// 123123123 (123 three times), 1212121212 (12 five times), and 1111111 (1 seven times) are all invalid IDs.

// From the same example as before:

// 11-22 still has two invalid IDs, 11 and 22.
// 95-115 now has two invalid IDs, 99 and 111.
// 998-1012 now has two invalid IDs, 999 and 1010.
// 1188511880-1188511890 still has one invalid ID, 1188511885.
// 222220-222224 still has one invalid ID, 222222.
// 1698522-1698528 still contains no invalid IDs.
// 446443-446449 still has one invalid ID, 446446.
// 38593856-38593862 still has one invalid ID, 38593859.
// 565653-565659 now has one invalid ID, 565656.
// 824824821-824824827 now has one invalid ID, 824824824.
// 2121212118-2121212124 now has one invalid ID, 2121212121.
// Adding up all the invalid IDs in this example produces 4174379265.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let file = File::open(input_path("ranges_final"))?;
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
    for current_number in start..=end {
        if has_a_pattern(current_number) {
            invalid_ids.push(current_number)
        }
    }
    invalid_ids
}

fn has_a_pattern(number: u64) -> bool {
    let num_as_str = number.to_string();
    let mut chunk = num_as_str.len() / 2;
    while chunk > 0 {
        if find_pattern(&num_as_str, chunk) {
            println!("{} {}", chunk, number);
            return true;
        } else {
            chunk -= 1
        }
    }
    false
}

fn find_pattern(pattern: &str, chunk: usize) -> bool {
    if chunk == 0 {
        false
    } else {
        let chunks = (0..pattern.len()).step_by(chunk).map(|i| {
            let end = (i + chunk).min(pattern.len());
            &pattern[i..end]
        });
        // dbg!(chunks.clone().collect::<Vec<&str>>());
        all_equal(chunks)
    }
}

// lifetime 'a is needed because otherwise 'static will be used
fn all_equal<'a>(mut iter: impl Iterator<Item = &'a str>) -> bool {
    if let Some(first) = iter.next() {
        // `first` is captured and then passed to `all` method
        iter.all(|s| s == first)
    } else {
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_pattern_test() {
        assert_eq!(false, find_pattern("112", 1));
        assert_eq!(false, find_pattern("1123", 2));
        assert_eq!(false, find_pattern("112112", 2));
        assert_eq!(true, find_pattern("112112", 3));
        assert_eq!(true, find_pattern("111", 1));
        assert_eq!(false, find_pattern("1111112", 3));
    }
    #[test]
    fn has_a_pattern_test() {
        assert_eq!(true, has_a_pattern(11));
        assert_eq!(true, has_a_pattern(111));
        assert_eq!(false, has_a_pattern(1122));
        assert_eq!(false, has_a_pattern(21122));
        assert_eq!(true, has_a_pattern(211211));
        assert_eq!(true, has_a_pattern(212121));
    }

    #[test]
    fn collect_invalid_ids_test() {
        assert_eq!(vec![11, 22], collect_invalid_ids_for_range("11", "22"));
        assert_eq!(vec![99, 111], collect_invalid_ids_for_range("95", "115"));
        assert_eq!(
            vec![999, 1010],
            collect_invalid_ids_for_range("998", "1012")
        );
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
        assert_eq!(
            vec![565656],
            collect_invalid_ids_for_range("565653", "565659")
        );
        assert_eq!(
            vec![824824824],
            collect_invalid_ids_for_range("824824821", "824824827")
        );
        assert_eq!(
            vec![2121212121],
            collect_invalid_ids_for_range("2121212118", "2121212124")
        );
    }
}
