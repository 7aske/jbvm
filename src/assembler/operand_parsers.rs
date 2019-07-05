use crate::assembler::opcode::Token;

use nom::number::complete::be_i32;

/// Parser for integer numbers, which we preface with `#` in our assembly language:
/// #100
named!(integer_operand<&[u8], Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            reg_num: be_i32 >>
            (
                Token::IntegerOperand{value: reg_num}
            )
        )
    )
);

#[test]
fn test_parse_integer_operand() {
    let result = integer_operand("#10".as_bytes());
    assert_eq!(result.is_ok(), true);
    let (rest, value) = result.unwrap();
    assert_eq!(rest, "".as_bytes());
    assert_eq!(value, Token::IntegerOperand { value: 10 });

    let result = integer_operand("10".as_bytes());
    assert_eq!(result.is_ok(), false);
}