#[derive(Debug)]
pub enum TokenKind {
    Name,
    From,
    Type(TypeKind),
    Alter,
    Comma,
    Drop,
    Table,
    AutoIncrement,
    Database,
    Asterisk,
    CloseParen,
    Not,
    Key,
    Primary,
    OpenParen,
    Create,
    Colon,
    Dot,
    Equals,
    Minus,
    Plus,
    Semicolon,
    Newline,
}

#[derive(Debug)]
pub enum TypeKind {
    Numberic,
    Integer,
    Null,
    Real,
    Varchar,
    Text,
    Blob,
    Bool,
}

#[derive(Debug)]
pub struct Token {
    token: TokenKind,
    token_str: Option<String>,
}

impl Token {
    pub fn to_mysql(items: Vec<Token>) -> String {
        // TODO: Implement right types
        let mut buff = String::new();
        for i in items.into_iter() {
            match i.token {
                TokenKind::Type(TypeKind::Integer) => buff.push_str("INTEGER "),
                TokenKind::Type(TypeKind::Numberic) => buff.push_str("INTEGER "),
                TokenKind::Type(TypeKind::Null) => buff.push_str("NULL "),
                TokenKind::Type(TypeKind::Real) => buff.push_str("REAL "),
                TokenKind::Type(TypeKind::Text) => buff.push_str("TEXT "),
                TokenKind::Type(TypeKind::Blob) => buff.push_str("BYITEA "),
                TokenKind::Type(TypeKind::Bool) => buff.push_str("BOOLEAN "),
                TokenKind::Type(TypeKind::Varchar) => buff.push_str("TEXT "),

                TokenKind::From => buff.push_str("FROM "),
                TokenKind::AutoIncrement => buff.push_str("AUTO_INCREMENT "),
                TokenKind::Drop => buff.push_str("DROP "),
                TokenKind::Database => buff.push_str("DATABASE "),
                TokenKind::Alter => buff.push_str("ALTER "),
                TokenKind::Create => buff.push_str("CREATE "),
                TokenKind::Table => buff.push_str("TABLE "),
                TokenKind::Newline => buff.push_str(" \n "),
                TokenKind::Semicolon => buff.push_str(";"),
                TokenKind::Asterisk => buff.push_str("* "),
                TokenKind::OpenParen => buff.push_str("( "),
                TokenKind::CloseParen => buff.push_str(") "),
                TokenKind::Comma => buff.push_str(", "),
                TokenKind::Name => buff.push_str(format!("{} ", i.token_str.unwrap()).as_str()),
                TokenKind::Not => buff.push_str("NOT "),
                TokenKind::Key => buff.push_str("KEY "),
                TokenKind::Primary => buff.push_str("PRIMARY "),
                TokenKind::Colon => buff.push_str(": "),
                TokenKind::Dot => buff.push_str(". "),
                TokenKind::Equals => buff.push_str("= "),
                TokenKind::Plus => buff.push_str("+ "),
                TokenKind::Minus => buff.push_str("- "),
            }
        }
        buff
    }
    pub fn to_sqlite(items: Vec<Token>) -> String {
        // TODO: Implement right types
        let mut buff = String::new();
        for i in items.into_iter() {
            match i.token {
                TokenKind::Type(TypeKind::Integer) => buff.push_str("INTEGER "),
                TokenKind::Type(TypeKind::Numberic) => buff.push_str("INTEGER "),
                TokenKind::Type(TypeKind::Null) => buff.push_str("NULL "),
                TokenKind::Database => buff.push_str("DATABASE "),
                TokenKind::Type(TypeKind::Real) => buff.push_str("REAL "),
                TokenKind::Type(TypeKind::Text) => buff.push_str("TEXT "),
                TokenKind::Type(TypeKind::Blob) => buff.push_str("BYITEA "),
                TokenKind::Type(TypeKind::Bool) => buff.push_str("BOOLEAN "),
                TokenKind::Type(TypeKind::Varchar) => buff.push_str("TEXT "),

                TokenKind::Drop => buff.push_str("DROP "),
                TokenKind::AutoIncrement => buff.push_str("AUTOINCREMENT "),
                TokenKind::From => buff.push_str("FROM "),
                TokenKind::Alter => buff.push_str("ALTER "),
                TokenKind::Create => buff.push_str("CREATE "),
                TokenKind::Table => buff.push_str("TABLE "),
                TokenKind::Newline => buff.push_str(" \n "),
                TokenKind::Semicolon => buff.push_str(";"),
                TokenKind::Asterisk => buff.push_str("* "),
                TokenKind::OpenParen => buff.push_str("( "),
                TokenKind::CloseParen => buff.push_str(") "),
                TokenKind::Comma => buff.push_str(", "),
                TokenKind::Name => buff.push_str(format!("{} ", i.token_str.unwrap()).as_str()),
                TokenKind::Not => buff.push_str("NOT "),
                TokenKind::Key => buff.push_str("KEY "),
                TokenKind::Primary => buff.push_str("PRIMARY "),
                TokenKind::Colon => buff.push_str(": "),
                TokenKind::Dot => buff.push_str(". "),
                TokenKind::Equals => buff.push_str("= "),
                TokenKind::Plus => buff.push_str("+ "),
                TokenKind::Minus => buff.push_str("- "),
            }
        }
        buff
    }
    pub fn to_postgresql(items: Vec<Token>) -> String {
        let mut buff = String::new();
        for i in items.into_iter() {
            match i.token {
                TokenKind::Type(TypeKind::Integer) => buff.push_str("INTEGER "),
                TokenKind::Type(TypeKind::Numberic) => buff.push_str("INTEGER "),
                TokenKind::Type(TypeKind::Null) => buff.push_str("NULL "),
                TokenKind::Type(TypeKind::Real) => buff.push_str("REAL "),
                TokenKind::Type(TypeKind::Text) => buff.push_str("TEXT "),
                TokenKind::Type(TypeKind::Blob) => buff.push_str("BYITEA "),
                TokenKind::Type(TypeKind::Bool) => buff.push_str("BOOLEAN "),
                TokenKind::Type(TypeKind::Varchar) => buff.push_str("TEXT "),
                TokenKind::From => buff.push_str("FROM "),
                TokenKind::AutoIncrement => buff.push_str("SERIAL "),
                TokenKind::Drop => buff.push_str("DROP "),
                TokenKind::Alter => buff.push_str("ALTER "),
                TokenKind::Database => buff.push_str("DATABASE "),
                TokenKind::Create => buff.push_str("CREATE "),
                TokenKind::Table => buff.push_str("TABLE "),
                TokenKind::Newline => buff.push_str(" \n "),
                TokenKind::Semicolon => buff.push_str(";"),
                TokenKind::Asterisk => buff.push_str("* "),
                TokenKind::OpenParen => buff.push_str("( "),
                TokenKind::CloseParen => buff.push_str(") "),
                TokenKind::Comma => buff.push_str(", "),
                TokenKind::Name => buff.push_str(format!("{} ", i.token_str.unwrap()).as_str()),
                TokenKind::Not => buff.push_str("NOT "),
                TokenKind::Key => buff.push_str("KEY "),
                TokenKind::Primary => buff.push_str("PRIMARY "),
                TokenKind::Colon => buff.push_str(": "),
                TokenKind::Dot => buff.push_str(". "),
                TokenKind::Equals => buff.push_str("= "),
                TokenKind::Plus => buff.push_str("+ "),
                TokenKind::Minus => buff.push_str("- "),
            }
        }
        buff
    }
}

impl TokenKind {
    pub fn tokenize(word: String) -> Token {
        match word.to_lowercase().as_str() {
            // TODO: Make it so any sql-like languages are parsed
            "drop" => Token {
                token: TokenKind::Drop,
                token_str: None,
            },
            "database" => Token {
                token: TokenKind::Database,
                token_str: None,
            },
            "alter" => Token {
                token: TokenKind::Alter,
                token_str: None,
            },
            "from" => Token {
                token: TokenKind::From,
                token_str: None,
            },
            "\n" | "\n\r" => Token {
                token: TokenKind::Newline,
                token_str: None,
            },
            "primary" => Token {
                token: TokenKind::Primary,
                token_str: None,
            },
            "key" => Token {
                token: TokenKind::Key,
                token_str: None,
            },
            "not" => Token {
                token: TokenKind::Not,
                token_str: None,
            },
            "create" => Token {
                token: TokenKind::Create,
                token_str: None,
            },
            "table" => Token {
                token: TokenKind::Table,
                token_str: None,
            },
            "real" => Token {
                token: TokenKind::Type(TypeKind::Real),
                token_str: None,
            },
            "numeric" => Token {
                token: TokenKind::Type(TypeKind::Numberic),
                token_str: None,
            },
            "auto_increment" | "serial" | "autoincrement" => Token {
                token: TokenKind::AutoIncrement,
                token_str: None,
            },
            "boolean" | "bool" => Token {
                token: TokenKind::Type(TypeKind::Bool),
                token_str: None,
            },
            "true" => Token {
                token: TokenKind::Type(TypeKind::Bool),
                token_str: None,
            },
            "false" => Token {
                token: TokenKind::Type(TypeKind::Bool),
                token_str: None,
            },
            "varchar" => Token {
                token: TokenKind::Type(TypeKind::Varchar),
                token_str: None,
            },
            "integer" => Token {
                token: TokenKind::Type(TypeKind::Integer),
                token_str: None,
            },
            "blob" => Token {
                token: TokenKind::Type(TypeKind::Blob),
                token_str: None,
            },
            "null" => Token {
                token: TokenKind::Type(TypeKind::Null),
                token_str: None,
            },
            "text" => Token {
                token: TokenKind::Type(TypeKind::Text),
                token_str: None,
            },
            "=" => Token {
                token: TokenKind::Equals,
                token_str: None,
            },
            "(" => Token {
                token: TokenKind::OpenParen,
                token_str: None,
            },
            ")" => Token {
                token: TokenKind::CloseParen,
                token_str: None,
            },
            ":" => Token {
                token: TokenKind::Colon,
                token_str: None,
            },
            "*" => Token {
                token: TokenKind::Asterisk,
                token_str: None,
            },
            ";" => Token {
                token: TokenKind::Semicolon,
                token_str: None,
            },
            "," => Token {
                token: TokenKind::Comma,
                token_str: None,
            },
            "+" => Token {
                token: TokenKind::Plus,
                token_str: None,
            },
            "-" => Token {
                token: TokenKind::Minus,
                token_str: None,
            },
            "." => Token {
                token: TokenKind::Dot,
                token_str: None,
            },
            _ => Token {
                token: TokenKind::Name,
                token_str: Some(word),
            },
        }
    }
}
