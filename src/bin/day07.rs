use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/day07.txt").trim();
    let instructions: Vec<_> = input.lines().map(parse_inst).collect();

    let mut cache = HashMap::new();

    let part_one = compute("a", &instructions, &mut cache);
    cache.clear();

    cache.insert("b", part_one);
    let part_two = compute("a", &instructions, &mut cache);
    println!("Day 07 part one: {}, part_two: {}", part_one, part_two);
}

#[derive(Debug, Clone, Copy)]
struct Instruction<'a> {
    identifier: &'a str,
    opcode: OpCode<'a>,
}

#[derive(Debug, Clone, Copy)]
enum OpCode<'a> {
    Assign(&'a str),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Not(&'a str),
    LShift(&'a str, u16),
    RShift(&'a str, u16),
}

const AND: &str = "AND";
const OR: &str = "OR";
const NOT: &str = "NOT";
const LSHIFT: &str = "LSHIFT";
const RSHIFT: &str = "RSHIFT";
const ARROW: &str = "->";

fn parse_inst(input: &str) -> Instruction {
    let mut words = input.split_ascii_whitespace();
    let (first, second) = (words.next().unwrap(), words.next().unwrap());

    if first == NOT {
        let _arrow = words.next();
        return Instruction {
            identifier: words.next().unwrap(),
            opcode: OpCode::Not(second),
        };
    }

    if second == ARROW {
        return Instruction {
            identifier: words.next().unwrap(),
            opcode: OpCode::Assign(first),
        };
    }

    match second {
        AND => {
            let third = words.next().unwrap();
            let _arrow = words.next();
            Instruction {
                identifier: words.next().unwrap(),
                opcode: OpCode::And(first, third),
            }
        }
        OR => {
            let third = words.next().unwrap();
            let _arrow = words.next();
            Instruction {
                identifier: words.next().unwrap(),
                opcode: OpCode::Or(first, third),
            }
        }
        LSHIFT => {
            let third = words.next().unwrap().parse().unwrap();
            let _arrow = words.next();
            Instruction {
                identifier: words.next().unwrap(),
                opcode: OpCode::LShift(first, third),
            }
        }
        RSHIFT => {
            let third = words.next().unwrap().parse().unwrap();
            let _arrow = words.next();
            Instruction {
                identifier: words.next().unwrap(),
                opcode: OpCode::RShift(first, third),
            }
        }
        _ => unreachable!(),
    }
}

fn compute<'a>(
    key: &'a str,
    circuit: &'a [Instruction<'a>],
    cache: &mut HashMap<&'a str, u16>,
) -> u16 {
    if let Some(value) = cache.get(key) {
        return *value;
    }

    if key.chars().next().unwrap().is_ascii_digit() {
        return key.parse().unwrap();
    }
    let instruction = circuit.iter().find(|inst| inst.identifier == key).unwrap();
    let result = match instruction.opcode {
        OpCode::Assign(id) => compute(id, circuit, cache),
        OpCode::And(rhs, lhs) => compute(rhs, circuit, cache) & compute(lhs, circuit, cache),
        OpCode::Or(rhs, lhs) => compute(rhs, circuit, cache) | compute(lhs, circuit, cache),
        OpCode::Not(v) => !compute(v, circuit, cache),
        OpCode::LShift(rhs, lhs) => compute(rhs, circuit, cache) << lhs,
        OpCode::RShift(rhs, lhs) => compute(rhs, circuit, cache) >> lhs,
    };

    cache.insert(key, result);
    result
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{compute, parse_inst};

    #[test]
    fn example() {
        let input = r#"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"#;

        let instructions: Vec<_> = input.lines().map(parse_inst).collect();
        let mut cache = HashMap::new();
        assert_eq!(72, compute("d", &instructions, &mut cache));
        assert_eq!(507, compute("e", &instructions, &mut cache));
        assert_eq!(492, compute("f", &instructions, &mut cache));
        assert_eq!(114, compute("g", &instructions, &mut cache));
        assert_eq!(65412, compute("h", &instructions, &mut cache));
        assert_eq!(65079, compute("i", &instructions, &mut cache));
        assert_eq!(123, compute("x", &instructions, &mut cache));
        assert_eq!(456, compute("y", &instructions, &mut cache));
    }
}
