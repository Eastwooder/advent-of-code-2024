#![feature(result_flattening)]

use crate::SingleInstructionResult::{Continue, Output};
use itertools::Itertools;
use SingleInstructionResult::IncreaseInstructionPtr;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let (mut computer, program) = parse_input(input);
    Some(
        execute_instructions(&mut computer, &program)
            .iter()
            .join(","),
    )
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

fn execute_instructions(computer: &mut Computer, program: &[Byte]) -> Vec<Byte> {
    let mut output = vec![];
    while let Ok(Some(print)) = computer.interpret_program(program) {
        output.push(print);
    }
    output
}

#[allow(dead_code)]
fn ro_execute_instructions(computer: Computer, program: &[Byte]) -> (Computer, Vec<Byte>) {
    let mut output = vec![];
    let mut computer = computer;
    while let Ok((next, res)) = computer.ro_interpret_program(program) {
        computer = next;
        if let Some(byte) = res {
            output.push(byte);
        }
    }
    (computer, output)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Instr {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug)]
struct Trap;

impl TryFrom<Byte> for Instr {
    type Error = Trap;

    fn try_from(value: Byte) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Instr::Adv),
            1 => Ok(Instr::Bxl),
            2 => Ok(Instr::Bst),
            3 => Ok(Instr::Jnz),
            4 => Ok(Instr::Bxc),
            5 => Ok(Instr::Out),
            6 => Ok(Instr::Bdv),
            7 => Ok(Instr::Cdv),
            _ => Err(Trap),
        }
    }
}

type Register = u64;
type Pointer = usize;
type Byte = u64;
type Program = Vec<Byte>;

#[derive(Debug, Copy, Clone)]
struct Computer {
    a: Register,
    b: Register,
    c: Register,
    pointer: Pointer,
}

impl Computer {
    #[allow(dead_code)]
    fn new(a: Register, b: Register, c: Register) -> Self {
        Computer {
            a,
            b,
            c,
            pointer: 0,
        }
    }

    fn current(&self, program: &[Byte]) -> Result<Instr, Trap> {
        program
            .get(self.pointer)
            .ok_or(Trap)
            .and_then(|&v| v.try_into())
    }

    fn literal(&self, program: &[Byte]) -> Result<Byte, Trap> {
        program.get(self.pointer + 1).copied().ok_or(Trap)
    }

    fn combo(&self, program: &[Byte]) -> Result<Register, Trap> {
        match program[self.pointer + 1] {
            nop @ 0..=3 => Ok(nop as Register),
            4 => Ok(self.a),
            5 => Ok(self.b),
            6 => Ok(self.c),
            _ => Err(Trap),
        }
    }

    fn ro_interpret_program(&self, program: &[Byte]) -> Result<(Computer, Option<Byte>), Trap> {
        let mut computer = *self;
        while computer.pointer < program.len() {
            let (next, result) = computer.ro_single_instruction(program)?;
            computer = next;
            if result.is_some() {
                return Ok((computer, result));
            }
        }
        Ok((computer, None))
    }

    #[inline]
    fn ro_single_instruction(&self, program: &[Byte]) -> Result<(Computer, Option<Byte>), Trap> {
        const MOD: Register = 8;
        let (a, b, c, pointer) = match self.current(program)? {
            Instr::Adv => (
                self.a >> self.combo(program)?,
                self.b,
                self.c,
                self.pointer + 2,
            ),
            Instr::Bxl => (
                self.a,
                self.b ^ self.literal(program)?,
                self.c,
                self.pointer + 2,
            ),
            Instr::Bst => (self.a, self.combo(program)? % MOD, self.c, self.pointer + 2),
            Instr::Jnz if self.a != 0 => (self.a, self.b, self.c, self.literal(program)? as usize),
            Instr::Jnz => (self.a, self.b, self.c, self.pointer + 2),
            Instr::Bxc => (self.a, self.b ^ self.c, self.c, self.pointer + 2),
            Instr::Out => {
                let out = self.combo(program)? % MOD;
                let (a, b, c, pointer) = (self.a, self.b, self.c, self.pointer + 2);
                return Ok((Computer { a, b, c, pointer }, Some(out)));
            }
            Instr::Bdv => (
                self.a,
                self.a >> self.combo(program)?,
                self.c,
                self.pointer + 2,
            ),
            Instr::Cdv => (
                self.a,
                self.b,
                self.a >> self.combo(program)?,
                self.pointer + 2,
            ),
        };
        Ok((Computer { a, b, c, pointer }, None))
    }

    fn interpret_program(&mut self, program: &[Byte]) -> Result<Option<Byte>, Trap> {
        while self.pointer < program.len() {
            match self.single_instruction(program)? {
                Output(byte) => {
                    self.pointer += 2;
                    return Ok(Some(byte));
                }
                IncreaseInstructionPtr => self.pointer += 2,
                Continue => {}
            }
        }
        Ok(None)
    }

    #[inline]
    fn single_instruction(&mut self, program: &[Byte]) -> Result<SingleInstructionResult, Trap> {
        const MOD: Register = 8;
        match self.current(program)? {
            Instr::Adv => self.a >>= self.combo(program)?,
            Instr::Bxl => self.b ^= self.literal(program)?,
            Instr::Bst => self.b = self.combo(program)? % MOD,
            Instr::Jnz if self.a != 0 => {
                self.pointer = self.literal(program)? as usize;
                return Ok(Continue);
            }
            Instr::Jnz => {}
            Instr::Bxc => self.b ^= self.c,
            Instr::Out => {
                let out = self.combo(program)? % MOD;
                return Ok(Output(out));
            }
            Instr::Bdv => self.b = self.a >> self.combo(program)?,
            Instr::Cdv => self.c = self.a >> self.combo(program)?,
        }
        Ok(IncreaseInstructionPtr)
    }
}

enum SingleInstructionResult {
    Output(Byte),
    Continue,
    IncreaseInstructionPtr,
}

fn parse_input(input: &str) -> (Computer, Program) {
    let mut iter = input.lines();
    let a = iter
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let b = iter
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let c = iter
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    iter.next().unwrap(); // empty line
    let band = iter
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|b| b.parse().unwrap())
        .collect();
    let pointer = 0;

    (Computer { a, b, c, pointer }, band)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[rstest]
    #[case((0, 0, 9), vec![2, 6], (0, 1, 9), vec![])]
    #[case((10, 0, 0), vec![5,0,5,1,5,4], (10, 0, 0), vec![0,1,2])]
    #[case((2024, 0, 0), vec![0,1,5,4,3,0], (0, 0, 0), vec![4,2,5,6,7,7,7,7,3,1,0])]
    #[case((0, 29, 0), vec![1,7], (0, 26, 0), vec![])]
    #[case((0, 2024, 43690), vec![4,0], (0, 44354, 43690), vec![])]
    fn test_computer_ops(
        #[case] (a, b, c): (Register, Register, Register),
        #[case] instructions: Program,
        #[case] expected: (Register, Register, Register),
        #[case] expected_output: Vec<Byte>,
    ) {
        let mut comp = Computer::new(a, b, c);
        let output = execute_instructions(&mut comp, &instructions);
        assert_eq!(expected, (comp.a, comp.b, comp.c));
        assert_eq!(expected_output, output);
    }

    // #[rstest]
    // #[case((0, 0, 9), vec![2, 6], (0, 1, 9), vec![])]
    // #[case((10, 0, 0), vec![5,0,5,1,5,4], (10, 0, 0), vec![0,1,2])]
    // #[case((2024, 0, 0), vec![0,1,5,4,3,0], (0, 0, 0), vec![4,2,5,6,7,7,7,7,3,1,0])]
    // #[case((0, 29, 0), vec![1,7], (0, 26, 0), vec![])]
    // #[case((0, 2024, 43690), vec![4,0], (0, 44354, 43690), vec![])]
    // fn test_computer_ops_ro(#[case] (a, b, c): (Register, Register, Register), #[case] instructions: Program, #[case] expected: (Register, Register, Register), #[case] expected_output: Vec<Byte>) {
    //     let comp = Computer::new(a, b, c);
    //     let (comp, output) = ro_execute_instructions(comp, &instructions);
    //     assert_eq!(expected, (comp.a, comp.b, comp.c));
    //     assert_eq!(expected_output, output);
    // }
}
