#[derive(Debug, PartialEq)]
pub enum Opcode {
    NOP,
    LOAD,
    ADD,
    SUB,
    DIV,
    MUL,
    PRT,
    JMP,
    JMPF,
    JMPB,
    EQ,
    NEQ,
    GT,
    LT,
    GTQ,
    LTQ,
    JEQ,
    JNEQ,
    HALT,
    IGL,
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::NOP,
            1 => Opcode::LOAD,
            2 => Opcode::ADD,
            4 => Opcode::SUB,
            5 => Opcode::MUL,
            6 => Opcode::DIV,
            7 => Opcode::JMP,
            8 => Opcode::JMPF,
            9 => Opcode::JMPB,
            10 => Opcode::EQ,
            11 => Opcode::NEQ,
            12 => Opcode::GT,
            13 => Opcode::LT,
            14 => Opcode::GTQ,
            15 => Opcode::LTQ,
            16 => Opcode::JEQ,
            17 => Opcode::JNEQ,
            200 => Opcode::PRT,
            255 => Opcode::HALT,
            _ => Opcode::IGL,
        }
    }
}

pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction {
            opcode
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HALT;
        assert_eq!(opcode, Opcode::HALT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HALT);
        assert_eq!(instruction.opcode, Opcode::HALT);
    }
}