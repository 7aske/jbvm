use crate::assembler::opcode::Token;
use crate::instruction::Opcode;
use nom::IResult;
use nom::bytes::complete::tag;
//
//named!(pub opcode_load<&[u8], Token>,
//  do_parse!(tag!("load") >> (Token::Opcode{code: Opcode::LOAD}))
//);

pub fn opcode_load(i: &str) -> IResult<&str, &str> {
    tag("abcd")(i)
}

mod tests {
    use super::*;

    #[test]
    fn test_opcode_load() {
        let result = opcode_load("load");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
//        assert_eq!(token, Token::Opcode { code: Opcode::LOAD });
        assert_eq!(rest, "");

        let result = opcode_load("aold");
        assert_eq!(result.is_ok(), false);
    }
}