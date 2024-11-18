use std::fs;
use std::iter::zip;
use std::path::Path;

fn main() {
    let total: usize = full_solve_together(Path::new("day6.txt"));

    println!("Total: {total}");
}

fn full_solve(p: &Path) -> usize {
    let file = fs::read_to_string(p).expect("File does not exist, fool");
    let mut lines = file.lines();
    let times = read_info(lines.next().expect("File is empty"));
    let dists = read_info(lines.next().expect("File has too few lines"));
    let mut output: usize = 1;

    for (t, d) in zip(times, dists) {
        output *= count_times(t, d);
    }

    output
}

fn full_solve_together(p: &Path) -> usize {
    let file = fs::read_to_string(p).expect("File does not exist, fool");
    let mut lines = file.lines();
    let time = read_info_together(lines.next().expect("File is empty"));
    let dist = read_info_together(lines.next().expect("File has too few lines"));

    count_times(time, dist)
}

fn read_info(s: &str) -> Vec<usize> {
    s.split(':')
        .last()
        .expect("Entry empty")
        .trim()
        .split(' ')
        .filter_map(|x| x.parse().ok())
        .collect()
}

fn read_info_together(s: &str) -> usize {
    let mut output: usize = 0;

    for i in s.split(':').last().expect("Entry empty").chars() {
        if let Some(end) = i.to_digit(10) {
            append_int(&mut output, end as usize);
        }
    }

    output
}

fn append_int(num: &mut usize, end: usize) {
    *num = *num * 10 + end;
}

fn count_times(t: usize, d: usize) -> usize {
    let mut count: usize = 0;

    for i in 0..t {
        if (t - i) * i > d {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse1() {
        assert_eq!(read_info("Time:      7  15   30"), vec![7, 15, 30]);
    }

    #[test]
    fn parse2() {
        assert_eq!(read_info("Distance:  9  40  200"), vec![9, 40, 200]);
    }

    #[test]
    fn parse_together1() {
        assert_eq!(read_info_together("Time:      7  15   30"), 71530);
    }

    #[test]
    fn parse_together2() {
        assert_eq!(read_info_together("Distance:  9  40  200"), 940200);
    }

    #[test]
    fn count_together() {
        assert_eq!(count_times(71530, 940200), 71503);
    }

    #[test]
    fn count1() {
        assert_eq!(count_times(7, 9), 4);
    }

    #[test]
    fn count2() {
        assert_eq!(count_times(15, 40), 8);
    }

    #[test]
    fn count3() {
        assert_eq!(count_times(30, 200), 9);
    }

    #[test]
    fn full() {
        assert_eq!(full_solve(Path::new("test.txt")), 288);
    }

    #[test]
    fn full_together() {
        assert_eq!(full_solve_together(Path::new("test.txt")), 71503);
    }
}
