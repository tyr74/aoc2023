use std::fs;
use std::path::Path;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct Point(usize, usize);

#[derive(Debug, Clone, Copy)]
enum Dir {
    North,
    East,
    South,
    West,
}

fn main() {
    let mut grid = parse_in(Path::new("day14.txt"));
    let mut rounds = find_rounds(&grid);

    // for _ in 0..1_000_000_000 {
    //     rounds = cycle(&mut grid, &rounds);
    // }

    let now = Instant::now();
    for _ in 0..1_000 {
        rounds = cycle(&mut grid, &rounds);
    }
    let elapsed = now.elapsed();

    println!("Duration: {elapsed:.2?}");

    let sum: usize = all_load(&grid);

    println!("Load: {sum}");
}

fn all_load(grid: &[Vec<char>]) -> usize {
    let mut sum: usize = 0;

    for r in find_rounds(grid) {
        sum += get_load(grid, r);
    }

    sum
}

fn print_grid(grid: &[Vec<char>]) {
    for ln in grid {
        println!("{ln:?}");
    }
}

fn cycle(grid: &mut [Vec<char>], rounds: &[Point]) -> Vec<Point> {
    let dirs = [Dir::North, Dir::West, Dir::South, Dir::East];
    let mut rounds = rounds.to_vec();

    for d in dirs {
        rounds = tilt(grid, d, &rounds);
    }

    rounds
}

#[allow(clippy::match_on_vec_items)]
fn tilt(grid: &mut [Vec<char>], dir: Dir, rounds: &[Point]) -> Vec<Point> {
    let mut rounds = rounds.to_vec();
    let mut output: Vec<Point> = Vec::new();
    match dir {
        Dir::North => {
            rounds.sort_unstable_by(|&x, &y| x.0.cmp(&y.0));
            rounds.reverse();
        }
        Dir::West => {
            rounds.sort_unstable_by(|&x, &y| x.1.cmp(&y.1));
            rounds.reverse();
        }
        Dir::South => {
            rounds.sort_unstable_by(|&x, &y| x.0.cmp(&y.0));
        }
        Dir::East => {
            rounds.sort_unstable_by(|&x, &y| x.1.cmp(&y.1));
        }
    }
    'o: while let Some(pt) = rounds.pop() {
        match dir {
            Dir::North => {
                for i in (0..pt.0).rev() {
                    match grid[i][pt.1] {
                        '#' | 'O' => {
                            swap(grid, pt, Point(i + 1, pt.1));
                            output.push(Point(i + 1, pt.1));
                            continue 'o;
                        }
                        '.' => (),
                        _ => panic!("Wrong character in grid"),
                    }
                }
                swap(grid, pt, Point(0, pt.1));
                output.push(Point(0, pt.1));
            }
            Dir::West => {
                for i in (0..pt.1).rev() {
                    match grid[pt.0][i] {
                        '#' | 'O' => {
                            swap(grid, pt, Point(pt.0, i + 1));
                            output.push(Point(pt.0, i + 1));
                            continue 'o;
                        }
                        '.' => (),
                        _ => panic!("Wrong character in grid"),
                    }
                }
                swap(grid, pt, Point(pt.0, 0));
                output.push(Point(pt.0, 0));
            }
            Dir::South => {
                for i in pt.0 + 1..grid.len() {
                    match grid[i][pt.1] {
                        '#' | 'O' => {
                            swap(grid, pt, Point(i - 1, pt.1));
                            output.push(Point(i - 1, pt.1));
                            continue 'o;
                        }
                        '.' => (),
                        _ => panic!("Wrong character in grid"),
                    }
                }
                swap(grid, pt, Point(grid.len() - 1, pt.1));
                output.push(Point(grid.len() - 1, pt.1));
            }
            Dir::East => {
                for i in pt.1 + 1..grid[pt.0].len() {
                    match grid[pt.0][i] {
                        '#' | 'O' => {
                            swap(grid, pt, Point(pt.0, i - 1));
                            output.push(Point(pt.0, i - 1));
                            continue 'o;
                        }
                        '.' => (),
                        _ => panic!("Wrong character in grid"),
                    }
                }
                swap(grid, pt, Point(pt.0, grid[pt.0].len() - 1));
                output.push(Point(pt.0, grid[pt.0].len() - 1));
            }
        }
    }

    output
}

fn swap(grid: &mut [Vec<char>], one: Point, two: Point) {
    let temp = grid[one.0][one.1];
    grid[one.0][one.1] = grid[two.0][two.1];
    grid[two.0][two.1] = temp;
}

fn get_load(grid: &[Vec<char>], pt: Point) -> usize {
    let total_len: usize = grid.len();
    let mut num_rounds: usize = 0;

    for i in (0..pt.0).rev() {
        if grid[i][pt.1] == 'O' {
            num_rounds += 1;
        } else if grid[i][pt.1] == '#' {
            return total_len - i - num_rounds - 1;
        }
    }

    total_len - num_rounds
}

fn find_rounds(grid: &[Vec<char>]) -> Vec<Point> {
    let mut output: Vec<Point> = Vec::new();

    for (i, ln) in grid.iter().enumerate() {
        for (j, &c) in ln.iter().enumerate() {
            if c == 'O' {
                output.push(Point(i, j));
            }
        }
    }

    output
}

fn parse_in(p: &Path) -> Vec<Vec<char>> {
    fs::read_to_string(p)
        .expect("File does not exist, fool")
        .lines()
        .map(|x| x.chars().collect())
        .collect()
}
