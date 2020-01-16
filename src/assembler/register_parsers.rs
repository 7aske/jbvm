use crate::assembler::opcode::Token;
use nom::number::complete::be_u8;

use nom::bytes::complete::tag;
named!(pub register <&[u8], Token>,
    ws!(
        do_parse!(
            tag!("$") >>
            reg_num: be_u8 >>
            (
                Token::Register{
                  reg: reg_num - 48
                }
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_register() {
        let result = register(b"$0");
        assert_eq!(result.is_ok(), true);
        let result = register(b"1");
        assert_eq!(result.is_ok(), false);
        let result = register(b"$a");
        assert_eq!(result.is_ok(), true);
    }
}