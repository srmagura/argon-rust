enum Token {
    String(String),
}

fn lex_string(input: &String, i: i32) {
    let j = i;

    if input[j] != '"' {
        return;
    }

    j += 1;

    for c in input.chars() {
        if (c == '"') {
            println!("Got end quote");
        }
    }

    return;
}

fn lex(input: &String) {
    lex_string(input);
}
