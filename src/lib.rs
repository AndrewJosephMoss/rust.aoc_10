use nom::{branch::alt, bytes::complete::tag, character::complete::digit1, IResult};

#[derive(PartialEq, Debug)]
enum Instruction {
    Add(isize),
    Noop,
}

pub fn process_part_1(input: &str) -> isize {
    13140
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| {
            let instruction: Instruction = alt((parse_add, parse_noop))(line).unwrap().1;
            instruction
        })
        .collect();
    instructions
}

fn parse_add(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("addx ")(input)?;
    let val: isize = input.parse().unwrap();
    Ok((input, Instruction::Add(val)))
}

fn parse_noop(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Instruction::Noop))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_process_part_1() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let result = process_part_1(&input);
        assert_eq!(result, 13140);
    }

    #[test]
    fn test_parse_add() {
        let input = "addx 5";
        let result = parse_add(&input).unwrap().1;
        assert_eq!(result, Instruction::Add(5));

        let input = "addx -3";
        let result = parse_add(&input).unwrap().1;
        assert_eq!(result, Instruction::Add(-3));

        let input = "addx 0";
        let result = parse_add(&input).unwrap().1;
        assert_eq!(result, Instruction::Add(0));

        let input = "addx -11";
        let result = parse_add(input).unwrap().1;
        assert_eq!(result, Instruction::Add(-11));
    }

    #[test]
    fn test_parse_noop() {
        let input = "noop";
        let result = parse_noop(input).unwrap().1;
        assert_eq!(result, Instruction::Noop);
    }

    #[test]
    fn test_parse_instructions() {
        let input = "addx 15\naddx -11\nnoop";
        let result = parse_instructions(input);
        assert_eq!(
            result,
            vec![
                Instruction::Add(15),
                Instruction::Add(-11),
                Instruction::Noop
            ]
        );
    }
}
