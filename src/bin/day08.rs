fn main() {
    let input = include_str!("../../input/day08.txt");
    let part_one: usize = input.lines().map(|l| unescaped_len_diff(l)).sum();
    let part_two: usize = input.lines().map(|l| escaped_len_diff(l)).sum();
    println!("Day 08 part one: {}, part two: {}", part_one, part_two);
}

fn unescaped_len_diff(input: &str) -> usize {
    let mut chars = input.chars();
    let mut diff = 0;
    while let Some(c) = chars.next() {
        if c == '\\' {
            println!("\\");
            let n = chars.next().unwrap();
            diff += if n == 'x' { 3 } else { 1 }
        }
    }
    diff + 2
}

fn escaped_len_diff(input: &str) -> usize {
    let mut chars = input.chars();
    let mut diff = 0;
    while let Some(c) = chars.next() {
        if ['\\', '\"'].contains(&c) {
            println!("\\");
            diff += 1;
        }
    }
    diff + 2
}

#[cfg(test)]
mod test {
    use crate::unescaped_len_diff;

    #[test]
    fn remove_main_quote() {
        assert_eq!(2, unescaped_len_diff(r#""""#));
        assert_eq!(2, unescaped_len_diff(r#""abc""#));
    }

    #[test]
    fn remove_hexa_quote() {
        assert_eq!(5, unescaped_len_diff(r#""\x27""#));
    }

    #[test]
    fn real_test() {
        assert_eq!(
            6,
            unescaped_len_diff(r#""\"ihjqlhtwbuy\"hdkiv\"mtiqacnf\\""#)
        );
    }

    #[test]
    fn example() {
        let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;
        let total_diff: usize = input.lines().map(|l| unescaped_len_diff(l)).sum();
        assert_eq!(12, total_diff);
    }
}
