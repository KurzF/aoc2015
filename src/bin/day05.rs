use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/day05.txt");
    let part_one = input
        .lines()
        .filter(|s| has_three_vowels(s) && has_double_char(s) && !has_forbidden_pattern(s))
        .count();

    let part_two = input
        .lines()
        .filter(|s| has_repeat_pair(s) && has_repeat_letter(s))
        .count();

    println!("Day 05 part one: {}, part_two: {}", part_one, part_two);
}

fn has_three_vowels(input: &str) -> bool {
    input.chars().filter(|c| "aeiou".contains(*c)).count() >= 3
}

fn has_double_char(input: &str) -> bool {
    input.as_bytes().windows(2).find(|w| w[0] == w[1]).is_some()
}

const FORBIDDEN_PATTERN: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn has_forbidden_pattern(input: &str) -> bool {
    for pattern in FORBIDDEN_PATTERN {
        if input.contains(pattern) {
            return true;
        }
    }

    false
}

fn count_letters(input: &str) -> [u8; 26] {
    let mut count = [0; 26];
    for c in input.as_bytes() {
        let i = c - b'a';
        count[i as usize] += 1;
    }
    count
}

fn has_repeat_pair(input: &str) -> bool {
    let count = count_letters(input);

    // Find all pair where both character are present multiple time in input
    let possible_pair: Vec<_> = input
        .as_bytes()
        .windows(2)
        .enumerate()
        .filter(|(_i, c)| {
            let i1 = c[0] - b'a';
            let i2 = c[1] - b'a';
            count[i1 as usize] > 1 && count[i2 as usize] > 1
        })
        .collect();

    // To eliminate pair with overlap (ex: "aaa") we take the min and max index of matching pair
    // If max - min > 1 pair are not overlapping
    // TODO: replace this hashmap with a 26^2 array ?
    let mut pair_extremum = HashMap::<[u8; 2], (usize, usize)>::new();

    for pair in &possible_pair {
        let key = [pair.1[0], pair.1[1]];
        let (mut min, mut max) = pair_extremum.entry(key).or_insert((256, 0));

        min = std::cmp::min(min, pair.0);
        max = std::cmp::max(max, pair.0);

        pair_extremum.insert(key, (min, max));
    }

    pair_extremum
        .values()
        .find(|(min, max)| max - min > 1)
        .is_some()
}

fn has_repeat_letter(input: &str) -> bool {
    input.as_bytes().windows(3).find(|c| c[0] == c[2]).is_some()
}

#[cfg(test)]
mod test {
    use crate::{
        has_double_char, has_forbidden_pattern, has_repeat_letter, has_repeat_pair,
        has_three_vowels,
    };

    #[test]
    fn example_has_three_vowels() {
        assert!(has_three_vowels("aei"));
        assert!(has_three_vowels("xazegov"));
        assert!(has_three_vowels("aeiouaeiouaeiou"));
    }

    #[test]
    fn example_has_double_char() {
        assert!(has_double_char("xx"));
        assert!(has_double_char("abcdde"));
        assert!(has_double_char("aabbccdd"));
    }

    #[test]
    fn example_contain_forbidden_pattern() {
        assert!(has_forbidden_pattern("dhaegwjzuvuyypxyu"))
    }

    #[test]
    fn example_has_repeat_pair() {
        assert!(has_repeat_pair("xyxy"));
        assert!(has_repeat_pair("aabcdefgaa"));
        assert!(has_repeat_pair("qjhvhtzxzqqjkmpb"));
        assert!(has_repeat_pair("xxyxx"));
        assert!(has_repeat_pair("aaaa"));
        assert!(!has_repeat_pair("aaa"));
        assert!(!has_repeat_pair("ieodomkazucvgmuy"));
    }

    #[test]
    fn example_has_repeat_letter() {
        assert!(has_repeat_letter("xyx"));
        assert!(has_repeat_letter("aaa"));
        assert!(has_repeat_letter("abcdefeghi"));
        assert!(has_repeat_letter("qjhvhtzxzqqjkmpb"));
        assert!(has_repeat_letter("xxyxx"));
        assert!(!has_repeat_letter("uurcxstgmygtbstg"));
    }
}
