mod token;

fn main() {
    let text: String = "1+2/3".to_string();
    let lex: Vec<(crate::token::token::Operation, usize)> = crate::token::token::gen_tockens(&text);
    println!("lex = {:?}", lex);
}
