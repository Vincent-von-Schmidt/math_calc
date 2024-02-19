mod token;

fn main() {
    let text: String = "1+2/3".to_string();
    let lex: Vec<(crate::token::tocken::Operation, usize)> =
        crate::token::tocken::gen_tockens(&text);
    println!("lex = {:?}", lex);
}
