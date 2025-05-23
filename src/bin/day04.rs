fn main() {
    let part_one = zero_hash("bgvyzdsv", 0xf0);
    let part_two = zero_hash("bgvyzdsv", 0xff);
    println!("Day 04 part one: {part_one}, part two: {part_two}");
}

fn zero_hash(input: &str, mask: u8) -> u32 {
    let mut answer = 0;
    loop {
        let hash = md5::compute(format!("{}{}", input, answer));
        if hash.0[0] == 0 && hash.0[1] == 0 && (hash.0[2] & mask == 0) {
            break;
        }
        answer += 1;
    }

    answer
}

#[cfg(test)]
mod test {
    use crate::zero_hash;

    #[test]
    fn example() {
        assert_eq!(609043, zero_hash("abcdef", 0xf0));
        assert_eq!(1048970, zero_hash("pqrstuv", 0xf0));
    }
}
