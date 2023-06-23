use std::collections::HashMap;
use std::fs;

pub fn part_1() -> u32 {
    let mut houses_visited: HashMap<(i32, i32), u32> = HashMap::new();
    let file_path = "/Users/hamish.macdonald/Dev/advent_of_code/puzzle_inputs/2015/3.txt";
    let contents = get_instructions(file_path);
    let mut curr_coords: Option<(i32, i32)> = Some((0, 0));
    match curr_coords {
        Some(coords) => {
            houses_visited.insert(coords, 1);
        }
        None => (),
    }

    for c in contents.chars() {
        let new_coords = get_next_coordinates(c, curr_coords);
        match new_coords {
            Some(coords) => {
                if let Some(count) = houses_visited.get(&coords) {
                    houses_visited.insert(coords, count + 1);
                } else {
                    houses_visited.insert(coords, 1);
                };
                curr_coords = new_coords;
            }
            None => (),
        }
    }
    let no_houses_visited: u32 = houses_visited.keys().len() as u32;
    no_houses_visited
}

pub fn part_2() -> u32 {
    let mut houses_visited: HashMap<(i32, i32), u32> = HashMap::new();
    let file_path = "/Users/hamish.macdonald/Dev/advent_of_code/puzzle_inputs/2015/3.txt";
    let contents = get_instructions(file_path);
    let (instr_santa, instr_robo_santa) = split_instructions(&contents);

    let mut curr_coords: Option<(i32, i32)> = Some((0, 0));
    match curr_coords {
        Some(coords) => {
            houses_visited.insert(coords, 1);
        }
        None => (),
    }
    for c in instr_santa.chars() {
        let new_coords = get_next_coordinates(c, curr_coords);
        match new_coords {
            Some(coords) => {
                if let Some(count) = houses_visited.get(&coords) {
                    houses_visited.insert(coords, count + 1);
                } else {
                    houses_visited.insert(coords, 1);
                };
                curr_coords = new_coords;
            }
            None => (),
        }
    }

    let mut curr_coords: Option<(i32, i32)> = Some((0, 0));
    let count = houses_visited.get(&curr_coords.unwrap()).unwrap();
    houses_visited.insert(curr_coords.unwrap(), count + 1);
    for c in instr_robo_santa.chars() {
        let new_coords = get_next_coordinates(c, curr_coords);
        match new_coords {
            Some(coords) => {
                if let Some(count) = houses_visited.get(&coords) {
                    houses_visited.insert(coords, count + 1);
                } else {
                    houses_visited.insert(coords, 1);
                };
                curr_coords = new_coords;
            }
            None => (),
        }
    }
    let no_houses_visited: u32 = houses_visited.keys().len() as u32;
    no_houses_visited
}

pub fn get_instructions(fp: &str) -> String {
    let contents = fs::read_to_string(fp).expect("Should have been able to read the file");
    contents
}

pub fn split_instructions(instructions: &str) -> (String, String) {
    let mut instructions_1 = String::new();
    let mut instructions_2 = String::new();

    let mut idx = 1;
    for c in instructions.chars() {
        if idx % 2 != 0 {
            instructions_1.push(c);
        } else {
            instructions_2.push(c);
        }
        idx += 1;
    }
    (instructions_1, instructions_2)
}

pub fn get_next_coordinates(mv: char, current_coords: Option<(i32, i32)>) -> Option<(i32, i32)> {
    match current_coords {
        None => return Some((0, 0)),
        Some((x, y)) => match mv {
            '^' => return Some((x, y + 1)),
            '>' => return Some((x + 1, y)),
            'v' => return Some((x, y - 1)),
            '<' => return Some((x - 1, y)),
            _ => return None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_first_coordinates() {
        let mv = '>';
        let curr_coords: Option<(i32, i32)> = None;
        let next_coords = get_next_coordinates(mv, curr_coords);
        assert_eq!(next_coords, Some((0, 0)));
    }
    #[test]
    fn test_mv_up() {
        let mv = '^';
        let curr_coords: Option<(i32, i32)> = Some((0, 0));
        let next_coords = get_next_coordinates(mv, curr_coords);
        assert_eq!(next_coords, Some((0, 1)));
    }
    #[test]
    fn test_mv_right() {
        let mv = '>';
        let curr_coords: Option<(i32, i32)> = Some((0, 100));
        let next_coords = get_next_coordinates(mv, curr_coords);
        assert_eq!(next_coords, Some((1, 100)));
    }
    #[test]
    fn test_mv_down() {
        let mv = 'v';
        let curr_coords: Option<(i32, i32)> = Some((-2, -2));
        let next_coords = get_next_coordinates(mv, curr_coords);
        assert_eq!(next_coords, Some((-2, -3)));
    }
    #[test]
    fn test_mv_left() {
        let mv = '<';
        let curr_coords: Option<(i32, i32)> = Some((-2, -2));
        let next_coords = get_next_coordinates(mv, curr_coords);
        assert_eq!(next_coords, Some((-3, -2)));
    }
    #[test]
    fn test_bad_mv() {
        let mv = 'b';
        let curr_coords: Option<(i32, i32)> = Some((-2, -2));
        let next_coords = get_next_coordinates(mv, curr_coords);
        assert_eq!(next_coords, None);
    }
    #[test]
    fn test_split_instructions() {
        let contents = String::from("^^vv><");
        let (instr_santa, instr_robo_santa) = split_instructions(&contents);
        assert_eq!(instr_santa, String::from("^v>"));
        assert_eq!(instr_robo_santa, String::from("^v<"));
    }
}
