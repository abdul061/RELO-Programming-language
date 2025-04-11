#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Literal<'a> {
    BoolTrue,
    BoolFalse,
    Int(i64),
    Float(f64),
    Char(char),
    Str(&'a str),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum KeyWord {
    ProgramStart,
    ProgramEnd,
    Print,

    StartDeclare,
    Declare,
    Assign,
    DeclareAlt,

    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    Equal,
    NotEqual,

    Sum,
    Sub,
    Mul,
    Div,
    Mod,

    SemiColon,
    LeftBrace,
    RightBrace,

    IfCond,
    ElseCond,
    WhileLoop,
    ForStart,
    ForRangeStart,
    ForRangeEnd,
    EndBlock,
    BreakLoop,
    FuncDeclare,
    EndFunc,
    FuncReturn,
    FuncCall,
}

impl KeyWord {
    pub(crate) const fn as_str(&self) -> &'static str {
        match self {
            KeyWord::ProgramStart => "START",
            KeyWord::ProgramEnd => "END",
            KeyWord::Print => "DISPLAY",

            KeyWord::StartDeclare => "VARIABLE",
            KeyWord::Declare => "IS",
            KeyWord::Assign => "ASSIGN",
            KeyWord::DeclareAlt => "IS",

            KeyWord::GreaterThan => ">",
            KeyWord::LessThan => "<",
            KeyWord::GreaterThanEqual => ">=",
            KeyWord::LessThanEqual => "<=",
            KeyWord::Equal => "==",
            KeyWord::NotEqual => "!=",

            KeyWord::Sum => "+",
            KeyWord::Sub => "-",
            KeyWord::Mul => "*",
            KeyWord::Div => "/",
            KeyWord::Mod => "%",

            KeyWord::SemiColon => ";",
            KeyWord::LeftBrace => "{",
            KeyWord::RightBrace => "}",

            KeyWord::IfCond => "WHEN",
            KeyWord::ElseCond => "OTHERWISE",
            KeyWord::WhileLoop => "REPEAT",
            KeyWord::ForStart => "LOOP",
            KeyWord::ForRangeStart => "Start",
            KeyWord::ForRangeEnd => "End",
            KeyWord::EndBlock => "DONE",
            KeyWord::BreakLoop => "QUIT",
            KeyWord::FuncDeclare => "CREATE FUNCTION",
            KeyWord::EndFunc => "END_FN",
            KeyWord::FuncReturn => "RETURN",
            KeyWord::FuncCall => "CALL",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum TokenKind<'a> {
    Eof,
    Ident(&'a str),
    Comment(&'a str),
    KeyWord(KeyWord),
    Literal(Literal<'a>),
}

impl<'a> Default for TokenKind<'a> {
    fn default() -> Self {
        Self::Eof
    }
}

impl<'a> Default for &TokenKind<'_> {
    fn default() -> &'static TokenKind<'static> {
        &TokenKind::Eof
    }
}

impl<'a> From<KeyWord> for TokenKind<'a> {
    fn from(val: KeyWord) -> TokenKind<'a> {
        TokenKind::KeyWord(val)
    }
}

// impl<'a> From<KeyWord> for &TokenKind<'_> {
//     fn from(val: KeyWord) -> &'static TokenKind<'static> {
//         let static_val: &'static TokenKind = &TokenKind::KeyWord(val);
//         static_val
// let static_str: &'static str = "This is a static string";
//     }
// }

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Span {
    pub(crate) row: usize,
    pub(crate) col: usize,
    pub(crate) length: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Token<'a> {
    pub(crate) kind: TokenKind<'a>,
    pub(crate) span: Span,
}
