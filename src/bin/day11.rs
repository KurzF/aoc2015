use std::{fmt::Display, str::FromStr};

const CONFUSING: &[u8] = &[b'i', b'l', b'o'];

type Password = [u8; 8];

fn main() {
    let input = include_str!("../../input/day11.txt")
        .trim()
        .as_bytes()
        .try_into()
        .unwrap();

    let mut part_one: Password = input;
    while !is_valid_password(&part_one) {
        part_one = next_password(part_one);
    }

    let mut part_two: Password = next_password(part_one);
    while !is_valid_password(&part_two) {
        part_two = next_password(part_two);
    }
    println!(
        "Day 11 part one {} part two {}",
        std::str::from_utf8(&part_one).unwrap(),
        std::str::from_utf8(&part_two).unwrap()
    );
}

fn is_valid_password(password: &Password) -> bool {
    let not_confusing = password.iter().find(|c| CONFUSING.contains(c)).is_none();
    let straight = password
        .windows(3)
        .find(|cs| cs[2].saturating_sub(cs[1]) == 1 && cs[1].saturating_sub(cs[0]) == 1)
        .is_some();

    let pairs: Vec<usize> = password
        .windows(2)
        .enumerate()
        .filter(|(_i, cs)| cs[0] == cs[1])
        .map(|(i, _)| i)
        .collect();

    let pair_count = pairs.windows(2).filter(|is| is[1] - is[0] > 1).count();

    not_confusing && straight && pair_count > 0
}

fn next_password(password: Password) -> Password {
    let v = (password[7] - b'a') as u64
        + (password[6] - b'a') as u64 * 26
        + (password[5] - b'a') as u64 * 26 * 26
        + (password[4] - b'a') as u64 * 26 * 26 * 26
        + (password[3] - b'a') as u64 * 26 * 26 * 26 * 26
        + (password[2] - b'a') as u64 * 26 * 26 * 26 * 26 * 26
        + (password[1] - b'a') as u64 * 26 * 26 * 26 * 26 * 26 * 26
        + (password[0] - b'a') as u64 * 26 * 26 * 26 * 26 * 26 * 26 * 26;

    let next = v + 1;
    let mut password = [0; 8];
    let mut div = 1;
    for i in 0..8 {
        password[7 - i] = (next / div % 26) as u8 + b'a';
        div *= 26;
    }
    password
}
#[cfg(test)]
mod test {
    use crate::{is_valid_password, next_password, Password};

    #[test]
    fn valid_password() {
        // Confusing char
        assert!(!is_valid_password(
            "ijjkkabc".as_bytes().try_into().unwrap()
        ));
        assert!(!is_valid_password(
            "hijklmmn".as_bytes().try_into().unwrap()
        ));

        // Missing straight
        assert!(!is_valid_password(
            "abbceffg".as_bytes().try_into().unwrap()
        ));

        // Miss a pair
        assert!(!is_valid_password(
            "abbcegjk".as_bytes().try_into().unwrap()
        ));

        // Valid
        assert!(is_valid_password("ghjaabcc".as_bytes().try_into().unwrap()));
    }

    #[test]
    fn test_next_password() {
        assert_eq!(
            [b'a', b'a', b'a', b'a', b'a', b'a', b'a', b'b'],
            next_password("aaaaaaaa".as_bytes().try_into().unwrap())
        );
    }
}
