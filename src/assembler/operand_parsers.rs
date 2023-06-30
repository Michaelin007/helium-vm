use crate::assembler::Token;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::sequence::tuple;
use nom::IResult;

pub fn integer_operand(input: &str) -> IResult<&str, Token> {
    let (input, (_, reg_num, _)) = tuple((tag("#"), digit1, multispace0))(input)?;
    let reg_num = reg_num.parse::<i32>().unwrap();
    Ok((input, Token::IntegerOperand { value: reg_num }))
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_integer_operand() {
        let result = integer_operand("#10");
        assert_eq!(result.is_ok(), true);
        let (rest, value) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(value, Token::IntegerOperand { value: 10 });

        //let test an invalid input
        let result = integer_operand("10");
        assert_eq!(result.is_ok(), false);
    }
}
