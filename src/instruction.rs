#[derive(Debug, PartialEq, Copy, Clone)]
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
        Instruction { opcode }
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => return Opcode::LOAD,
            1 => return Opcode::HLT,
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

impl <'a> From<&'a str> for Opcode {
    fn from(value: &'a str) -> Self {
        match value {
            "load" => Opcode::LOAD,
            "add" => Opcode::ADD,
            "sub" => Opcode::SUB,
            "mul" => Opcode::MUL,
            "div" => Opcode::DIV,
            "hlt" => Opcode::HLT,
            "jmp" => Opcode::JMP,
            "jmpf" => Opcode::JMPF,
           
            "eq" => Opcode::EQ,
            "neq" => Opcode::NEQ,
         
            "gt" => Opcode::GT,
          
            "lt" => Opcode::LT,
           
            _ => Opcode::IGL,
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

    #[test]
    fn test_str_to_opcode() {
        let opcode = Opcode::from("load");
        assert_eq!(opcode, Opcode::LOAD);
        let opcode = Opcode::from("illegal");
        assert_eq!(opcode, Opcode::IGL
        )
    }
}
