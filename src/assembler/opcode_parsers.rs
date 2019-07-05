use crate::assembler::opcode::Token;
use crate::instruction::Opcode;

named!(opcode_load<&str, Token>,
  do_parse!(tag!("load") >> (Token::Op{code: Opcode::LOAD}))
);


mod tests {
    use super::*;

    #[test]
    fn test_opcode_load() {
        let result = opcode_load("load");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, "");

        let result = opcode_load("aold");
        assert_eq!(result.is_ok(), false);
    }
}