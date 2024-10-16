use md5::{self, Digest};
pub fn part_1() -> u32 {
    let secret_key = "yzbqklnj";
    let lowest_number = find_lowest_number(&secret_key);
    lowest_number
}

fn get_md5_hash(input: &str) -> Digest {
    md5::compute(input.as_bytes())
}

fn get_first_5_nibbles_hex(digest: &Digest) -> String {
    let mut hex_string = String::new();
    let nibbles = [
        digest[0] >> 4,
        digest[0] & 0x0F,
        digest[1] >> 4,
        digest[1] & 0x0F,
        digest[2] >> 4,
    ];
    for nibble in &nibbles {
        hex_string.push_str(&format!("{:x}", nibble));
    }
    hex_string
}

fn find_lowest_number(secret_key: &str) -> u32 {
    let mut number = 0;
    loop {
        let digest = get_md5_hash(format!("{}{}", secret_key, number).as_str());
        if get_first_5_nibbles_hex(&digest) == "00000" {
            break;
        }
        number += 1;
    }
    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_lowest_number() {
        let secret_key = "abcdef";
        let lowest_number = find_lowest_number(&secret_key);
        assert_eq!(
            lowest_number,
            "609043".parse::<u32>().unwrap(),
            "Lowest does not match expected"
        );
    }

    #[test]
    fn test_get_first_5_nibbles_hex() {
        let secret_key = "abcdef";
        let digest = get_md5_hash(format!("{}{}", secret_key, 609043).as_str());
        let hex_string = get_first_5_nibbles_hex(&digest);
        assert_eq!(
            hex_string, "00000",
            "The first 5 nibbles hex string is incorrect"
        );
    }

    #[test]
    fn test_first_5_nibbles_zero() {
        let secret_key = "abcdef";
        let digest = get_md5_hash(format!("{}{}", secret_key, 609043).as_str());

        let first_5_nibbles = (digest[0] >> 4) == 0
            && (digest[0] & 0x0F) == 0
            && (digest[1] >> 4) == 0
            && (digest[1] & 0x0F) == 0
            && (digest[2] >> 4) == 0;
        assert!(first_5_nibbles, "The first 5 nibbles are not zero");
    }
}
