#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    Identifier, Strings, Number,

    And, Class, Else, False, Fun, For, If, Nils, Or,
    Print, Return, Super, This, True, Var, While,

    Eofs,
}