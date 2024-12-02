use std::fs;
use std::path::Path;

#[derive(Debug)]
struct Point(usize, usize);

fn main() {
    let sum = get_total_distance(Path::new("day11.txt"));

    println!("Total distance: {sum}");
}

fn get_total_distance(p: &Path) -> usize {
    let grid = create_array(p);
    let galaxies = get_galaxies(&grid);
    // println!("Galaxies: {galaxies:?}");
    let rows = find_empty_rows(&grid);
    // println!("Rows: {rows:?}");
    let cols = find_empty_cols(&grid);
    // println!("Rows: {cols:?}");
    let mut sum = 0usize;

    for (idx, g1) in galaxies.iter().enumerate() {
        for g2 in &galaxies[idx + 1..] {
            sum += rows.iter().fold(0, |acc, x| {
                if (g1.0..=g2.0).contains(x) || (g2.0..=g1.0).contains(x) {
                    acc + 1000000 - 1
                } else {
                    acc
                }
            });
            sum += cols.iter().fold(0, |acc, x| {
                if (g1.1..=g2.1).contains(x) || (g2.1..=g1.1).contains(x) {
                    acc + 1000000 - 1
                } else {
                    acc
                }
            });

            sum += g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1);
        }
    }

    sum
}

fn get_galaxies(grid: &[Vec<char>]) -> Vec<Point> {
    let mut out: Vec<Point> = Vec::new();

    for (i, ln) in grid.iter().enumerate() {
        for (j, c) in ln.iter().enumerate() {
            if *c == '#' {
                out.push(Point(i, j));
            }
        }
    }

    out
}

fn find_empty_rows(grid: &[Vec<char>]) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();

    for (idx, row) in grid.iter().enumerate() {
        if row.iter().all(|&x| x == '.') {
            out.push(idx);
        }
    }

    out
}

fn find_empty_cols(grid: &[Vec<char>]) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();
    let mut empty = true;

    for i in 0..grid[0].len() {
        empty = true;
        for row in grid {
            if row[i] != '.' {
                empty = false;
                break;
            }
        }
        if empty {
            out.push(i);
        }
    }

    out
}

fn create_array(p: &Path) -> Vec<Vec<char>> {
    fs::read_to_string(p)
        .expect("File does not exist, fool")
        .lines()
        .map(|x| -> Vec<char> { x.trim().chars().collect() })
        .collect()
}
