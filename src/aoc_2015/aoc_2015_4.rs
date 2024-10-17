use hex;
use md5::{Digest, Md5};

pub fn part_1(start_loop_int: u32) -> u32 {
    let input = "yzbqklnj";
    let lowest_number = find_lowest_number(input, "00000", start_loop_int);
    lowest_number
}

pub fn part_2(start_loop_int: u32) -> u32 {
    let input = "yzbqklnj";
    let lowest_number = find_lowest_number(input, "000000", start_loop_int);
    lowest_number
}

fn find_lowest_number(input: &str, zero_string: &str, start_loop_int: u32) -> u32 {
    let mut i = start_loop_int;
    loop {
        let test = get_md5_hash(&format!("{}{}", input, i));
        if test.starts_with(zero_string) {
            break;
        }
        i += 1;
    }
    i
}

fn get_md5_hash(input: &str) -> String {
    let mut sh = Md5::new();
    sh.update(input);
    hex::encode(sh.finalize().to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_md5_hash() {
        let input = "yzbqklnj282749";
        let output = get_md5_hash(input);
        assert_eq!(output, "000002c655df7738246e88f6c1c43eb7");
    }
}
