use std::fmt;

#[derive(Copy)]
pub enum ErrorToken {
    UnterminatedString,
    UnexpectedCharacter(char),

    StringUnderflow,
    StringOverflow,
    NumberUnderflow,
    NumberOverflow,
    IdentifierUnderflow,
    IdentifierOverflow,
    CommentUnderflow,
    CommentOverflow,
}

impl Clone for ErrorToken {
    fn clone(&self) -> ErrorToken { *self }
}

impl fmt::Display for ErrorToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorToken::UnterminatedString => write!(f, "unterminated string"),
            ErrorToken::UnexpectedCharacter(c) => write!(f, "unexpected character: '{}'", c),

            ErrorToken::StringUnderflow => write!(f, "string underflow"),
            ErrorToken::StringOverflow => write!(f, "string overflow"),
            ErrorToken::NumberUnderflow => write!(f, "number underflow"),
            ErrorToken::NumberOverflow => write!(f, "number overflow"),
            ErrorToken::IdentifierUnderflow => write!(f, "identifier underflow"),
            ErrorToken::IdentifierOverflow => write!(f, "identifier overflow"),
            ErrorToken::CommentUnderflow => write!(f, "comment underflow"),
            ErrorToken::CommentOverflow => write!(f, "comment overflow"),
        }
    }
}