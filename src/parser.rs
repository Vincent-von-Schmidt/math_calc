pub mod parser {
    pub struct Node {
        pub left: Box<crate::parser::parser::ParseTree>,
        pub right: Box<crate::parser::parser::ParseTree>,
    }

    pub struct Leaf {
        pub value: i32,
    }

    pub enum ParseTree {
        Leaf(Leaf),
        Node(Node),
    }

    /// generates a ParseTree
    ///
    /// * `text` - text to parse
    pub fn gen_tree(text: &String) -> crate::parser::parser::ParseTree {
        return ParseTree::Leaf { value: 0 };
    }
}
