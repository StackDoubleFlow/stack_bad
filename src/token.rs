pub enum TokenData {
    Stack([u32; 5]),
    Bad([u32; 3]),
}

pub struct Token {
    pub line: usize,
    pub col: usize,
    pub data: TokenData,
}
