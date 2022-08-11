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
    input: Vec<char>,
    position: Pos,
    ch: Option<char>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Pos {
    file: Option<String>,
    line: usize,
    column: usize,
    raw: usize,
}

const INTERNAL_ERR_MSG: &str = "This is an error within renrs-lang itself.\nReport this bug on Github: https://github.com/FaCsaba/renrs-lang/issues";

impl Pos {
    pub fn advance(&mut self, new_line: bool) -> &Self {
        if new_line {
            self.column += 1;
        } else {
            self.line += 1;
        }
        self.raw += 1;
        return self;
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
#[derive(Debug, PartialEq)]
pub enum Token {
    ASSIGN(char),
    PLUS(char),
    MINUS(char),

    LCurly(char),
    RCurly(char),

    LParen(char),
    RParen(char),

    EndOfLine,

    NUM(Vec<char>),
    IDENT(Vec<char>),

    INVALID(char),
}

impl Lexer {
    #[warn(dead_code)]
    pub fn new(input: &str) -> Self {
        let a: Vec<char> = input.chars().collect();
        return Lexer {
            input: a,
            position: Pos::default(),
            ch: None,
        };
    }

    fn read_char(&mut self) -> Option<char> {
        if (self.position.raw + 1) > self.input.len() {
            self.ch = None;
            return None;
        }
        self.ch = Some(self.input[self.position.raw]);

        self.position.advance(self.ch == Some('\n'));
        self.ch
    }

    fn peak_next_char(&self) -> Option<char> {
        if (self.position.raw + 1) > self.input.len() {
            return None
        }
        Some(self.input[self.position.raw])
    }

    fn is_whitespace(&self) -> bool {
        match self.ch {
            Some(ch) => match ch {
                ' ' | '\t' => return true,
                _ => return false,
            },
            _ => return false,
        }
    }

    fn is_ident_char(&self) -> bool {
        self.ch.is_some_and(|&ch| ch.is_alphabetic() || ch == '_')
    }

    fn is_alphanumberic(&self) -> bool {
        self.ch.is_some_and(|&ch| ch.is_alphanumeric())
    }

    
    fn read_ident(&mut self) -> Result<Vec<char>, CompilationErr> {
        let mut ident = vec![];
        let mut can_be_num = false;
        while self.is_ident_char() || (can_be_num && self.is_alphanumberic()) {
            ident.push(self.ch.ok_or(CompilationErr {
                kind: CompilationErrKind::Unreachable,
                message: format!("Reached unreachable. {}", INTERNAL_ERR_MSG),
            })?); // We know this can not panic
            self.read_char();
            can_be_num = true;
        }
        Ok(ident)
    }

    fn is_num_char(&self) -> bool {
        self.ch.is_some_and(|&ch| ch.is_numeric() || ch == '.')
    }
    
    fn read_num(&mut self) -> Result<Vec<char>, CompilationErr> {
        let mut num = vec![];
        while self.is_num_char() {
            num.push(self.ch.ok_or(CompilationErr {
                kind: CompilationErrKind::Unreachable,
                message: format!("Reached unreachable. {}", INTERNAL_ERR_MSG),
            })?);
            self.read_char();
        }
        if num.iter().filter(|x| **x == '.').count() > 1 || (num.len() == 1 && num[0] == '.') {
            Err(CompilationErr {
                kind: CompilationErrKind::InvalidNumber,
                message: format!(
                    "{} isn't a valid number",
                    num.iter().cloned().collect::<String>()
                ),
            })
        } else {
            Ok(num)
        }
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
                '=' => return Some(Ok((Token::ASSIGN(ch), pos))),
                '+' => return Some(Ok((Token::PLUS(ch), pos))),
                '-' => return Some(Ok((Token::MINUS(ch), pos))),

                '{' => return Some(Ok((Token::LCurly(ch), pos))),
                '}' => return Some(Ok((Token::RCurly(ch), pos))),
                '(' => return Some(Ok((Token::LParen(ch), pos))),
                ')' => return Some(Ok((Token::RParen(ch), pos))),

                '\n' | ';' => return Some(Ok((Token::EndOfLine, pos))),
                _ => {
                    if self.is_ident_char() {
                        return Some(Ok((Token::IDENT(self.read_ident().ok()?), pos)));
                    }
                    if self.is_num_char() {
                        return Some(Ok((Token::NUM(self.read_num().ok()?), pos)));
                    }
                    return Some(Ok((Token::INVALID(ch), pos)));
                }
            },
            None => return None,
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
            Token::ASSIGN('='),
            Token::PLUS('+'),
            Token::MINUS('-'),
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
            Token::IDENT(vec!['_', 'i', 'd', 'e', 'n', 't'])
        );
        let mut lex = Lexer::new("hello");
        assert_eq!(
            lex.next().unwrap()?.0,
            Token::IDENT(vec!['h', 'e', 'l', 'l', 'o'])
        );
        let mut lex = Lexer::new("   \nhello");
        lex.next();
        assert_eq!(
            lex.next().unwrap()?.0,
            Token::IDENT(vec!['h', 'e', 'l', 'l', 'o'])
        );
        let mut lex = Lexer::new("     h1");
        assert_eq!(lex.next().unwrap()?.0, Token::IDENT(vec!['h', '1']));
        Ok(())
    }
    #[test]
    fn num() -> Result<(), CompilationErr> {
        let mut lex = Lexer::new("1");
        assert_eq!(lex.next().unwrap()?.0, Token::NUM(vec!['1']));
        let mut lex = Lexer::new(".1");
        assert_eq!(lex.next().unwrap()?.0, Token::NUM(vec!['.', '1']));
        let mut lex = Lexer::new("1.1");
        assert_eq!(lex.next().unwrap()?.0, Token::NUM(vec!['1', '.', '1']));
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
    fn incorrect_single_dot() {
        let mut lex = Lexer::new(".");
        let _ = lex.next().unwrap();
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
        let mut lex = Lexer::new("asdf.sdf");
        lex.next();
    }
}
