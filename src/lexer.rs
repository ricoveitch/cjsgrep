#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Asterisk,
    CloseBraces,
    CloseParen,
    Comma,
    Dot,
    Equals,
    OpenBraces,
    OpenParen,
    Newline,
    Whitespace,
    ForwardSlash,
    EOF,
    String(String),
    CatchAll(String),
    Identifier(String),
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            TokenType::Asterisk => "*",
            TokenType::CloseBraces => "}",
            TokenType::CloseParen => ")",
            TokenType::Comma => ",",
            TokenType::Dot => ".",
            TokenType::EOF => "EOF",
            TokenType::Equals => "=",
            TokenType::Newline => "\n",
            TokenType::OpenBraces => "{",
            TokenType::OpenParen => "(",
            TokenType::Whitespace => " ",
            TokenType::ForwardSlash => "/",
            TokenType::Identifier(s) => return write!(f, "{}", s),
            TokenType::String(s) => return write!(f, "{}", s),
            TokenType::CatchAll(s) => s.as_str(),
        };

        write!(f, "{}", s)
    }
}

pub fn bytes_to_string(bytes: Vec<u8>) -> String {
    String::from_utf8_lossy(&bytes).to_string()
}

fn is_whitespace(byte: u8) -> bool {
    match byte {
        b' ' | b'\t' | b'\r' => true,
        _ => false,
    }
}

#[derive(Clone)]
pub struct Cursor {
    pos: usize,
    pub line_num: usize,
    prev: TokenType,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            pos: 0,
            line_num: 0,
            prev: TokenType::Whitespace,
        }
    }
}

pub struct Lexer {
    src: Vec<u8>,
    pub cursor: Cursor,
}

impl Lexer {
    pub fn new(src: &str) -> Lexer {
        Lexer {
            src: src.as_bytes().to_owned(),
            cursor: Cursor::default(),
        }
    }

    fn peak_byte(&self, distance: usize) -> Option<&u8> {
        self.src.get(self.cursor.pos + distance)
    }

    fn read_while(&self, mut pred: impl FnMut(&u8) -> bool, offset: usize) -> (Vec<u8>, usize) {
        let mut bytes = vec![];
        for byte in self.src.iter().skip(self.cursor.pos + offset) {
            if pred(byte) {
                bytes.push(byte.clone())
            } else {
                break;
            }
        }
        let bytes_read = bytes.len();
        (bytes, bytes_read)
    }

    fn read_identifier(&self) -> (TokenType, usize) {
        let (bytes, bytes_read) = self.read_while(|b| b.is_ascii_alphanumeric() || *b == b'_', 0);

        (TokenType::Identifier(bytes_to_string(bytes)), bytes_read)
    }

    fn read_string(&self, quote: u8) -> (TokenType, usize) {
        let (s_bytes, inner_bytes_read) = self.read_while(|b| *b != quote, 1);
        let s = bytes_to_string(s_bytes);
        let bytes_read = inner_bytes_read + 2;

        (TokenType::String(s), bytes_read)
    }

    fn read_whitespace(&self) -> (TokenType, usize) {
        let (_, bytes_read) = self.read_while(|b| is_whitespace(*b), 0);
        (TokenType::Whitespace, bytes_read)
    }

    fn read_catch_all(&self, byte: u8) -> (TokenType, usize) {
        let s = match String::from_utf8(vec![byte]) {
            Ok(s) => s,
            Err(_) => panic!("invalid character {}", byte as char),
        };

        (TokenType::CatchAll(s), 1)
    }

    fn peak(&self) -> (TokenType, usize) {
        let byte = match self.peak_byte(0) {
            Some(b) => b,
            None => return (TokenType::EOF, 0),
        };

        match byte {
            b'*' => (TokenType::Asterisk, 1),
            b'/' => (TokenType::ForwardSlash, 1),
            b',' => (TokenType::Comma, 1),
            b'.' => (TokenType::Dot, 1),
            b'(' => (TokenType::OpenParen, 1),
            b')' => (TokenType::CloseParen, 1),
            b'{' => (TokenType::OpenBraces, 1),
            b'}' => (TokenType::CloseBraces, 1),
            b'=' => (TokenType::Equals, 1),
            b'\n' => (TokenType::Newline, 1),
            b if *b == b'"' || *b == b'\'' => self.read_string(*b),
            b if is_whitespace(*b) => self.read_whitespace(),
            b if b.is_ascii_alphabetic() => self.read_identifier(),
            _ => self.read_catch_all(*byte),
        }
    }

    pub fn next_token(&mut self) -> TokenType {
        loop {
            if self.cursor.prev == TokenType::Newline {
                self.cursor.line_num += 1;
            }

            let (token, bytes_read) = self.peak();
            self.cursor.pos += bytes_read;
            self.cursor.prev = token.clone();

            if token != TokenType::Whitespace {
                return token;
            }
        }
    }

    pub fn lookahead(&mut self, distance: usize) -> TokenType {
        let mut i = distance as u32;
        let cursor_snapshot = self.cursor.clone();

        loop {
            let token = self.next_token();
            if token == TokenType::Whitespace {
                continue;
            }

            i -= 1;
            if i <= 0 || token == TokenType::EOF {
                self.cursor = cursor_snapshot;
                return token;
            }
        }
    }
}
