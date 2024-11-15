use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("day1.txt")?;
    let sum: u32 = file.lines().map(parse_words).sum();

    println!("{sum}");

    Ok(())
}

#[allow(dead_code)]
fn parse(s: &str) -> u32 {
    let nums: Vec<u32> = s.chars().filter_map(|x| x.to_digit(10)).collect();

    match (nums.first(), nums.last()) {
        (Some(a), Some(b)) => a * 10 + b,
        _ => panic!("There weren't any integers in the string"),
    }
}

fn parse_words(s: &str) -> u32 {
    let mut first: u32 = 0;
    let mut last: u32 = 0;

    for i in 0..s.len() {
        if let Some(a) = check_num(&s[..=i]) {
            first = a;
            break;
        }
    }
    for i in (0..s.len()).rev() {
        if let Some(a) = check_num(&s[i..]) {
            last = a;
            break;
        }
    }

    first * 10 + last
}

fn check_num(s: &str) -> Option<u32> {
    if s.contains('1') || s.contains("one") {
        Some(1)
    } else if s.contains('2') || s.contains("two") {
        Some(2)
    } else if s.contains('3') || s.contains("three") {
        Some(3)
    } else if s.contains('4') || s.contains("four") {
        Some(4)
    } else if s.contains('5') || s.contains("five") {
        Some(5)
    } else if s.contains('6') || s.contains("six") {
        Some(6)
    } else if s.contains('7') || s.contains("seven") {
        Some(7)
    } else if s.contains('8') || s.contains("eight") {
        Some(8)
    } else if s.contains('9') || s.contains("nine") {
        Some(9)
    } else if s.contains('0') || s.contains("zero") {
        Some(0)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_num1() {
        let s: &str = "1abc2";
        println!("{s}");
        assert_eq!(parse("1abc2"), 12);
    }

    #[test]
    fn parse_num2() {
        let s: &str = "pqr3stu8vwx";
        println!("{s}");
        assert_eq!(parse("pqr3stu8vwx"), 38);
    }

    #[test]
    fn parse_num3() {
        let s: &str = "a1b2c3d4e5f";
        println!("{s}");
        assert_eq!(parse("a1b2c3d4e5f"), 15);
    }

    #[test]
    fn parse_num4() {
        let s: &str = "treb7uchet";
        println!("{s}");
        assert_eq!(parse("treb7uchet"), 77);
    }

    #[test]
    fn full_sum() -> Result<(), Box<dyn Error>> {
        let file = fs::read_to_string("test1.txt")?;
        let sum: u32 = file.lines().map(parse).sum();
        assert_eq!(sum, 142);
        Ok(())
    }

    #[test]
    fn parse_word1() {
        let s: &str = "two1nine";
        println!("{s}");
        assert_eq!(parse_words(s), 29);
    }

    #[test]
    fn parse_word2() {
        let s: &str = "eightwothree";
        println!("{s}");
        assert_eq!(parse_words(s), 83);
    }

    #[test]
    fn parse_word3() {
        let s: &str = "abcone2threexyz";
        println!("{s}");
        assert_eq!(parse_words(s), 13);
    }

    #[test]
    fn parse_word4() {
        let s: &str = "xtwone3four";
        println!("{s}");
        assert_eq!(parse_words(s), 24);
    }

    #[test]
    fn parse_word5() {
        let s: &str = "4nineeightseven2";
        println!("{s}");
        assert_eq!(parse_words(s), 42);
    }

    #[test]
    fn parse_word6() {
        let s: &str = "zoneight234";
        println!("{s}");
        assert_eq!(parse_words(s), 14);
    }

    #[test]
    fn parse_word7() {
        let s: &str = "7pqrstsixteen";
        println!("{s}");
        assert_eq!(parse_words(s), 76);
    }

    #[test]
    fn word_sum() -> Result<(), Box<dyn Error>> {
        let file = fs::read_to_string("test2.txt")?;
        let sum: u32 = file.lines().map(parse_words).sum();
        assert_eq!(sum, 281);

        Ok(())
    }
}
