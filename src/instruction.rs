#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,
    IGL,
    LOAD,
    ADD,
    SUB,
    MUL,
    JMP,
    JMPF,
    DIV,
    EQ,
    NEQ,
    GT,
    LT,
    GTQ,
    LTQ,
    JEQ,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode: opcode }
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => return Opcode::HLT,
            1 => return Opcode::LOAD,
            2 => return Opcode::ADD,
            3 => return Opcode::SUB,
            4 => return Opcode::MUL,
            5 => return Opcode::DIV,
            6 => return Opcode::EQ,
            7 => return Opcode::NEQ,
            8 => return Opcode::GT,
            9 => return Opcode::LT,
            10 => return Opcode::JEQ,
            11 => return Opcode::JMP,
            12 => return Opcode::JMPF,
            13 => return Opcode::GTQ,
            14 => return Opcode::LTQ,

            _ => return Opcode::IGL,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
