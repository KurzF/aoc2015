use regex::Regex;

const INIT_PASSWORD: u64 = 20151125;

fn main() {
    let input = include_str!("../../input/day25.txt");
    let re = Regex::new(r#"To continue, please consult the code grid in the manual.  Enter the code at row (?<row>\d+), column (?<column>\d+)."#).unwrap();
    let caps = re.captures(input).unwrap();

    let row: u32 = caps["row"].parse().unwrap();

    let column: u32 = caps["column"].parse().unwrap();

    let part_one = password_by_index(INIT_PASSWORD, row - 1, column - 1);

    println!("Day 25 part one: {}", part_one);
}

fn index_of(row: u32, column: u32) -> u32 {
    (row + column) * (row + column + 1) / 2 + column
}

fn password_generation(prev: u64) -> u64 {
    prev * 252533 % 33554393
}

fn password_by_index(init: u64, row: u32, column: u32) -> u64 {
    let mut password = init;
    for _ in 0..index_of(row, column) {
        password = password_generation(password);
    }
    password
}

#[cfg(test)]
mod test {
    use crate::{index_of, password_by_index, password_generation, INIT_PASSWORD};

    #[test]
    fn index_zero() {
        assert_eq!(0, index_of(0, 0));
    }

    #[test]
    fn index_row_zero() {
        assert_eq!(5, index_of(0, 2));
    }

    #[test]
    fn index_column_zero() {
        assert_eq!(10, index_of(4, 0));
    }

    #[test]
    fn password_generation_test() {
        assert_eq!(31916031, password_generation(INIT_PASSWORD));
    }

    #[test]
    fn example() {
        assert_eq!(20151125, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(31916031, password_by_index(INIT_PASSWORD, 1, 0));
        assert_eq!(18749137, password_by_index(INIT_PASSWORD, 0, 1));
        assert_eq!(1728984, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(3094333, password_by_index(INIT_PASSWORD, 1, 1));
        assert_eq!(1007177, password_by_index(INIT_PASSWORD, 0, 2));
        assert_eq!(33511524, password_by_index(INIT_PASSWORD, 3, 0));
        assert_eq!(3191603, password_by_index(INIT_PASSWORD, 2, 1));
        assert_eq!(2162979, password_by_index(INIT_PASSWORD, 1, 2));
        assert_eq!(16929656, password_by_index(INIT_PASSWORD, 0, 3));
        assert_eq!(772664, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(15514188, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(4041754, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(16080970, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(805725, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(160113, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(798124, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(1166186, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(16474243, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(2459265, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(3245196, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(21345942, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(938009, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(1060067, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(31527494, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(77061, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(1755225, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(28094349, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(689965, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(925075, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(31663883, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(33071741, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(679674, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(2539745, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(24659492, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(153492, password_by_index(INIT_PASSWORD, 0, 0));
        assert_eq!(27995004, password_by_index(INIT_PASSWORD, 0, 0));
    }
}
