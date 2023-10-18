use crate::lexer;

#[test]
fn lex_string() {
    lexer::lex(&String::from("\"foo\""));
}
