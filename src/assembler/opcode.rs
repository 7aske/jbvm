use crate::instruction::Opcode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Opcode { code: Opcode },
    Register { reg: u8 },
    IntegerOperand { value: i32 },
}
