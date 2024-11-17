use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    let sum = find_soln(Path::new("day4.txt"));

    println!("{sum}");
}

fn find_soln(p: &Path) -> u32 {
    let file = fs::read_to_string(p).expect("File does not exist, fool");
    let cards: Vec<&str> = file
        .lines()
        .map(|x| x.split(':').last().expect("Card not read properly"))
        .collect();
    cards.iter().map(|x| parse_card(x)).sum()
}

fn parse_card(s: &str) -> u32 {
    let sides: Vec<&str> = s.split('|').map(str::trim).collect();
    let wins: HashSet<u32> = parse_nums(sides[0]);
    let nums: HashSet<u32> = parse_nums(sides[1]);
    let count: u32 = get_count(&wins, &nums);

    match count {
        0 => 0,
        _ => 2u32.pow(count - 1),
    }
}

#[allow(clippy::cast_possible_truncation)]
fn get_count(wins: &HashSet<u32>, nums: &HashSet<u32>) -> u32 {
    wins.intersection(nums).count() as u32
}

fn parse_nums(s: &str) -> HashSet<u32> {
    let mut output: HashSet<u32> = HashSet::new();

    s.split(' ')
        .filter_map(|x| x.parse::<u32>().ok())
        .for_each(|x| {
            let _ = output.insert(x);
        });

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse1() {
        let s: &str = "41 48 83 86 17";
        let mut set: HashSet<u32> = HashSet::new();
        set.insert(41);
        set.insert(48);
        set.insert(83);
        set.insert(86);
        set.insert(17);
        assert_eq!(set, parse_nums(s));
    }

    #[test]
    fn parse2() {
        let s: &str = "83 86  6 31 17  9 48 53";
        let mut set: HashSet<u32> = HashSet::new();
        set.insert(83);
        set.insert(86);
        set.insert(6);
        set.insert(31);
        set.insert(17);
        set.insert(9);
        set.insert(48);
        set.insert(53);
        assert_eq!(set, parse_nums(s));
    }

    #[test]
    fn test_count() {
        assert_eq!(
            get_count(
                &parse_nums("41 48 83 86 17"),
                &parse_nums("83 86  6 31 17  9 48 53")
            ),
            4
        );
    }

    #[test]
    fn card1() {
        assert_eq!(parse_card("41 48 83 86 17 | 83 86  6 31 17  9 48 53"), 8);
    }

    #[test]
    fn card2() {
        assert_eq!(parse_card("13 32 20 16 61 | 61 30 68 82 17 32 24 19"), 2);
    }

    #[test]
    fn card3() {
        assert_eq!(parse_card(" 1 21 53 59 44 | 69 82 63 72 16 21 14  1"), 2);
    }

    #[test]
    fn card4() {
        assert_eq!(parse_card("41 92 73 84 69 | 59 84 76 51 58  5 54 83"), 1);
    }

    #[test]
    fn card5() {
        assert_eq!(parse_card("87 83 26 28 32 | 88 30 70 12 93 22 82 36"), 0);
    }

    #[test]
    fn card6() {
        assert_eq!(parse_card("31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 0);
    }

    #[test]
    fn test_soln() {
        assert_eq!(find_soln(Path::new("test.txt")), 13);
    }
}
