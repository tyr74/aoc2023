use std::fs;
use std::path::Path;

type CharGrid = Vec<Vec<char>>;

#[derive(Debug)]
struct Number {
    value: u32,
    locations: Vec<(u32, u32)>,
    included: bool,
}

impl Number {
    fn close(&self, pt: (u32, u32)) -> bool {
        self.locations
            .iter()
            .any(|&x| x.0.abs_diff(pt.0) <= 1 && x.1.abs_diff(pt.1) <= 1)
    }
}

fn main() {
    let sum = get_gear_ratio(Path::new("day3.txt"));
    println!("{sum}");
}

fn get_sum(p: &Path) -> u32 {
    let arr: CharGrid = create_array(p);
    let mut nums = find_nums(&arr);
    let syms = find_syms(&arr);
    compare_nums(&mut nums, &syms);
    nums.iter()
        .filter_map(|x| if x.included { Some(x.value) } else { None })
        .sum()
}

fn get_gear_ratio(p: &Path) -> u32 {
    let arr: CharGrid = create_array(p);
    let nums = find_nums(&arr);
    let gears = find_gears(&arr);
    let ratios = compare_gears(&nums, &gears);

    ratios.iter().map(|x| x.0 * x.1).sum()
}

fn compare_nums(nums: &mut [Number], syms: &[(u32, u32)]) {
    for pt in syms {
        for x in nums.iter_mut() {
            if x.close(*pt) {
                x.included = true;
            }
        }
    }
}

fn compare_gears(nums: &[Number], syms: &[(u32, u32)]) -> Vec<(u32, u32)> {
    let mut possible: Vec<&Number> = Vec::new();
    let mut output: Vec<(u32, u32)> = Vec::new();

    for pt in syms {
        for x in nums {
            if x.close(*pt) {
                possible.push(x);
            }
        }
        if possible.len() == 2 {
            output.push((possible[0].value, possible[1].value));
        }
        possible = Vec::new();
    }

    output
}

#[allow(clippy::cast_possible_truncation)]
fn find_syms(arr: &CharGrid) -> Vec<(u32, u32)> {
    let mut output: Vec<(u32, u32)> = Vec::new();

    for (i, ln) in arr.iter().enumerate() {
        for (j, c) in ln.iter().enumerate() {
            if !(c.is_numeric() || *c == '.') {
                output.push((i as u32, j as u32));
            }
        }
    }

    output
}

#[allow(clippy::cast_possible_truncation)]
fn find_gears(arr: &CharGrid) -> Vec<(u32, u32)> {
    let mut output: Vec<(u32, u32)> = Vec::new();

    for (i, ln) in arr.iter().enumerate() {
        for (j, c) in ln.iter().enumerate() {
            if *c == '*' {
                output.push((i as u32, j as u32));
            }
        }
    }

    output
}

#[allow(clippy::cast_possible_truncation)]
fn find_nums(arr: &CharGrid) -> Vec<Number> {
    let mut cur_int: u32 = 0;
    let mut cur_vec: Vec<(u32, u32)> = Vec::new();
    let mut output: Vec<Number> = Vec::new();

    for (i, ln) in arr.iter().enumerate() {
        for (j, c) in ln.iter().enumerate() {
            if let Some(num) = c.to_digit(10) {
                cur_int = cur_int * 10 + num;
                cur_vec.push((i as u32, j as u32));
            } else {
                if cur_int != 0 && !cur_vec.is_empty() {
                    output.push(Number {
                        value: cur_int,
                        locations: cur_vec,
                        included: false,
                    });
                }
                cur_int = 0;
                cur_vec = Vec::new();
            }
        }
        if cur_int != 0 && !cur_vec.is_empty() {
            output.push(Number {
                value: cur_int,
                locations: cur_vec,
                included: false,
            });
        }
        cur_int = 0;
        cur_vec = Vec::new();
    }

    output
}

fn create_array(p: &Path) -> Vec<Vec<char>> {
    fs::read_to_string(p)
        .expect("Error reading file")
        .lines()
        .map(|x| -> Vec<char> { x.trim().chars().collect() })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_up_left() {
        let num = Number {
            value: 10,
            locations: vec![(2, 2)],
            included: false,
        };
        assert!(num.close((1, 1)));
        assert!(!num.close((0, 0)));
    }

    #[test]
    fn test_up() {
        let num = Number {
            value: 10,
            locations: vec![(2, 2)],
            included: false,
        };
        assert!(num.close((1, 2)));
        assert!(!num.close((0, 2)));
    }

    #[test]
    fn test_up_right() {
        let num = Number {
            value: 10,
            locations: vec![(2, 2)],
            included: false,
        };
        assert!(num.close((1, 3)));
        assert!(!num.close((0, 4)));
    }

    #[test]
    fn test_right() {
        let num = Number {
            value: 10,
            locations: vec![(2, 2)],
            included: false,
        };
        assert!(num.close((2, 3)));
        assert!(!num.close((2, 4)));
    }

    #[test]
    fn test_left() {
        let num = Number {
            value: 10,
            locations: vec![(2, 2)],
            included: false,
        };
        assert!(num.close((2, 1)));
        assert!(!num.close((2, 0)));
    }

    #[test]
    fn test_down_left() {
        let num = Number {
            value: 10,
            locations: vec![(2, 2)],
            included: false,
        };
        assert!(num.close((3, 1)));
        assert!(!num.close((5, 0)));
    }

    #[test]
    fn test_down() {
        let num = Number {
            value: 10,
            locations: vec![(2, 2)],
            included: false,
        };
        assert!(num.close((3, 2)));
        assert!(!num.close((5, 2)));
    }

    #[test]
    fn test_down_right() {
        let num = Number {
            value: 10,
            locations: vec![(2, 2)],
            included: false,
        };
        assert!(num.close((3, 3)));
        assert!(!num.close((5, 5)));
    }

    #[test]
    fn test_sum() {
        let sum = get_sum(Path::new("test.txt"));
        assert_eq!(sum, 4361);
    }

    #[test]
    fn test_gears() {
        let sum = get_gear_ratio(Path::new("test.txt"));
        assert_eq!(sum, 467835);
    }
}
