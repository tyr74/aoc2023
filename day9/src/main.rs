use std::fs;
use std::path::Path;

fn main() {
    let sum = full_solve_back(Path::new("day9.txt"));

    println!("Sum: {sum}");
}

fn full_solve(p: &Path) -> isize {
    let file = fs::read_to_string(p).expect("File does not exist, fool");
    file.lines().map(|x| next_term(&parse_ln(x))).sum()
}

fn full_solve_back(p: &Path) -> isize {
    let file = fs::read_to_string(p).expect("File does not exist, fool");
    file.lines()
        .map(|x| {
            let mut rev = parse_ln(x);
            rev.reverse();
            next_term(&rev)
        })
        .sum()
}

fn parse_ln(s: &str) -> Vec<isize> {
    s.split(' ')
        .map(|x| x.parse::<isize>().expect("Number parsed incorrectly"))
        .collect()
}

fn next_term(nums: &[isize]) -> isize {
    if nums.iter().all(|x| *x == 0) {
        return 0;
    }
    let len = nums.len();
    assert!(len != 1, "Sequence cannot be extrapolated");
    nums.last().expect("List of numbers is empty") + next_term(&arr_diffs(nums))
}

fn arr_diffs(nums: &[isize]) -> Vec<isize> {
    let mut output: Vec<isize> = Vec::new();
    for i in 1..nums.len() {
        output.push(nums[i] - nums[i - 1]);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    mod diff {
        use super::*;

        #[test]
        fn diff1() {
            assert_eq!(arr_diffs(&[0, 3, 6, 9, 12, 15]), vec![3, 3, 3, 3, 3]);
        }

        #[test]
        fn diff2() {
            assert_eq!(arr_diffs(&[3, 3, 3, 3, 3]), vec![0, 0, 0, 0]);
        }

        #[test]
        fn diff3() {
            assert_eq!(arr_diffs(&[1, 3, 6, 10, 15, 21]), vec![2, 3, 4, 5, 6]);
        }

        #[test]
        fn diff4() {
            assert_eq!(arr_diffs(&[2, 3, 4, 5, 6]), vec![1, 1, 1, 1]);
        }

        #[test]
        fn diff5() {
            assert_eq!(arr_diffs(&[1, 1, 1, 1]), vec![0, 0, 0]);
        }
    }

    mod next {
        use super::*;

        #[test]
        fn next1() {
            assert_eq!(next_term(&[0, 3, 6, 9, 12, 15]), 18);
        }

        #[test]
        fn next2() {
            assert_eq!(next_term(&[1, 3, 6, 10, 15, 21]), 28);
        }

        #[test]
        fn next3() {
            assert_eq!(next_term(&[10, 13, 16, 21, 30, 45]), 68);
        }
    }

    mod parse {
        use super::*;

        #[test]
        fn parse1() {
            assert_eq!(parse_ln("0 3 6 9 12 15"), vec![0, 3, 6, 9, 12, 15]);
        }

        #[test]
        fn parse2() {
            assert_eq!(parse_ln("1 3 6 10 15 21"), vec![1, 3, 6, 10, 15, 21]);
        }

        #[test]
        fn parse3() {
            assert_eq!(parse_ln("10 13 16 21 30 45"), vec![10, 13, 16, 21, 30, 45]);
        }
    }

    #[test]
    fn full() {
        assert_eq!(full_solve(Path::new("test.txt")), 114);
    }

    #[test]
    fn full_back() {
        assert_eq!(full_solve_back(Path::new("test.txt")), 2);
    }
}
