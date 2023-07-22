use std::collections::VecDeque;

use nom::{branch::alt, bytes::complete::tag, IResult};

#[derive(PartialEq, Debug)]
enum Instruction {
    Add(isize),
    Noop,
}

pub fn process_part_1(input: &str) -> isize {
    let instructions = parse_instructions(input);
    let mut signal: isize = 1;
    let mut mod_20_signals = Vec::<isize>::new();
    let mut cycle: isize = 1;
    instructions.iter().for_each(|instr| match instr {
        Instruction::Add(val) => {
            add_mod_20_signal(&signal, &mut mod_20_signals, &cycle);
            cycle += 1;
            add_mod_20_signal(&signal, &mut mod_20_signals, &cycle);
            cycle += 1;
            signal += *val;
        }
        Instruction::Noop => {
            add_mod_20_signal(&signal, &mut mod_20_signals, &cycle);
            cycle += 1;
        }
    });
    let total: isize = mod_20_signals.iter().sum();
    total
}

pub fn process_part_2(input: &str) -> String {
    let instructions = parse_instructions(input);
    let mut signal: isize = 1;
    let mut cycle: isize = 1;
    let mut screen = String::new();
    instructions.iter().for_each(|instr| match instr {
        Instruction::Add(val) => {
            add_pixel(&cycle, &signal, &mut screen);
            cycle += 1;
            add_pixel(&cycle, &signal, &mut screen);
            cycle += 1;
            signal += *val;
        }
        Instruction::Noop => {
            add_pixel(&cycle, &signal, &mut screen);
            cycle += 1;
        }
    });
    screen.to_owned()
}

fn add_pixel(cycle: &isize, signal: &isize, screen: &mut String) {
    let cyc = if *cycle > 40 {
        *cycle - 40 * (*cycle / 40)
    } else {
        *cycle
    };
    if (cyc - *signal - 1).abs() <= 1 {
        screen.push('#');
    } else {
        screen.push('.');
    }
    if *cycle % 40 == 0 {
        screen.push('\n');
    }
}

fn add_mod_20_signal(signal: &isize, mod_20_signals: &mut Vec<isize>, cycle: &isize) {
    if (cycle - 20) % 40 != 0 {
        return;
    };
    let signal_str: isize = signal * (*cycle as isize);
    mod_20_signals.push(signal_str);
}

fn parse_instructions(input: &str) -> VecDeque<Instruction> {
    let instructions: VecDeque<Instruction> = input
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
    fn test_process_part_2() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let result = process_part_2(&input);
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
