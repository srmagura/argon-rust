pub enum TokenType {
    String,
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

fn lex_string(input: &str) -> Option<Token> {
    let mut chars = input.chars();
    let mut len = 0;

    match chars.next() {
        None => return None,
        Some(_c) => (),
    };

    len += 1;

    for c in chars {
        len += 1;

        if c == '"' {
            return Some(Token {
                token_type: TokenType::String,
                value: String::from(&input[0..len]),
            });
        }
    }

    // We got to the end without finding a closing quote
    return None;
}

pub fn lex(input: &str) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut i = 0;

    let chars: Vec<char> = input.chars().collect();

    while i < input.len() {
        let next_char = chars[i];

        if next_char == ' ' || next_char == '\t' {
            i += 1;
            continue;
        }

        match lex_string(&input[i..]) {
            None => (),
            Some(token) => {
                i += token.value.len();
                tokens.push(token);
            }
        }
    }

    if i == input.len() {
        return Some(tokens);
    }

    return None;
}
