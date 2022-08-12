use super::err::{CompilationErr, CompilationErrKind};

/**
 * # Lexer
 * -- implementation from [Mohit Karekar](https://mohitkarekar.com/posts/pl/lexer/)
 *  
 * what it does: it breaks down source code input and makes tokens out of them
 *
 * ## We have the following fields in this struct
 * - input: Vec<char>       -- Source code
 * - position: usize        -- Current reading position
 * - read_position: usize   -- Current moving read position
 * - ch: char               -- Current read character
 *
 * ## Examples
 *
 * ```ignore
 * use super::Lexer;
 * let two = Lexer::new("1 + 1");
 * ```
 */
#[derive(Debug)]
pub struct Lexer {
    input: std::iter::Peekable<std::vec::IntoIter<char>>,
    position: Pos,
    ch: Option<char>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pos {
    file: Option<String>,
    line: usize,
    column: usize,
    raw: Option<usize>,
}

impl Default for Pos {
    fn default() -> Self {
        Pos {
            file: None,
            line: 1,
            column: 0,
            raw: None,
        }
    }
}

const INTERNAL_ERR_MSG: &str = r#"Reached unreachable. This is an error within renrs-lang itself.
Report this bug on Github: https://github.com/FaCsaba/renrs-lang/issues"#;

impl Pos {
    pub fn advance(&mut self, new_line: bool) -> Result<&Self, CompilationErr> {
        if new_line {
            self.line += 1;
        } else {
            self.column += 1;
        }
        match self.raw {
            None => self.raw = Some(0),
            _ => {
                self.raw = Some(
                    self.raw.ok_or(CompilationErr {
                        kind: CompilationErrKind::Unreachable,
                        message: format!(
                            "We expected a character at: {:?}:{}:{} \n{}",
                            self.file.as_ref().or(Some(&String::from(""))),
                            self.line,
                            self.column,
                            INTERNAL_ERR_MSG
                        ),
                    })? + 1,
                )
            }
        }
        Ok(self)
    }
}

/**
 * # Token
 *
 * A way of representing a character or string of characters,
 * by identifying them
 *
 * ```ignore
 * use Lexer;
 * let mut lex = Lexer::new("1 + 1");
 * ```
 */
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Assign(char),
    Plus(char),
    Minus(char),

    LCurly(char),
    RCurly(char),

    LParen(char),
    RParen(char),

    Dot,

    Num(Vec<char>),
    Ident(Vec<char>),

    String(Vec<char>),

    EndOfLine,
    Invalid(char),
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect::<Vec<char>>().into_iter().peekable(),
            position: Pos {
                file: None,
                line: 0,
                column: 1,
                raw: None,
            },
            ch: None,
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.position.advance(self.ch == Some('\n')).unwrap();
        self.ch = self.input.next();
        self.ch
    }

    fn is_whitespace(&self) -> bool {
        match self.ch {
            Some(ch) => matches!(ch, ' ' | '\t'),
            _ => false,
        }
    }

    fn is_alphabetic(ch: Option<&char>) -> bool {
        ch.is_some_and(|&ch| ch.is_alphabetic() || ch == &'_')
    }

    fn is_alphanumeric(ch: Option<&char>) -> bool {
        ch.is_some_and(|&ch| ch.is_alphanumeric() || ch == &'_')
    }

    fn read_ident(&mut self) -> Result<Vec<char>, CompilationErr> {
        let mut ident = vec![self.ch.ok_or(CompilationErr {
            kind: CompilationErrKind::Unreachable,
            message: format!("{}", INTERNAL_ERR_MSG.to_string()),
        })?];

        while Self::is_alphanumeric(self.input.peek()) {
            if let Some(c) = self.read_char() {
                ident.push(c);
            }
        }
        Ok(ident)
    }

    fn is_num_char(ch: Option<&char>) -> bool {
        ch.is_some_and(|&ch| ch.is_numeric() || ch == &'.')
    }

    fn read_num(&mut self) -> Result<Vec<char>, CompilationErr> {
        let mut num = vec![self.ch.ok_or(CompilationErr {
            kind: CompilationErrKind::Unreachable,
            message: format!("{}", INTERNAL_ERR_MSG),
        })?];

        while Self::is_num_char(self.input.peek()) {
            if let Some(c) = self.read_char() {
                num.push(c);
            }
        }

        if num.iter().filter(|&ch| ch == &'.').count() > 1 {
            return Err(CompilationErr {
                kind: CompilationErrKind::InvalidNumber,
                message: format!(
                    "Invalid number at: {}:{}:{}",
                    self.position.file.as_ref().unwrap_or(&String::from("")),
                    self.position.line,
                    self.position.column
                ),
            });
        }
        Ok(num)
    }

    fn read_string(&mut self) -> Result<Vec<char>, CompilationErr> {
        let mut string = vec![];
        while self.input.peek() != None
            && self.input.peek() != Some(&'"')
            && self.input.peek() != Some(&'\'')
            && self.input.peek() != Some(&'\n')
            && self.input.peek() != Some(&';')
        {
            string.push(self.read_char().unwrap()) // Unreachable
        }

        if self.input.peek() == Some(&'"') {
            self.read_char();
        }

        Ok(string)
    }

    fn take_whitespace(&mut self) {
        while self.is_whitespace() {
            self.read_char();
        }
    }
}

impl Iterator for Lexer {
    type Item = Result<(Token, Pos), CompilationErr>;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_char();
        self.take_whitespace();
        let pos = self.position.clone();
        match self.ch {
            Some(ch) => match ch {
                '=' => Some(Ok((Token::Assign(ch), pos))),
                '+' => Some(Ok((Token::Plus(ch), pos))),
                '-' => Some(Ok((Token::Minus(ch), pos))),

                '{' => Some(Ok((Token::LCurly(ch), pos))),
                '}' => Some(Ok((Token::RCurly(ch), pos))),
                '(' => Some(Ok((Token::LParen(ch), pos))),
                ')' => Some(Ok((Token::RParen(ch), pos))),

                '"' | '\'' | '`' => Some(if let Ok(string) = self.read_string() {
                    Ok((Token::String(string), pos))
                } else {
                    Err(CompilationErr {
                        kind: CompilationErrKind::InvalidString,
                        message: format!(
                            "Invalid String found at: {}:{},{}",
                            self.position.file.as_ref().unwrap_or(&String::from("")),
                            self.position.line,
                            self.position.column
                        ),
                    })
                }),

                '\n' | ';' => Some(Ok((Token::EndOfLine, pos))),
                o => {
                    if Self::is_alphabetic(Some(&o)) {
                        return Some(Ok((Token::Ident(self.read_ident().ok()?), pos)));
                    }

                    if o == '.' && Self::is_alphabetic(self.input.peek()) {
                        return Some(Ok((Token::Dot, pos)));
                    }

                    if Self::is_num_char(Some(&o)) {
                        return Some(Ok((Token::Num(self.read_num().ok()?), pos)));
                    }
                    Some(Ok((Token::Invalid(ch), pos)))
                }
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn end_of_file() {
        let mut a = Lexer::new("");
        assert_eq!(a.next(), None);
    }

    #[test]
    fn all_single_tokens() -> Result<(), CompilationErr> {
        let src = vec!["=", "+", "-", "{", "}", "(", ")"];
        let toks = vec![
            Token::Assign('='),
            Token::Plus('+'),
            Token::Minus('-'),
            Token::LCurly('{'),
            Token::RCurly('}'),
            Token::LParen('('),
            Token::RParen(')'),
        ];
        for (i, t) in src.into_iter().enumerate() {
            let mut lex = Lexer::new(t);
            let tok = lex.next().unwrap()?;
            assert_eq!(tok.0, toks[i]);
        }
        Ok(())
    }

    #[test]
    fn ident() -> Result<(), CompilationErr> {
        let mut lex = Lexer::new("_ident");
        assert_eq!(
            lex.next().unwrap()?.0,
            Token::Ident(vec!['_', 'i', 'd', 'e', 'n', 't'])
        );
        let mut lex = Lexer::new("hello");
        assert_eq!(
            lex.next().unwrap()?.0,
            Token::Ident(vec!['h', 'e', 'l', 'l', 'o'])
        );
        let mut lex = Lexer::new("   \nhello");
        lex.next();
        assert_eq!(
            lex.next().unwrap()?.0,
            Token::Ident(vec!['h', 'e', 'l', 'l', 'o'])
        );
        let mut lex = Lexer::new("     h1");
        assert_eq!(lex.next().unwrap()?.0, Token::Ident(vec!['h', '1']));
        Ok(())
    }

    #[test]
    fn num() -> Result<(), CompilationErr> {
        let mut lex = Lexer::new("1");
        assert_eq!(lex.next().unwrap()?.0, Token::Num(vec!['1']));
        let mut lex = Lexer::new(".1");
        assert_eq!(lex.next().unwrap()?.0, Token::Num(vec!['.', '1']));
        let mut lex = Lexer::new("1.1");
        assert_eq!(lex.next().unwrap()?.0, Token::Num(vec!['1', '.', '1']));
        Ok(())
    }

    #[test]
    #[should_panic]
    fn incorrect_num_dots() {
        let mut lex = Lexer::new("..");
        _ = lex.next().unwrap();
    }

    #[test]
    #[should_panic]
    fn incorrect_num() {
        let mut lex = Lexer::new("0000.10.0");
        let _ = lex.next().unwrap();
    }

    #[test]
    fn token_chain() {
        let mut lex = Lexer::new("a b c d");
        lex.next();
        lex.next();
        lex.next();
        lex.next();
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn dot_for_access() {
        let mut lex = Lexer::new("a.z");
        lex.next();
        assert_eq!(lex.next().unwrap().unwrap().0, Token::Dot);
    }

    #[test]
    fn reading_string() {
        let mut lex = Lexer::new(r#""Hello, world!"#);
        assert_eq!(
            Token::String(vec![
                'H', 'e', 'l', 'l', 'o', ',', ' ', 'w', 'o', 'r', 'l', 'd', '!'
            ]),
            lex.next().unwrap().unwrap().0
        );
    }

    #[test]
    fn read_till_new_line_or_another_quoteation_mark() {
        let mut lex = Lexer::new("\"a\n\"b");
        assert_eq!(Token::String(vec!['a']), lex.next().unwrap().unwrap().0);
        assert_eq!(Token::EndOfLine, lex.next().unwrap().unwrap().0);
        assert_eq!(Token::String(vec!['b']), lex.next().unwrap().unwrap().0);

        let mut lex = Lexer::new("\"a\" b");
        assert_eq!(Token::String(vec!['a']), lex.next().unwrap().unwrap().0);
        assert_eq!(Token::Ident(vec!['b']), lex.next().unwrap().unwrap().0);
    }

    //#[test]
    fn _complex() {
        let lex = Lexer::new(
            r#"c = Character "Crab", "./sprites/crab"
c_idle = Animation {
    c show left
    wait 1s
    c show right
}
c_idle run
c "What a fine day""#,
        );
        panic!(
            "{:?}",
            lex.collect::<Vec<Result<(Token, Pos), CompilationErr>>>()
        )
    }
}
