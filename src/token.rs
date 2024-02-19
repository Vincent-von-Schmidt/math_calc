pub mod tocken {
    use std::usize;

    #[derive(Debug)]
    pub enum Operation {
        PLUS,
        MINUS,
        TIMES,
        DIVIDE,
        POWER,
        OPEN,
        CLOSE,
    }

    /// generates a vector of operation and index
    ///
    /// * `text` - text to check
    pub fn gen_tockens(text: &String) -> Vec<(Operation, usize)> {
        let mut output: Vec<(Operation, usize)> = Vec::new();
        for (index, chr) in text.chars().enumerate() {
            let token: Operation = match chr {
                '+' => Operation::PLUS,
                '-' => Operation::MINUS,
                '*' => Operation::TIMES,
                '/' => Operation::DIVIDE,
                '^' => Operation::POWER,
                '(' => Operation::OPEN,
                ')' => Operation::CLOSE,
                _ => continue,
            };

            output.push((token, index));
        }
        return output;
    }
}
