use crate::assembler::instruction_parsers::{AssemblerInstruction, instruction_one};

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>
}

named!(pub program<&[u8], Program>,
    do_parse!(
        instructions: many1!(instruction_one) >>
        (
            Program {
                instructions: instructions
            }
        )
    )
);

#[test]
fn test_parse_program() {
    let result = program("load $0 #100\n".as_bytes());
    assert_eq!(result.is_ok(), true);
    let (leftover, p) = result.unwrap();
    assert_eq!(leftover, "".as_bytes());
    assert_eq!(
        1,
        p.instructions.len()
    );
}