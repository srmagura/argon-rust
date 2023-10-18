use crate::lexer::{lex, TokenType};

#[test]
fn lex_string() {
    let a = lex(&String::from("\"foo\""));
    let aa = a.unwrap();

    assert!(matches!(aa[0].token_type, TokenType::String));
    assert_eq!(aa[0].value, String::from("\"foo\""));

    let b = lex(&String::from("\"foo\"\"bar\""));
    let bb = b.unwrap();

    assert!(matches!(bb[0].token_type, TokenType::String));
    assert_eq!(bb[0].value, String::from("\"foo\""));
    assert!(matches!(bb[1].token_type, TokenType::String));
    assert_eq!(bb[1].value, String::from("\"bar\""));
}
