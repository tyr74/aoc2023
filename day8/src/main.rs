use std::fs;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Node {
    name: String,
    left: String,
    right: String,
    id: char,
}

impl Node {
    fn new(s: &str) -> Self {
        let split: Vec<&str> = s.split('=').map(str::trim).collect();
        let name: String = String::from(split[0]);
        let lr: Vec<&str> = split[1][1..]
            .trim_end_matches(')')
            .split(',')
            .map(str::trim)
            .collect();

        Self {
            left: String::from(lr[0]),
            right: String::from(lr[1]),
            id: name.chars().last().expect("Name is empty"),
            name,
        }
    }
}

fn main() {
    let steps = full_solve(Path::new("day8.txt"));

    println!("Steps: {steps}");
}

fn full_solve(p: &Path) -> usize {
    let (nodes, dir) = parse_file(p);
    let starts: Vec<&Node> = nodes.iter().filter(|&x| x.id == 'A').collect();
    let mut periods: Vec<usize> = Vec::new();

    for n in starts {
        let mut count: usize = 0;
        let mut cur = n;
        while cur.id != 'Z' {
            let cur_dir = dir.chars().nth(count % dir.len()).expect("Dir is empty");
            if cur_dir == 'L' {
                cur = nodes
                    .iter()
                    .find(|x| x.name == cur.left)
                    .expect("Node {cur.left} does not exist");
            } else {
                cur = nodes
                    .iter()
                    .find(|x| x.name == cur.right)
                    .expect("Node {cur.right} does not exist");
            }
            count += 1;
        }
        periods.push(count);
    }

    lcm_vec(&periods)
}

fn lcm_vec(nums: &[usize]) -> usize {
    if nums.len() == 2 {
        lcm(nums[0], nums[1])
    } else {
        lcm(nums[0], lcm_vec(&nums[1..]))
    }
}

const fn lcm(mut num1: usize, mut num2: usize) -> usize {
    let mut output: usize = 1;
    let mut cur: usize = 2;

    while num1 != 1 || num2 != 1 {
        match (num1 % cur, num2 % cur) {
            (0, 0) => {
                num1 /= cur;
                num2 /= cur;
                output *= cur;
            }
            (0, _) => {
                output *= cur;
                num1 /= cur;
            }
            (_, 0) => {
                output *= cur;
                num2 /= cur;
            }
            (_, _) => cur += 1,
        }
    }

    output
}

fn parse_file(p: &Path) -> (Vec<Node>, String) {
    let file = fs::read_to_string(p).expect("File does not exist, fool");
    let mut lines = file.lines();
    let dir: String = String::from(lines.next().expect("File is empty"));
    let mut nodes: Vec<Node> = Vec::new();
    let _ = lines.next();

    for i in lines {
        nodes.push(Node::new(i));
    }

    (nodes, dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcm1() {
        assert_eq!(lcm(7, 10), 70);
    }

    #[test]
    fn lcm2() {
        assert_eq!(lcm(48, 180), 720);
    }

    #[test]
    fn lcm_vec1() {
        assert_eq!(lcm_vec(&[12, 15, 75]), 300);
    }

    #[test]
    fn parse1() {
        assert_eq!(
            parse_file(Path::new("test1.txt")),
            (
                vec![
                    Node {
                        name: String::from("11A"),
                        left: String::from("11B"),
                        right: String::from("XXX"),
                        id: 'A',
                    },
                    Node {
                        name: String::from("11B"),
                        left: String::from("XXX"),
                        right: String::from("11Z"),
                        id: 'B'
                    },
                    Node {
                        name: String::from("11Z"),
                        left: String::from("11B"),
                        right: String::from("XXX"),
                        id: 'Z',
                    },
                    Node {
                        name: String::from("22A"),
                        left: String::from("22B"),
                        right: String::from("XXX"),
                        id: 'A',
                    },
                    Node {
                        name: String::from("22B"),
                        left: String::from("22C"),
                        right: String::from("22C"),
                        id: 'B',
                    },
                    Node {
                        name: String::from("22C"),
                        left: String::from("22Z"),
                        right: String::from("22Z"),
                        id: 'C',
                    },
                    Node {
                        name: String::from("22Z"),
                        left: String::from("22B"),
                        right: String::from("22B"),
                        id: 'Z',
                    },
                    Node {
                        name: String::from("XXX"),
                        left: String::from("XXX"),
                        right: String::from("XXX"),
                        id: 'X',
                    },
                ],
                String::from("LR")
            )
        );
    }

    #[test]
    fn full1() {
        assert_eq!(full_solve(Path::new("test1.txt")), 6);
    }
}
