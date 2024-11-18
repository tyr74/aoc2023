use std::fs;
use std::path::Path;

fn main() {
    let min = parse_file_extra(Path::new("day5.txt"));
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

fn parse_file_extra(p: &Path) -> u64 {
    let file = fs::read_to_string(p).expect("File does not exist, fool");
    let mut lines = file.lines();
    let mut maps: Vec<(u64, u64, u64)> = Vec::new();
    let mut locs: Vec<(u64, u64)> = parse_seed_pairs(lines.next().expect("File is empty"));
    let mut temp: Vec<(u64, u64)> = Vec::new();
    let mut skip: bool = false;

    for i in lines {
        if skip {
            skip = false;
            continue;
        }
        if i.is_empty() {
            for j in &locs {
                temp.extend(transform_pairs(&maps, *j));
            }
            locs.clone_from(&temp);
            temp.clear();
            maps.clear();
            skip = true;
        } else {
            maps.push(parse_ln(i));
        }
    }
    for j in &locs {
        temp.extend(transform_pairs(&maps, *j));
    }
    locs.clone_from(&temp);

    locs.into_iter().min().expect("On god idfk what happened").0
}

fn transform(maps: &[(u64, u64, u64)], val: &mut u64) {
    for (dest, src, len) in maps {
        if *val >= *src && *val < (*src + *len) {
            *val = *dest + (*val - *src);
            break;
        }
    }
}

fn transform_pairs(maps: &[(u64, u64, u64)], pair: (u64, u64)) -> Vec<(u64, u64)> {
    let mut output: Vec<(u64, u64)> = Vec::new();
    for (dest, src, len) in maps {
        let ranges: Vec<(u64, u64)> = split_ranges((*src, *src + *len - 1), pair);
        match ranges.len() {
            0 => continue,
            1 => {
                output.push(transform_internal_pair((*dest, *src), ranges[0]));
                break;
            }
            2 => {
                if ranges[1].0 == *src {
                    output.push(transform_internal_pair((*dest, *src), ranges[1]));
                    output.extend(transform_pairs(maps, ranges[0]));
                } else {
                    output.push(transform_internal_pair((*dest, *src), ranges[0]));
                    output.extend(transform_pairs(maps, ranges[1]));
                }
                break;
            }
            3 => {
                output.push(transform_internal_pair((*dest, *src), ranges[1]));
                output.extend(transform_pairs(maps, ranges[0]));
                output.extend(transform_pairs(maps, ranges[2]));
                break;
            }
            _ => panic!("How have you managed to do this?"),
        }
    }
    if output.is_empty() {
        output.push(pair);
    }

    output
}

#[allow(clippy::suspicious_operation_groupings)]
fn split_ranges(range: (u64, u64), pair: (u64, u64)) -> Vec<(u64, u64)> {
    let mut output: Vec<(u64, u64)> = Vec::new();
    if (pair.0 < range.0 && pair.1 < range.0) || (pair.0 > range.1 && pair.1 > range.1) {
        return vec![];
    } else if pair.0 >= range.0 && pair.1 <= range.1 {
        output.push(pair);
    } else if pair.0 < range.0 && pair.1 > range.1 {
        output.push((pair.0, range.0 - 1));
        output.push(range);
        output.push((range.1 + 1, pair.1));
    } else if pair.0 < range.0 && pair.1 <= range.1 {
        output.push((pair.0, range.0 - 1));
        output.push((range.0, pair.1));
    } else if pair.0 >= range.0 && pair.1 > range.1 {
        output.push((pair.0, range.1));
        output.push((range.1 + 1, pair.1));
    }

    output
}

const fn transform_internal_pair(map: (u64, u64), pair: (u64, u64)) -> (u64, u64) {
    (map.0 + (pair.0 - map.1), map.0 + (pair.1 - map.1))
}

fn parse_seeds(s: &str) -> Vec<u64> {
    let nums = s.split(':').last().expect("Malformed seed list");
    nums.trim()
        .split(' ')
        .map(|x| x.parse::<u64>().expect("Seed number parsed incorrectly"))
        .collect()
}

fn parse_seed_pairs(s: &str) -> Vec<(u64, u64)> {
    let nums: Vec<u64> = s
        .split(':')
        .last()
        .expect("Malformed seed list")
        .trim()
        .split(' ')
        .map(|x| x.parse::<u64>().expect("Seed number parsed incorrectly"))
        .collect();
    let mut output: Vec<(u64, u64)> = Vec::new();

    for i in (0..nums.len()).step_by(2) {
        output.push((nums[i], nums[i] + nums[i + 1] - 1));
    }

    output
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
        let nums: Vec<(u64, u64)> = vec![(79, 92), (55, 67)];
        assert_eq!(parse_seed_pairs(s), nums);
    }

    #[test]
    fn transform_int_pair1() {
        let nums = (79, 92);
        assert_eq!(transform_internal_pair((52, 50), nums), (81, 94));
    }

    #[test]
    fn transform_int_pair2() {
        let nums = (55, 67);
        assert_eq!(transform_internal_pair((52, 50), nums), (57, 69));
    }

    #[test]
    fn transform_pair1() {
        let nums = (79, 92);
        assert_eq!(
            transform_pairs(&[(50, 98, 2), (52, 50, 48)], nums),
            vec![(81, 94)]
        );
    }

    #[test]
    fn transform_pair2() {
        let nums = (55, 67);
        assert_eq!(
            transform_pairs(&[(50, 98, 2), (52, 50, 48)], nums),
            vec![(57, 69)]
        );
    }

    #[test]
    fn transform_pair3() {
        let nums = (95, 99);
        assert_eq!(
            transform_pairs(&[(50, 98, 2), (52, 50, 48)], nums),
            vec![(50, 51), (97, 99)]
        );
    }

    #[test]
    fn split1() {
        assert_eq!(
            split_ranges((51, 60), (41, 70)),
            vec![(41, 50), (51, 60), (61, 70)]
        );
    }

    #[test]
    fn split2() {
        assert_eq!(split_ranges((51, 60), (41, 55)), vec![(41, 50), (51, 55)]);
    }

    #[test]
    fn split3() {
        assert_eq!(split_ranges((51, 60), (55, 70)), vec![(55, 60), (61, 70)]);
    }

    #[test]
    fn split4() {
        assert_eq!(split_ranges((51, 60), (52, 58)), vec![(52, 58)]);
    }

    #[test]
    fn split5() {
        assert_eq!(split_ranges((51, 60), (51, 70)), vec![(51, 60), (61, 70)]);
    }

    #[test]
    fn split6() {
        assert_eq!(split_ranges((51, 60), (41, 50)), vec![]);
    }

    #[test]
    fn split7() {
        assert_eq!(split_ranges((51, 60), (61, 70)), vec![]);
    }

    #[test]
    fn split8() {
        assert_eq!(split_ranges((51, 60), (51, 60)), vec![(51, 60)]);
    }

    #[test]
    fn split9() {
        assert_eq!(split_ranges((51, 60), (41, 60)), vec![(41, 50), (51, 60)]);
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
    #[ignore]
    fn transform1() {
        let mut num: u64 = 79;
        transform(&[(50, 98, 2), (52, 50, 48)], &mut num);
        assert_eq!(num, 81);
    }

    #[test]
    #[ignore]
    fn transform2() {
        let mut num: u64 = 14;
        transform(&[(50, 98, 2), (52, 50, 48)], &mut num);
        assert_eq!(num, 14);
    }

    #[test]
    #[ignore]
    fn transform3() {
        let mut num: u64 = 55;
        transform(&[(50, 98, 2), (52, 50, 48)], &mut num);
        assert_eq!(num, 57);
    }

    #[test]
    #[ignore]
    fn transform4() {
        let mut num: u64 = 13;
        transform(&[(50, 98, 2), (52, 50, 48)], &mut num);
        assert_eq!(num, 13);
    }

    #[test]
    #[ignore]
    fn transform5() {
        let mut num: u64 = 99;
        transform(&[(50, 98, 2), (52, 50, 48)], &mut num);
        assert_eq!(num, 51);
    }

    #[test]
    fn full() {
        let p: &Path = Path::new("test.txt");
        assert_eq!(parse_file_extra(p), 46);
    }
}
