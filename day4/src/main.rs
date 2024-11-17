use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    let sum = find_soln_extras(Path::new("day4.txt"));

    println!("{sum}");
}

fn find_soln(p: &Path) -> usize {
    let file = fs::read_to_string(p).expect("File does not exist, fool");
    let cards: Vec<&str> = file
        .lines()
        .map(|x| x.split(':').last().expect("Card not read properly"))
        .collect();
    cards.iter().map(|x| parse_card(x)).sum()
}

fn find_soln_extras(p: &Path) -> usize {
    let file = fs::read_to_string(p).expect("File does not exist, fool");
    let num_cards = file.lines().count();
    let mut memo: Vec<usize> = vec![1usize; num_cards];
    let mut cards = file.lines();
    for i in 1..=num_cards {
        let count = parse_card(cards.next().expect("Mismatch"));
        if count > 0 {
            for j in 1..=count {
                memo[i + j - 1] += memo[i - 1];
            }
        }
    }
    memo.iter().sum()
}

fn parse_card(s: &str) -> usize {
    let sides: Vec<&str> = s.split('|').map(str::trim).collect();
    let wins: HashSet<usize> = parse_nums(sides[0]);
    let nums: HashSet<usize> = parse_nums(sides[1]);
    let count: usize = get_count(&wins, &nums);

    count

    // Used for first challenge
    // match count {
    //     0 => 0,
    //     _ => 2usize.pow(count - 1),
    // }
}

#[allow(clippy::cast_possible_truncation)]
fn get_count(wins: &HashSet<usize>, nums: &HashSet<usize>) -> usize {
    wins.intersection(nums).count() as usize
}

fn parse_nums(s: &str) -> HashSet<usize> {
    let mut output: HashSet<usize> = HashSet::new();

    s.split(' ')
        .filter_map(|x| x.parse::<usize>().ok())
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
        let mut set: HashSet<usize> = HashSet::new();
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
        let mut set: HashSet<usize> = HashSet::new();
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
        assert_eq!(parse_card("41 48 83 86 17 | 83 86  6 31 17  9 48 53"), 4);
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
        assert_eq!(find_soln_extras(Path::new("test.txt")), 30);
    }
}
