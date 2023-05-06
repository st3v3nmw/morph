#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

#[test]
fn grammar() {
    assert!(grammar::TermParser::new().parse("22").is_ok());
    assert!(grammar::TermParser::new().parse("(22)").is_ok());
    assert!(grammar::TermParser::new().parse("((((22))))").is_ok());
    assert!(grammar::TermParser::new().parse("((22)").is_err());
}
