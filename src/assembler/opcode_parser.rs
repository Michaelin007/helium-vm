use nom::types::CompleteStr;

name!(opcode_load<CompleteStr, Token>,
     do_parse!(
        tag!("load") >> (Token::Op{code: Opcode::LOAD})
     ));

mod tests {
    use super::*;

    #[test]
    fn test_opcode_load() {
        let result = opcode_load(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);
    }

}