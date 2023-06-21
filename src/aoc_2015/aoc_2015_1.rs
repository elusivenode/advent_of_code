use std::fs;

pub fn part_1() -> i32 {
    let file_path = "/Users/hamish.macdonald/Dev/advent_of_code/puzzle_inputs/2015/1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut floor: i32 = 0;
    for c in contents.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => floor += 0,
        }
    }
    floor
}

pub fn part_2() -> u32 {
    let file_path = "/Users/hamish.macdonald/Dev/advent_of_code/puzzle_inputs/2015/1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut floor: i32 = 0;
    let mut idx: u32 = 0;
    for c in contents.chars() {
        idx += 1;
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => floor += 0,
        }
        if floor < 0 {
            break;
        }
    }
    idx
}
