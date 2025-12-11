// 987654321111111
// 811111111111119
// 234234234234278
// 818181911112111
// Now, the joltages are much larger:

// In 987654321111111, the largest joltage can be found by turning on everything except some 1s at the end to produce 987654321111.
// In the digit sequence 811111111111119, the largest joltage can be found by turning on everything except some 1s, producing 811111111119.
// In 234234234234278, the largest joltage can be found by turning on everything except a 2 battery, a 3 battery, and another 2 battery near the start to produce 434234234278.
// In 818181911112111, the joltage 888911112111 is produced by turning on everything except some 1s near the front.

fn main() -> std::io::Result<()> {
    let reader = file_read::read_to_buffer_lines("final")?;
    let mut sum = 0;
    for line in reader {
        let line = line?;
        sum += find_max_n_digit_value(&line, 12);
    }

    println!("sum: {}", sum);
    Ok(())
}

fn find_max_n_digit_value(chain: &str, n: usize) -> u64 {
    let numbers: Vec<u32> = chain.chars().filter_map(|c| c.to_digit(10)).collect();

    if numbers.len() <= n {
        // return numbers.iter().fold(0u64, |acc, d| acc * 10 + (*d as u64)); // as a math reduce
        return chain.parse().expect("not a number");
    }

    let mut result = 0u64; // idiomatic syntax
    let mut start = 0usize;
    let mut remaining = n;

    while remaining > 0 {
        let end = numbers.len() - remaining;
        let mut max_digit = 0u32;
        let mut max_index = start;

        // enumarate return (index, element)
        for (offset, digit) in numbers[start..=end].iter().enumerate() {
            if *digit > max_digit {
                max_digit = *digit;
                max_index = start + offset;
            }
        }

        result = result * 10 + (max_digit as u64);
        start = max_index + 1;
        remaining -= 1;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_max_n_digit_value_test() {
        assert_eq!(987654321111, find_max_n_digit_value("987654321111111", 12));
        assert_eq!(811111111119, find_max_n_digit_value("811111111111119", 12));
        assert_eq!(434234234278, find_max_n_digit_value("234234234234278", 12));
        assert_eq!(888911112111, find_max_n_digit_value("818181911112111", 12));
        assert_eq!(81, find_max_n_digit_value("81", 12));
    }
}
