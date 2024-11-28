use std::collections::VecDeque;
use std::fs;
use std::path::Path;

#[derive(Debug)]
enum Dir {
    North,
    East,
    South,
    West,
    All,
}

#[derive(Debug)]
struct Loc {
    i: usize,
    j: usize,
    dist: usize,
    next: Dir,
}

impl PartialEq for Loc {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j
    }
}

fn main() {
    let grid = create_array(Path::new("day10.txt"));
    let far = search(&grid);

    println!("Farthest: {far}");
}

fn search(grid: &[Vec<char>]) -> usize {
    let mut visited: Vec<Loc> = Vec::new();
    let mut next: VecDeque<Loc> = VecDeque::new();

    next.extend(get_start(grid));

    while let Some(cur) = next.pop_front() {
        if visited.contains(&cur) {
            continue;
        }
        next.push_back(get_next(&cur, grid));
        visited.push(cur);
    }

    visited
        .iter()
        .max_by(|&x, &y| x.dist.cmp(&y.dist))
        .expect("Loop does not exist")
        .dist
}

#[allow(clippy::match_on_vec_items, clippy::too_many_lines)]
fn get_next(loc: &Loc, grid: &[Vec<char>]) -> Loc {
    match loc.next {
        Dir::North => match grid[loc.i - 1][loc.j] {
            'F' => Loc {
                i: loc.i - 1,
                j: loc.j,
                dist: loc.dist + 1,
                next: Dir::East,
            },
            '7' => Loc {
                i: loc.i - 1,
                j: loc.j,
                dist: loc.dist + 1,
                next: Dir::West,
            },
            '|' => Loc {
                i: loc.i - 1,
                j: loc.j,
                dist: loc.dist + 1,
                next: Dir::North,
            },
            _ => panic!("Tunnel dead ends"),
        },
        Dir::East => match grid[loc.i][loc.j + 1] {
            'J' => Loc {
                i: loc.i,
                j: loc.j + 1,
                dist: loc.dist + 1,
                next: Dir::North,
            },
            '7' => Loc {
                i: loc.i,
                j: loc.j + 1,
                dist: loc.dist + 1,
                next: Dir::South,
            },
            '-' => Loc {
                i: loc.i,
                j: loc.j + 1,
                dist: loc.dist + 1,
                next: Dir::East,
            },
            _ => panic!("Tunnel dead ends"),
        },
        Dir::South => match grid[loc.i + 1][loc.j] {
            'J' => Loc {
                i: loc.i + 1,
                j: loc.j,
                dist: loc.dist + 1,
                next: Dir::West,
            },
            'L' => Loc {
                i: loc.i + 1,
                j: loc.j,
                dist: loc.dist + 1,
                next: Dir::East,
            },
            '|' => Loc {
                i: loc.i + 1,
                j: loc.j,
                dist: loc.dist + 1,
                next: Dir::South,
            },
            _ => panic!("Tunnel dead ends"),
        },
        Dir::West => match grid[loc.i][loc.j - 1] {
            'L' => Loc {
                i: loc.i,
                j: loc.j - 1,
                dist: loc.dist + 1,
                next: Dir::North,
            },
            'F' => Loc {
                i: loc.i,
                j: loc.j - 1,
                dist: loc.dist + 1,
                next: Dir::South,
            },
            '-' => Loc {
                i: loc.i,
                j: loc.j - 1,
                dist: loc.dist + 1,
                next: Dir::West,
            },
            _ => panic!("Tunnel dead ends"),
        },
        Dir::All => panic!("Dir::All shouldn't be passed to this function"),
    }
}

#[allow(clippy::match_on_vec_items, clippy::too_many_lines)]
fn get_start(grid: &[Vec<char>]) -> Vec<Loc> {
    let mut output: Vec<Loc> = Vec::new();
    let mut start: Loc = Loc {
        i: 0,
        j: 0,
        dist: 1,
        next: Dir::All,
    };

    for (i, arr) in grid.iter().enumerate() {
        for (j, c) in arr.iter().enumerate() {
            if *c == 'S' {
                start = Loc {
                    i,
                    j,
                    dist: 0,
                    next: Dir::All,
                };
                break;
            }
        }
    }

    assert!(start.dist != 1, "S character was not found.");

    if let Some(row) = grid.get(start.i.wrapping_sub(1)) {
        match row[start.j] {
            'F' => output.push(Loc {
                i: start.i - 1,
                j: start.j,
                dist: 1,
                next: Dir::East,
            }),
            '7' => output.push(Loc {
                i: start.i - 1,
                j: start.j,
                dist: 1,
                next: Dir::West,
            }),
            '|' => output.push(Loc {
                i: start.i - 1,
                j: start.j,
                dist: 1,
                next: Dir::North,
            }),
            _ => (),
        }
    }
    if let Some(row) = grid.get(start.i + 1) {
        match row[start.j] {
            'J' => output.push(Loc {
                i: start.i + 1,
                j: start.j,
                dist: 1,
                next: Dir::West,
            }),
            'L' => output.push(Loc {
                i: start.i + 1,
                j: start.j,
                dist: 1,
                next: Dir::East,
            }),
            '|' => output.push(Loc {
                i: start.i + 1,
                j: start.j,
                dist: 1,
                next: Dir::South,
            }),
            _ => (),
        }
    }
    if let Some(c) = grid[start.i].get(start.j.wrapping_sub(1)) {
        match c {
            'L' => output.push(Loc {
                i: start.i,
                j: start.j - 1,
                dist: 1,
                next: Dir::North,
            }),
            'F' => output.push(Loc {
                i: start.i,
                j: start.j - 1,
                dist: 1,
                next: Dir::South,
            }),
            '-' => output.push(Loc {
                i: start.i,
                j: start.j - 1,
                dist: 1,
                next: Dir::West,
            }),
            _ => (),
        }
    }
    if let Some(c) = grid[start.i].get(start.j + 1) {
        match c {
            'J' => output.push(Loc {
                i: start.i,
                j: start.j + 1,
                dist: 1,
                next: Dir::North,
            }),
            '7' => output.push(Loc {
                i: start.i,
                j: start.j + 1,
                dist: 1,
                next: Dir::South,
            }),
            '-' => output.push(Loc {
                i: start.i,
                j: start.j + 1,
                dist: 1,
                next: Dir::East,
            }),
            _ => (),
        }
    }

    output
}

fn create_array(p: &Path) -> Vec<Vec<char>> {
    fs::read_to_string(p)
        .expect("File does not exist, fool")
        .lines()
        .map(|x| -> Vec<char> { x.trim().chars().collect() })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start1() {
        let grid = create_array(Path::new("test1.txt"));
        let expected: Vec<Loc> = vec![
            Loc {
                i: 2,
                j: 1,
                dist: 1,
                next: Dir::South,
            },
            Loc {
                i: 1,
                j: 2,
                dist: 1,
                next: Dir::West,
            },
        ];
        assert_eq!(get_start(&grid), expected);
    }

    #[test]
    fn start2() {
        let grid = create_array(Path::new("test2.txt"));
        let expected: Vec<Loc> = vec![
            Loc {
                i: 3,
                j: 0,
                dist: 1,
                next: Dir::South,
            },
            Loc {
                i: 2,
                j: 1,
                dist: 1,
                next: Dir::North,
            },
        ];
        assert_eq!(get_start(&grid), expected);
    }
}
