use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("day1.txt")?;
    let sum: u32 = file.lines().map(|x| parse(x).unwrap()).sum();

    println!("{}", sum);

    Ok(())
}

fn parse(s: &str) -> Option<u32> {
    let nums: Vec<u32> = s.chars().filter_map(|x| x.to_digit(10)).collect();

    Some(nums.first()? * 10 + nums.last()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_num1() {
        let s: &str = "1abc2";
        println!("{s}");
        assert_eq!(parse("1abc2"), Some(12));
    }

    #[test]
    fn parse_num2() {
        let s: &str = "pqr3stu8vwx";
        println!("{s}");
        assert_eq!(parse("pqr3stu8vwx"), Some(38));
    }

    #[test]
    fn parse_num3() {
        let s: &str = "a1b2c3d4e5f";
        println!("{s}");
        assert_eq!(parse("a1b2c3d4e5f"), Some(15));
    }

    #[test]
    fn parse_num4() {
        let s: &str = "treb7uchet";
        println!("{s}");
        assert_eq!(parse("treb7uchet"), Some(77));
    }

    #[test]
    fn full_sum() -> Result<(), Box<dyn Error>> {
        let file = fs::read_to_string("test.txt")?;
        let sum: u32 = file.lines().map(|x| parse(x).unwrap()).sum();
        assert_eq!(sum, 142);
        Ok(())
    }
}
