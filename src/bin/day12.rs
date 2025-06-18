use std::{ops::RangeBounds, str::Bytes};

fn main() {
    let input = include_str!("../../input/day12.txt");
    let part_one: i32 = NumberFinder::new(input).sum();

    println!("Day 12 part_one {}", part_one);
}

pub struct NumberFinder<'a> {
    haystack: Bytes<'a>,
}

impl<'a> NumberFinder<'a> {
    fn new(haystack: &'a str) -> Self {
        Self {
            haystack: haystack.bytes(),
        }
    }
}

impl<'a> Iterator for NumberFinder<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut negative = false;
        let mut value: i32 = 0;
        loop {
            let b = self.haystack.next()?;
            if b == b'-' {
                negative = true;
                break;
            }
            let n = b.wrapping_sub(b'0');
            if n < 10 {
                value = n as i32;
                break;
            }
        }

        while let Some(b) = self.haystack.next() {
            let n = b.wrapping_sub(b'0');
            if n > 10 {
                break;
            }
            value = value * 10 + n as i32;
        }

        Some(if negative { -value } else { value })
    }
}
