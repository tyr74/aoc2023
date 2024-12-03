use itertools::Itertools;
use std::fs;
use std::iter::repeat;
use std::iter::zip;
use std::path::Path;

#[derive(Debug, Clone)]
struct Record {
    springs: String,
    groups: Vec<usize>,
}

fn main() {
    let sum = full_count(Path::new("day12.txt"));

    println!("Sum: {sum}");
}

fn parse_in_two(p: &Path) -> (Vec<String>, Vec<Vec<usize>>) {
    let file = fs::read_to_string(p).expect("File does not exist, fool");
    let mut maps: Vec<String> = Vec::new();
    let mut nums: Vec<Vec<usize>> = Vec::new();
    for line in file.lines() {
        let (springs, groups) = line.split_once(' ').unwrap();
        let springs = repeat(springs).take(5).collect::<Vec<_>>().join("?");
        let groups = groups
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec()
            .repeat(5);
        maps.push(springs);
        nums.push(groups);
    }

    (maps, nums)
}

fn full_count(p: &Path) -> usize {
    let mut sum: usize = 0;
    let (maps, nums) = parse_in_two(p);
    for (m, n) in zip(maps, nums) {
        sum += do_thing(&m, &n);
    }

    sum
}

fn do_thing(springs: &str, groups: &[usize]) -> usize {
    let mut dp: Vec<Vec<usize>> =
        vec![vec![0; groups.len()]; springs.len() + groups[groups.len() - 1] + 1];
    let mut min_j = 0;
    'i: for i in 0..springs.len() {
        if i > 0 {
            dp[i - 1].clear();
        }
        for j in 0..groups.len() {
            let cur_char = &springs[i..i + 1];
            if j < min_j {
                continue;
            }
            if cur_char == "#" && j == 0 {
                min_j = 1;
            }
            if cur_char == "." {
                continue 'i;
            }
            if j > 0 && dp[i][j - 1] == 0 {
                continue;
            }
            if groups[j..].iter().sum::<usize>() + groups[j..].len() - 1 > springs[i..].len() {
                continue;
            }
            if (j == groups.len() - 1) && springs[i + groups[j]..].chars().any(|c| c == '#') {
                continue;
            }
            let max_idx = springs.len().min(i + groups[j]);
            let end_reached = max_idx == springs.len();
            let subsequent_character = springs.get(max_idx..max_idx + 1).unwrap_or("");
            let group_valid = springs[i..i + groups[j]]
                .chars()
                .all(|x| x == '?' || x == '#')
                && (end_reached || subsequent_character != "#");
            if !group_valid {
                continue;
            }

            let next_start_idx = (springs.len()).min(i + groups[j] + 1);
            let next_broken_idx = match springs[next_start_idx..].find('#') {
                Some(n) => next_start_idx + n,
                None => dp.len() - 1,
            };
            for k in next_start_idx..=next_broken_idx {
                if j > 0 {
                    dp[k][j] += dp[i][j - 1];
                } else {
                    dp[k][j] += 1;
                }
            }
        }
    }
    dp[dp.len() - 1][dp[dp.len() - 1].len() - 1]
}

#[allow(clippy::cast_possible_truncation)]
fn count_permutations(map: &str, nums: &[usize]) -> usize {
    let mut count: usize = 0;
    let mut cpy: String;
    let unknwns = count_unknowns(map);

    for mut i in 0..2_usize.pow(unknwns as u32) {
        cpy = String::from(map);
        while i > 0 {
            if i % 2 == 1 {
                cpy = cpy.replacen('?', "#", 1);
            } else {
                cpy = cpy.replacen('?', ".", 1);
            }
            i /= 2;
        }
        cpy = cpy.replace('?', ".");
        if calc_cont(&cpy) == nums.to_vec() {
            count += 1;
        }
    }

    count
}

fn count_unknowns(map: &str) -> usize {
    let mut count: usize = 0;

    for c in map.chars() {
        if c == '?' {
            count += 1;
        }
    }

    count
}

fn calc_cont(map: &str) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();
    let mut cur: usize = 0;

    for c in map.chars() {
        if c == '#' {
            cur += 1;
        } else if cur != 0 {
            out.push(cur);
            cur = 0;
        }
    }
    if cur != 0 {
        out.push(cur);
    }

    out
}

fn parse_in(p: &Path) -> (Vec<String>, Vec<Vec<usize>>) {
    let file = fs::read_to_string(p).expect("File does not exist, fool");
    let maps: Vec<String> = file
        .lines()
        .map(|x| String::from(x.split(' ').next().expect("Line is empty")))
        .collect();
    let nums: Vec<Vec<usize>> = file
        .lines()
        .map(|x| -> Vec<usize> {
            x.split(' ')
                .last()
                .expect("Line is empty")
                .split(',')
                .map(|y| y.parse().expect("Number parsed incorrectly"))
                .collect()
        })
        .collect();

    (maps, nums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc1() {
        assert_eq!(calc_cont("#.#.###"), vec![1, 1, 3]);
    }

    #[test]
    fn calc2() {
        assert_eq!(calc_cont(".#...#....###."), vec![1, 1, 3]);
    }

    #[test]
    fn calc3() {
        assert_eq!(calc_cont(".#.###.#.######"), vec![1, 3, 1, 6]);
    }

    #[test]
    fn calc4() {
        assert_eq!(calc_cont("####.#...#..."), vec![4, 1, 1]);
    }

    #[test]
    fn calc5() {
        assert_eq!(calc_cont("#....######..#####."), vec![1, 6, 5]);
    }

    #[test]
    fn calc6() {
        assert_eq!(calc_cont(".###.##....#"), vec![3, 2, 1]);
    }
}
