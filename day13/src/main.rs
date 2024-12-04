use std::{fs, iter::zip, path::Path};

fn main() {
    let grids = parse_in(Path::new("day13.txt"));
    let mut sum: usize = 0;

    for g in grids {
        sum += find_mirror(&g);
    }

    println!("Sum: {sum}");
}

fn find_mirror(grid: &[Vec<char>]) -> usize {
    let mut err: usize = 0;
    'o: for idx in 1..grid.len() {
        err = 0;
        for (i, j) in zip(grid[..idx].iter().rev(), grid[idx..].iter()) {
            for (&ic, &jc) in zip(i, j) {
                if err > 1 {
                    continue 'o;
                }
                if ic != jc {
                    err += 1;
                }
            }
        }
        if err == 1 {
            return 100 * idx;
        }
    }
    'o: for idx in 1..grid[0].len() {
        err = 0;
        for (i, j) in zip((0..idx).rev(), idx..grid[0].len()) {
            for jdx in 0..grid.len() {
                if err > 1 {
                    continue 'o;
                }
                if grid[jdx][i] != grid[jdx][j] {
                    err += 1;
                }
            }
        }
        if err == 1 {
            return idx;
        }
    }

    panic!("No mirrors found");
}

fn parse_in(p: &Path) -> Vec<Vec<Vec<char>>> {
    let mut output: Vec<Vec<Vec<char>>> = Vec::new();
    let mut cur: Vec<Vec<char>> = Vec::new();
    let file = fs::read_to_string(p).expect("File does not exist, fool");

    for ln in file.lines() {
        if ln.is_empty() {
            output.push(cur);
            cur = Vec::new();
        } else {
            cur.push(ln.chars().collect());
        }
    }
    output.push(cur);

    output
}
