use core::panic;
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl Color {
    const fn check(self) -> bool {
        match self {
            Self::Red(a) => a <= 12,
            Self::Green(a) => a <= 13,
            Self::Blue(a) => a <= 14,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("day2.txt")?;
    let sum: u32 = file.lines().map(check).sum();

    println!("Sum: {sum}");

    Ok(())
}

fn check(s: &str) -> u32 {
    let game: Vec<&str> = s.split(':').collect();
    let num: u32 = game
        .first()
        .expect("No game found")
        .split(' ')
        .last()
        .expect("Game number malformed")
        .parse::<u32>()
        .expect("Game number not parsed correctly");
    let peeks: Vec<&str> = game.last().expect("No game found").split(';').collect();
    let bool_peeks: Vec<Vec<bool>> = peeks
        .iter()
        .map(|x| -> Vec<bool> { x.split(',').map(|y| parse(y.trim()).check()).collect() })
        .collect();
    let good: bool = bool_peeks.iter().all(|x| x.iter().all(|&y| y));

    if good {
        num
    } else {
        0
    }
}

fn parse(s: &str) -> Color {
    let marbles: Vec<&str> = s.split(' ').collect();
    let num = marbles[0]
        .parse::<u32>()
        .expect("Count not parsed correctly");

    match *marbles
        .get(1)
        .expect("Color-count pair not parsed correctly")
    {
        "blue" => Color::Blue(num),
        "red" => Color::Red(num),
        "green" => Color::Green(num),
        &_ => panic!("Color not parsed correctly"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse1() {
        assert_eq!(parse("1 blue"), Color::Blue(1));
    }

    #[test]
    fn parse2() {
        assert_eq!(parse("10 red"), Color::Red(10));
    }

    #[test]
    fn parse3() {
        assert_eq!(parse("5 green"), Color::Green(5));
    }

    #[test]
    fn parse4() {
        assert_eq!(parse("20 red"), Color::Red(20));
    }

    #[test]
    fn check1() {
        let s: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(check(s), 1);
    }

    #[test]
    fn check2() {
        let s: &str = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        assert_eq!(check(s), 2);
    }

    #[test]
    fn check3() {
        let s: &str = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        assert_eq!(check(s), 0);
    }

    #[test]
    fn check4() {
        let s: &str = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        assert_eq!(check(s), 0);
    }

    #[test]
    fn check5() {
        let s: &str = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(check(s), 5);
    }

    #[test]
    fn check_full() -> Result<(), Box<dyn Error>> {
        let file = fs::read_to_string("test1.txt")?;
        let sum: u32 = file.lines().map(check).sum();

        assert_eq!(sum, 8);

        Ok(())
    }
}
