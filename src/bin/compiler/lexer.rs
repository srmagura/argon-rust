enum Token<'a> {
    String(&'a str),
}

fn lex_string<'a>(input: &'a str) -> Option<Token<'a>> {
    let mut chars = input.chars();

    match chars.next() {
        None => return None,
        Some(_c) => (),
    };

    for c in chars {
        if c == '"' {
            return Some(Token::String(&input[0..1]));
        }
    }

    // We got to the end without finding a closing quote
    return None;
}

pub fn lex(input: &str) {
    lex_string(input);
}
