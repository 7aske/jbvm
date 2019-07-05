use crate::assembler::opcode::Token;
use nom::number::complete::be_u8;
named!(register <&[u8], Token>,
    ws!(
        do_parse!(
            tag!("$") >>
            reg_num: be_u8>>
            (
                Token::Register{
                  reg: reg_num
                }
            )
        )
    )
);

#[test]
fn test_parse_register() {
    let result = register("$0".as_bytes());
    assert_eq!(result.is_ok(), true);
    let result = register("0".as_bytes());
    assert_eq!(result.is_ok(), false);
    let result = register("$a".as_bytes());
    assert_eq!(result.is_ok(), false);
}