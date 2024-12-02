use std::fs;
use std::iter::zip;
use std::path::Path;

fn main() {
    let sum = full_count(Path::new("day12.txt"));

    println!("Sum: {sum}");
}

fn full_count(p: &Path) -> u32 {
    let mut sum: u32 = 0;
    let (maps, nums) = parse_in(p);
    for (m, n) in zip(maps, nums) {
        sum += count_permutations(&m, &n);
    }

    sum
}

fn count_permutations(map: &str, nums: &[u32]) -> u32 {
    let mut count: u32 = 0;
    let mut cpy: String;
    let unknwns = count_unknowns(map);

    for mut i in 0..2_u32.pow(unknwns) {
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

fn count_unknowns(map: &str) -> u32 {
    let mut count: u32 = 0;

    for c in map.chars() {
        if c == '?' {
            count += 1;
        }
    }

    count
}

fn calc_cont(map: &str) -> Vec<u32> {
    let mut out: Vec<u32> = Vec::new();
    let mut cur: u32 = 0;

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

fn parse_in(p: &Path) -> (Vec<String>, Vec<Vec<u32>>) {
    let file = fs::read_to_string(p).expect("File does not exist, fool");
    let maps: Vec<String> = file
        .lines()
        .map(|x| String::from(x.split(' ').next().expect("Line is empty")))
        .collect();
    let nums: Vec<Vec<u32>> = file
        .lines()
        .map(|x| -> Vec<u32> {
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
