use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

use crate::assembler::Token;
use crate::instruction::Opcode;

pub fn register(input: &str) -> IResult<&str, Token> {
    let (input, (_, reg_num, _)) = tuple((tag("$"), digit1, multispace0))(input)?;
    let reg_num = reg_num.parse::<u8>().unwrap();
    Ok((input, Token::Register { reg_num }))
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register("$0");
        assert_eq!(result.is_ok(), true);
        let result = register("0");
        assert_eq!(result.is_ok(), false);
        let result = register("$a");
        assert_eq!(result.is_ok(), false);
    }
}
