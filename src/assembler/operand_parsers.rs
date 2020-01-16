use crate::assembler::opcode::Token;

use nom::number::complete::be_i32;
use nom::bytes::complete::tag;

// Parser for integer numbers, which we preface with `#` in our assembly language:
// #100
named!(pub integer_operand<&[u8], Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            operand: be_i32 >>
            (
                Token::IntegerOperand{value: operand}
            )
        )
    )
);

#[test]
fn test_parse_integer_operand() {
    let result = integer_operand(b"#10");
    assert_eq!(result.is_ok(), true);
    let (rest, value) = result.unwrap();
    assert_eq!(rest, "".as_bytes());
    assert_eq!(value, Token::IntegerOperand { value: 10 });

    let result = integer_operand(b"10");
    assert_eq!(result.is_ok(), false);
}