// For example, if you have a bank like 12345 and you turn on batteries 2 and 4, the bank would produce 24 jolts. (You cannot rearrange batteries.)
// You'll need to find the largest possible joltage each bank can produce. In the above example:

// In 987654321111111, you can make the largest joltage possible, 98, by turning on the first two batteries.
// In 811111111111119, you can make the largest joltage possible by turning on the batteries labeled 8 and 9, producing 89 jolts.
// In 234234234234278, you can make 78 by turning on the last two batteries (marked 7 and 8).
// In 818181911112111, the largest joltage you can produce is 92.
fn main() -> std::io::Result<()> {
    let reader = file_read::read_to_buffer_lines("final")?;
    let mut sum = 0;
    for line in reader {
        let line = line?;
        sum += find_max_2d_value(&line);
    }

    println!("sum: {}", sum);
    Ok(())
}

fn find_max_2d_value(chain: &str) -> u32 {
    if chain.len() <= 2 {
        return chain.parse().expect("not a number");
    }

    let mut result: u32;
    let numbers: Vec<u32> = chain.chars().filter_map(|c| c.to_digit(10)).collect();
    let mut max_num = numbers.iter().max().copied().unwrap_or(0);
    let mut index = numbers.iter().position(|&el| el == max_num).unwrap_or(0);

    // find first number
    result = if index < (numbers.len() - 1) {
        max_num * 10
    } else {
        let mut numbers_clone = numbers.clone();
        numbers_clone.remove(index);
        max_num = numbers_clone.iter().max().copied().unwrap_or(0);
        index = numbers.iter().position(|&el| el == max_num).unwrap_or(0);
        max_num * 10
    };

    // find second number
    result += numbers[(index + 1)..(numbers.len())]
        .iter()
        .max()
        .copied()
        .unwrap_or(0);

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_max_2d_value_test() {
        assert_eq!(98, find_max_2d_value("987654321111111"));
        assert_eq!(89, find_max_2d_value("811111111111119"));
        assert_eq!(78, find_max_2d_value("234234234234278"));
        assert_eq!(92, find_max_2d_value("818181911112111"));
        assert_eq!(81, find_max_2d_value("81"));
    }
}
