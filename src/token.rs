mod tocken {
    use std::usize;

    // math operations
    pub enum Operation {
        PLUS,
        MINUS,
        TIMES,
        DIVIDE,
        POWER,
        OPEN,
        CLOSE
    }

    /// generates a vector of operation and index
    ///
    /// * `text` - text to check
    pub fn gen_tockens(text: &String) -> Vec<(Operation, usize)> {
        let mut output: Vec<(Operation, usize)> = Vec::new();
        for (index, chr) in text.chars().enumerate() {
            match chr {
                '+' => output.push((Operation::PLUS, index)),
                '-' => output.push((Operation::MINUS, index)),
                '*' => output.push((Operation::TIMES, index)),
                '/' => output.push((Operation::DIVIDE, index)),
                '^' => output.push((Operation::POWER, index)),
                _ => continue,
            }
        }
        return output;
    }
}
