use crate::assembler::instruction_parsers::{instruction_one, AssemblerInstruction};
use crate::instruction;
use nom::multi::many1;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instructions {
            program.append(&mut instruction.to_bytes());
        }
        program
    }
}

pub fn program(input: &str) -> IResult<&str, Program> {
    let (input, instructions) = many1(instruction_one)(input)?;
    Ok((input, Program { instructions }))
}

#[test]
fn test_parse_program() {
    let result = program("load $0 #100\n");
    assert_eq!(result.is_ok(), true);
    let (leftover, p) = result.unwrap();
    assert_eq!(leftover, "");
    assert_eq!(1, p.instructions.len());
}

#[test]
fn test_program_to_bytes() {
    let result = program("load $10 #100\n");
    assert_eq!(result.is_ok(), true);
    let (_, program) = result.unwrap();
    let bytecode = program.to_bytes();
    assert_eq!(bytecode.len(), 4);
    println!("{:?}", bytecode);
}
