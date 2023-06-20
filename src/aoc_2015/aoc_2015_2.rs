use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

pub fn part_1() -> u32 {
    let file_path = "/Users/hamish.macdonald/Dev/advent_of_code/puzzle_inputs/2015/1_2.txt";
    let mut square_feet_paper: u32 = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(dims) = line {
                let (l, w, h) = get_dims(&dims);
                square_feet_paper += get_required_paper(l, w, h);
            }
        }
    }
    square_feet_paper
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_required_paper(l: u32, w: u32, h: u32) -> u32 {
    2 * l * w + 2 * w * h + 2 * h * l + get_area_smallest_dims(l, w, h)
}

pub fn get_area_smallest_dims(l: u32, w: u32, h: u32) -> u32 {
    let mut v = vec![l, w, h];
    v.sort();
    v[0] * v[1]
}

pub fn get_dims(dims: &str) -> (u32, u32, u32) {
    let mut vec = Vec::new();
    let mut parts = dims.splitn(3, "x");
    if let Some(first) = parts.next() {
        vec.push(first);
    }
    if let Some(second) = parts.next() {
        vec.push(second);
    }
    if let Some(third) = parts.next() {
        vec.push(third);
    }
    let vec = vec
        .iter()
        .map(|s| u32::from_str(s).expect("failed to parse number"))
        .collect::<Vec<_>>();
    let vec: (u32, u32, u32) = (vec[0], vec[1], vec[2]);
    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_paper_correct() {
        let l = 10;
        let w = 2;
        let h = 3;
        let result = get_required_paper(l, w, h);
        assert_eq!(result, 2 * l * w + 2 * w * h + 2 * l * h + w * h);
    }
    #[test]
    fn get_smallest_dims_correct() {
        let l = 10;
        let w = 2;
        let h = 3;
        let result = get_area_smallest_dims(l, w, h);
        assert_eq!(result, w * h);
    }
    #[test]
    fn test_string_to_dims_tuple() {
        let dims = "30x28x5";
        let result = get_dims(dims);
        assert_eq!(result, (30, 28, 5));
    }
}
