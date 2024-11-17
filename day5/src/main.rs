use std::fs;
use std::path::Path;

fn main() {
    let min = parse_file(Path::new("day5.txt"));
    println!("Min: {min}");
}

fn parse_file(p: &Path) -> u64 {
    let file = fs::read_to_string(p).expect("File does not exist, fool");
    let mut lines = file.lines();
    let mut maps: Vec<(u64, u64, u64)> = Vec::new();
    let mut locs = parse_seeds(lines.next().expect("File is empty"));
    let mut skip: bool = false;

    for i in lines {
        if skip {
            skip = false;
            continue;
        }
        if i.is_empty() {
            locs.iter_mut().for_each(|x| transform(&maps, x));
            maps.clear();
            skip = true;
        } else {
            maps.push(parse_ln(i));
        }
    }
    locs.iter_mut().for_each(|x| transform(&maps, x));

    locs.into_iter().min().expect("On god idfk what happened")
}

fn transform(maps: &[(u64, u64, u64)], val: &mut u64) {
    for (dest, src, len) in maps {
        if *val >= *src && *val < (*src + *len) {
            *val = *dest + (*val - *src);
            break;
        }
    }
}

fn parse_seeds(s: &str) -> Vec<u64> {
    let nums = s.split(':').last().expect("Malformed seed list");
    nums.trim()
        .split(' ')
        .map(|x| x.parse::<u64>().expect("Seed number parsed incorrectly"))
        .collect()
}

fn parse_ln(s: &str) -> (u64, u64, u64) {
    let nums: Vec<u64> = s
        .trim()
        .split(' ')
        .map(|x| x.parse::<u64>().expect("Swap number parsed incorrectly"))
        .collect();

    (nums[0], nums[1], nums[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeds() {
        let s: &str = "seeds: 79 14 55 13";
        assert_eq!(parse_seeds(s), vec![79, 14, 55, 13]);
    }

    #[test]
    fn parse1() {
        let s: &str = "50 98 2";
        assert_eq!(parse_ln(s), (50, 98, 2));
    }

    #[test]
    fn parse2() {
        let s: &str = "0 15 37";
        assert_eq!(parse_ln(s), (0, 15, 37));
    }

    #[test]
    fn parse3() {
        let s: &str = "49 53 8";
        assert_eq!(parse_ln(s), (49, 53, 8));
    }

    #[test]
    fn parse4() {
        let s: &str = "1 0 69";
        assert_eq!(parse_ln(s), (1, 0, 69));
    }

    #[test]
    fn transform1() {
        let mut num: u64 = 79;
        transform(&[(50, 98, 2), (52, 50, 48)], &mut num);
        assert_eq!(num, 81);
    }

    #[test]
    fn transform2() {
        let mut num: u64 = 14;
        transform(&[(50, 98, 2), (52, 50, 48)], &mut num);
        assert_eq!(num, 14);
    }

    #[test]
    fn transform3() {
        let mut num: u64 = 55;
        transform(&[(50, 98, 2), (52, 50, 48)], &mut num);
        assert_eq!(num, 57);
    }

    #[test]
    fn transform4() {
        let mut num: u64 = 13;
        transform(&[(50, 98, 2), (52, 50, 48)], &mut num);
        assert_eq!(num, 13);
    }

    #[test]
    fn transform5() {
        let mut num: u64 = 99;
        transform(&[(50, 98, 2), (52, 50, 48)], &mut num);
        assert_eq!(num, 51);
    }

    #[test]
    fn full() {
        let p: &Path = Path::new("test.txt");
        assert_eq!(parse_file(p), 35);
    }
}
