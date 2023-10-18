mod lexer;

#[cfg(test)]
mod lexer_test;

fn main() {
    lexer::lex(&String::from("\"trip\""));
    println!("ran");
}
