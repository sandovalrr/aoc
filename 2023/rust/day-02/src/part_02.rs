/**
--- Part Two ---
Your calculation isn't quite right. It looks like some of the digits are actually spelled
out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
*/

pub fn process(value: &str) -> i32 {
    let output = value
        .lines()
        .map(|line| {
            // Filter out all non-digits and convert them to i32
            // it needs to be mutable because we need to call last() on it
            // and last() consumes the iterator
            let mut i32_chars = line.chars().enumerate().filter_map(|(idx, c)| {
                c.to_digit(10).or({
                    // get substring from index to end of string
                    let substr = &line[idx..];
                    // if the substring starts with one of the words, return the corresponding digit
                    if substr.starts_with("one") {
                        Some(1)
                    } else if substr.starts_with("two") {
                        Some(2)
                    } else if substr.starts_with("three") {
                        Some(3)
                    } else if substr.starts_with("four") {
                        Some(4)
                    } else if substr.starts_with("five") {
                        Some(5)
                    } else if substr.starts_with("six") {
                        Some(6)
                    } else if substr.starts_with("seven") {
                        Some(7)
                    } else if substr.starts_with("eight") {
                        Some(8)
                    } else if substr.starts_with("nine") {
                        Some(9)
                    } else {
                        None
                    }
                })
            });

            println!("{:?}", i32_chars);

            // Get the first digit and pop it from the iterator
            let first = i32_chars.next().expect("should have a first digit");

            // Get the last digit and pop it from the iterator
            match i32_chars.last() {
                Some(last) => format!("{first}{last}"),
                None => format!("{first}{first}"),
            }
            // Parse the string to i32
            .parse::<i32>()
            // if it fails, panic
            .expect("should be able to parse the number")
        })
        .sum();

    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(process(input), 281);
        assert_eq!(process(include_str!("../input_02.txt",)), 54845);
    }
}
