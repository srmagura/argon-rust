enum Token {
    String(String),
}

fn lex_string(input: &String, i: i32) {
    let mut chars = input.chars();
    let mut j = i;

    let q = match chars.next() {
        None => 0,
        ['"'] => 1,
    };

    // j += 1;

    // for c in input.chars() {
    //     if (c == '"') {
    //         println!("Got end quote");
    //     }
    // }

    // return;
}

fn lex(input: &String) {
    lex_string(input);
}
