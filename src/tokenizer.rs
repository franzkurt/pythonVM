use std::fmt;
use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    Symbol(Symbol),
    Text(String),
    FString(String),
    LineBreak,
    Space,
    IncrementIdent,
    CarriageReturn,
    Slash,
    IdentationTab,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    False,
    None,
    True,
    And,
    As,
    Assert,
    Break,
    Class,
    Continue,
    Def,
    Del,
    Elif,
    Else,
    Except,
    Finally,
    For,
    From,
    Global,
    If,
    Import,
    In,
    Is,
    Lambda,
    Nonlocal,
    Not,
    Or,
    Pass,
    Raise,
    Return,
    Try,
    While,
    With,
    Yield,
}
impl Keyword {
    fn len(&self) -> usize {
        match self {
            Keyword::False => "False".len(),
            Keyword::None => "None".len(),
            Keyword::True => "True".len(),
            Keyword::And => "and".len(),
            Keyword::As => "as".len(),
            Keyword::Assert => "assert".len(),
            Keyword::Break => "break".len(),
            Keyword::Class => "class".len(),
            Keyword::Continue => "continue".len(),
            Keyword::Def => "def".len(),
            Keyword::Del => "del".len(),
            Keyword::Elif => "elif".len(),
            Keyword::Else => "else".len(),
            Keyword::Except => "except".len(),
            Keyword::Finally => "finally".len(),
            Keyword::For => "for".len(),
            Keyword::From => "from".len(),
            Keyword::Global => "global".len(),
            Keyword::If => "if".len(),
            Keyword::Import => "import".len(),
            Keyword::In => "in".len(),
            Keyword::Is => "is".len(),
            Keyword::Lambda => "lambda".len(),
            Keyword::Nonlocal => "nonlocal".len(),
            Keyword::Not => "not".len(),
            Keyword::Or => "or".len(),
            Keyword::Pass => "pass".len(),
            Keyword::Raise => "raise".len(),
            Keyword::Return => "return".len(),
            Keyword::Try => "try".len(),
            Keyword::While => "while".len(),
            Keyword::With => "with".len(),
            Keyword::Yield => "yield".len(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Symbol {
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftCurlyBrace,
    RightCurlyBrace,
    Comma,
    Dot,
    Colon,
    SemiColon,
    Assign,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Exponent,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    Increment,
    Decrement,
    LogicalAnd,
    LogicalOr,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    ShiftLeft,
    ShiftRight,
    SingleQuote,
    DoubleQuote,
    SendMessage,
    HashTag,
    Dollar,
    AtSign,
    InterrogationMark,
    Crasis,
    UnderLine,
}

impl Symbol {
    fn len(&self) -> usize {
        match self {
            Symbol::LeftParen => 1,
            Symbol::RightParen => 1,
            Symbol::LeftBracket => 1,
            Symbol::RightBracket => 1,
            Symbol::LeftCurlyBrace => 1,
            Symbol::RightCurlyBrace => 1,
            Symbol::Comma => 1,
            Symbol::Dot => 1,
            Symbol::Colon => 1,
            Symbol::SemiColon => 1,
            Symbol::Assign => 1,
            Symbol::Plus => 1,
            Symbol::Minus => 1,
            Symbol::Multiply => 1,
            Symbol::Divide => 1,
            Symbol::Modulo => 1,
            Symbol::Exponent => 1,
            Symbol::LessThan => 1,
            Symbol::GreaterThan => 1,
            Symbol::LessThanOrEqual => 2,
            Symbol::GreaterThanOrEqual => 2,
            Symbol::Equal => 2,
            Symbol::NotEqual => 2,
            Symbol::Increment => 2,
            Symbol::Decrement => 2,
            Symbol::LogicalAnd => 2,
            Symbol::LogicalOr => 2,
            Symbol::BitwiseAnd => 1,
            Symbol::BitwiseOr => 1,
            Symbol::BitwiseXor => 1,
            Symbol::BitwiseNot => 1,
            Symbol::ShiftLeft => 2,
            Symbol::ShiftRight => 2,
            Symbol::SingleQuote => 1,
            Symbol::DoubleQuote => 1,
            Symbol::SendMessage => 1,
            Symbol::HashTag => 1,
            Symbol::Dollar => 1,
            Symbol::AtSign => 1,
            Symbol::InterrogationMark => 1,
            Symbol::Crasis => 1,
            Symbol::UnderLine => 1,
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEndOfInput,
    RecursionLimitExceeded,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken(token) => write!(f, "Unexpected token: {:?}", token),
            ParseError::UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
            ParseError::RecursionLimitExceeded => write!(f, "Recursion limit exceeded"),
        }
    }
}

fn parse_string(input: &str) -> Option<String> {
    match input.chars().nth(0) {
        Some('"') | Some('\'') => {
            let mut literal = String::new();
            let mut iter = input.chars().skip(1);
            while let Some(ch) = iter.next() {
                if ch == '"' || ch == '\'' {
                    break;
                }
                literal.push(ch);
            }
            Some(literal)
        }
        _ => None
    }
}

fn parse_special(input: &str) -> Option<Token> {
    match input.chars().nth(0)?.to_string().as_str() {
        "\n" => Some(Token::LineBreak),
        "\r" => Some(Token::CarriageReturn),
        " " => Some(Token::Space),
        "\\" => Some(Token::Slash),
        "\t" => Some(Token::IdentationTab),
        _ => None,
    }
}

fn parse_symbol(input: &str) -> Option<Symbol> {
    let (first, second) = input.split_at(1);
    // let (first_letter, second): (char, char) = "ab".chars().collect_tuple().unwrap();
    match first {
        "^" => Some(Symbol::BitwiseXor),
        "~" => Some(Symbol::BitwiseNot),
        "'" => Some(Symbol::SingleQuote),
        "!" => Some(Symbol::SendMessage),
        "\"" => Some(Symbol::DoubleQuote),
        "!" => {
            match second {
                "=" => Some(Symbol::NotEqual),
                _ => None,
            }
        }
        "|" => {
            match second {
                "|" => Some(Symbol::LogicalOr),
                _ => Some(Symbol::BitwiseOr),
            }
        }
        "<" => {
            match second {
                "<" => Some(Symbol::ShiftLeft),
                "=" => Some(Symbol::LessThanOrEqual),
                _ => Some(Symbol::LessThan),
            }
        }
        ">" => {
            match second {
                ">" => Some(Symbol::ShiftRight),
                "=" => Some(Symbol::GreaterThanOrEqual),
                _ => Some(Symbol::GreaterThan),
            }
        }
        "&" => {
            match second {
                "&" => Some(Symbol::LogicalAnd),
                _ => Some(Symbol::BitwiseAnd),
            }
        },
        
        "`" => Some(Symbol::Crasis),
        "@" => Some(Symbol::AtSign),
        "?" => Some(Symbol::InterrogationMark),
        "$" => Some(Symbol::Dollar),
        "#" => Some(Symbol::HashTag),
        "(" => Some(Symbol::LeftParen),
        ")" => Some(Symbol::RightParen),
        "[" => Some(Symbol::LeftBracket),
        "]" => Some(Symbol::RightBracket),
        "{" => Some(Symbol::LeftCurlyBrace),
        "}" => Some(Symbol::RightCurlyBrace),
        "," => Some(Symbol::Comma),
        "." => Some(Symbol::Dot),
        ":" => Some(Symbol::Colon),
        ";" => Some(Symbol::SemiColon),
        "=" => {
            match second {
                "=" => Some(Symbol::Equal),
                "+" => Some(Symbol::Increment),
                "-" => Some(Symbol::Decrement),
                _ => Some(Symbol::Assign)
            }
        },
        "+" => Some(Symbol::Plus),
        "-" => Some(Symbol::Minus),
        "_" => Some(Symbol::UnderLine),
        "*" => Some(Symbol::Multiply),
        "/" => Some(Symbol::Divide),
        "%" => Some(Symbol::Modulo),
        "^" => Some(Symbol::Exponent),
        _ => None,
    }
}

fn reverse_symbol(symbol: &Symbol) -> Option<String> {
    match symbol {
        Symbol::Increment => Some("=+".to_string()),
        Symbol::Decrement => Some("=-".to_string()),
        Symbol::Crasis => Some("`".to_string()),
        Symbol::AtSign => Some("@".to_string()),
        Symbol::InterrogationMark => Some("?".to_string()),
        Symbol::Dollar => Some("$".to_string()),
        Symbol::HashTag => Some("#".to_string()),
        Symbol::LeftParen => Some("(".to_string()),
        Symbol::RightParen => Some(")".to_string()),
        Symbol::LeftBracket => Some("[".to_string()),
        Symbol::RightBracket => Some("]".to_string()),
        Symbol::LeftCurlyBrace => Some("{".to_string()),
        Symbol::RightCurlyBrace => Some("}".to_string()),
        Symbol::Comma => Some(",".to_string()),
        Symbol::Dot => Some(".".to_string()),
        Symbol::Colon => Some(":".to_string()),
        Symbol::SemiColon => Some(";".to_string()),
        Symbol::Assign => Some("=".to_string()),
        Symbol::Plus => Some("+".to_string()),
        Symbol::Minus => Some("-".to_string()),
        Symbol::Multiply => Some("*".to_string()),
        Symbol::Divide => Some("/".to_string()),
        Symbol::Modulo => Some("%".to_string()),
        Symbol::Exponent => Some("^".to_string()),
        Symbol::LessThan => Some("<".to_string()),
        Symbol::GreaterThan => Some(">".to_string()),
        Symbol::LessThanOrEqual => Some("<=".to_string()),
        Symbol::GreaterThanOrEqual => Some(">=".to_string()),
        Symbol::Equal => Some("==".to_string()),
        Symbol::NotEqual => Some("!=".to_string()),
        Symbol::LogicalAnd => Some("&&".to_string()),
        Symbol::LogicalOr => Some("||".to_string()),
        Symbol::BitwiseAnd => Some("&".to_string()),
        Symbol::BitwiseOr => Some("|".to_string()),
        Symbol::BitwiseXor => Some("^".to_string()),
        Symbol::BitwiseNot => Some("~".to_string()),
        Symbol::ShiftLeft => Some("<<".to_string()),
        Symbol::ShiftRight => Some(">>".to_string()),
        Symbol::SingleQuote => Some("'".to_string()),
        Symbol::SendMessage => Some("!".to_string()),
        Symbol::DoubleQuote => Some("\"".to_string()),
        Symbol::UnderLine => Some("_".to_string()),
    }
}

fn parse_keyword(input: &str) -> Option<Keyword> {
    match input {
        "False" => Some(Keyword::False),
        "None" => Some(Keyword::None),
        "True" => Some(Keyword::True),
        "and" => Some(Keyword::And),
        "as" => Some(Keyword::As),
        "assert" => Some(Keyword::Assert),
        "break" => Some(Keyword::Break),
        "class" => Some(Keyword::Class),
        "continue" => Some(Keyword::Continue),
        "def" => Some(Keyword::Def),
        "del" => Some(Keyword::Del),
        "elif" => Some(Keyword::Elif),
        "else" => Some(Keyword::Else),
        "except" => Some(Keyword::Except),
        "finally" => Some(Keyword::Finally),
        "for" => Some(Keyword::For),
        "from" => Some(Keyword::From),
        "global" => Some(Keyword::Global),
        "if" => Some(Keyword::If),
        "import" => Some(Keyword::Import),
        "in" => Some(Keyword::In),
        "is" => Some(Keyword::Is),
        "lambda" => Some(Keyword::Lambda),
        "nonlocal" => Some(Keyword::Nonlocal),
        "not" => Some(Keyword::Not),
        "or" => Some(Keyword::Or),
        "pass" => Some(Keyword::Pass),
        "raise" => Some(Keyword::Raise),
        "return" => Some(Keyword::Return),
        "try" => Some(Keyword::Try),
        "while" => Some(Keyword::While),
        "with" => Some(Keyword::With),
        "yield" => Some(Keyword::Yield),
        _ => None,
    }
}

fn reverse_keyword(keyword: &Keyword) -> Option<String> {
    match keyword {
        Keyword::False => Some("False".to_string()),
        Keyword::None => Some("None".to_string()),
        Keyword::True => Some("True".to_string()),
        Keyword::And => Some("and".to_string()),
        Keyword::As => Some("as".to_string()),
        Keyword::Assert => Some("assert".to_string()),
        Keyword::Break => Some("break".to_string()),
        Keyword::Class => Some("class".to_string()),
        Keyword::Continue => Some("continue".to_string()),
        Keyword::Def => Some("def".to_string()),
        Keyword::Del => Some("del".to_string()),
        Keyword::Elif => Some("elif".to_string()),
        Keyword::Else => Some("else".to_string()),
        Keyword::Except => Some("except".to_string()),
        Keyword::Finally => Some("finally".to_string()),
        Keyword::For => Some("for".to_string()),
        Keyword::From => Some("from".to_string()),
        Keyword::Global => Some("global".to_string()),
        Keyword::If => Some("if".to_string()),
        Keyword::Import => Some("import".to_string()),
        Keyword::In => Some("in".to_string()),
        Keyword::Is => Some("is".to_string()),
        Keyword::Lambda => Some("lambda".to_string()),
        Keyword::Nonlocal => Some("nonlocal".to_string()),
        Keyword::Not => Some("not".to_string()),
        Keyword::Or => Some("or".to_string()),
        Keyword::Pass => Some("pass".to_string()),
        Keyword::Raise => Some("raise".to_string()),
        Keyword::Return => Some("return".to_string()),
        Keyword::Try => Some("try".to_string()),
        Keyword::While => Some("while".to_string()),
        Keyword::With => Some("with".to_string()),
        Keyword::Yield => Some("yield".to_string()),
    }
}

pub fn Tokenizer(input: &str) -> Vec<Token> {
    
    let mut input = input;
    let mut tokens = Vec::new();

    let identifier_regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
    let text_regex = Regex::new(r#"^[a-zA-Z0-9_]+"#).unwrap();

    dbg!(&input);
    while !input.is_empty() {
        match input.get(0..4) {
            Some("    ") => {
                tokens.push(Token::IncrementIdent);
                input = &input[4..];
                continue;
            }
            _ => {
                // dbg!(r);
            }
        }

        // symbols can have until 2 signs into syntax
        let twotokens = match input.get(..2) {
            Some(tokens) => tokens,
            None => { input }
        };
        // if format string, ignore tokens and create a fstring token (literal string into AST)
        if let Some(literal_string) = parse_string(&input) {
            dbg!(literal_string.len());
            tokens.push(Token::FString(literal_string.clone()));
            input = &input[literal_string.len()+2..];
        }
        else if let Some(special) = parse_special(&input) {
            tokens.push(special);
            input = &input[1..];
        } 
        else if let Some(symbol) = parse_symbol(&twotokens) {
            tokens.push(Token::Symbol(symbol.clone()));
            input = &input[symbol.len()..];

        } else if let Some(identifier) = identifier_regex.find(input) {
            if let Some(keyword) = parse_keyword(&identifier.as_str().to_string()){
                tokens.push(Token::Keyword(keyword.clone()));
            }
            else {
                tokens.push(Token::Identifier(identifier.as_str().to_string()));
            }
            input = &input[identifier.end()..];
        } else if let Some(rest) = text_regex.find(input){
            // Handle unknown or invalid tokens
            tokens.push(Token::Text(rest.as_str().to_string()));
            // dbg!(&tokens);
            input = &input[rest.end()..];
        }
        else {
            panic!("Invalid token found");
        }
    }
    tokens
}




