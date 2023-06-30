use nom::bytes::complete::tag;
use nom::IResult;
use nom::character::complete::alpha1;

use crate::assembler::Token;
use crate::instruction::Opcode;

pub fn opcode_load(input: &str) -> IResult<&str, Token> {
    let (input, opcode) = alpha1(input)?;
    Ok((input, Token::Op { code: Opcode::from(opcode)}))
}

mod tests {
    use super::*;

    #[test]
    fn test_opcode_load() {
        let result = opcode_load("load");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, "");
        //invalid opcode
        let result = opcode_load("aold");
        let(_, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::IGL });
    }
}
