mod parser;
mod token;

fn main() {
    let text: String = "(1+2)*9/(3*4+7)".to_string();
    let lex: Vec<(crate::token::token::Operation, usize)> = crate::token::token::gen_tockens(&text);
    println!("lex = {:?}", lex);
}
