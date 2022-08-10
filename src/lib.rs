pub mod ren_rs_lang {

    #[derive(Debug)]
    pub enum Token {
        INT(char),
        IDENT(Vec<char>),
        EOF
    }

    /**
     * # Lexer
     * -- implementation from [Mohit Karekar](https://mohitkarekar.com/posts/pl/lexer/)
     *  
     * what it does: it breaks down source code input and makes tokens out of them
     * 
     * ---
     * 
     * ### We have the following fields in this struct
     * - input: Vec<char>       -- Source code
     * - position: usize        -- Current reading position
     * - read_position: usize   -- Current moving read position
     * - ch: char               -- Current read character
     * 
     * ---
     * 
     * ## Examples
     * 
     * ```
     * let two = ren_rs_lang::Lexer::new("1 + 1");
     * 
     * ```
     * 
     */
    #[derive(Debug)]
    pub struct Lexer {
        input: Vec<char>,           
        position: Pos,
        ch: Option<char>
    }

    #[derive(Default, Debug)]
    struct Pos {
        file: Option<String>,
        line: usize,
        column: usize,
        raw: usize
    }

    impl Lexer {
        pub fn new(input: &str) -> Self {
            let a: Vec<char> = input.chars().collect();
            return Lexer {input: a, position: Pos::default(), ch: None}
        }

        pub fn read_char(&mut self) {
            if self.position.raw >= self.input.len() {
                self.ch = None;
                return
            }
            self.ch = Some(self.input[self.position.raw+1]);

            let line = if self.ch == Some('\n') {self.position.line+1} else {self.position.line};
            let column = if self.ch != Some('\n') {self.position.column+1} else {self.position.line};
            let file = self.position.file.to_owned();
            self.position = Pos {raw: self.position.raw + 1, file, line, column};
        }
    }

    impl Iterator for Lexer {
        type Item = Token;

        fn next(&mut self) -> std::option::Option<Token> {
            self.read_char();
            return None
        }
    }

    pub struct Parser {}


    pub fn parse() {
        unimplemented!();
    }

    pub fn compile() {
        unimplemented!();
    }

    pub fn run() {
        unimplemented!();
    }
}



#[test]
fn name() {
   let mut a = ren_rs_lang::Lexer::new("");
   println!("{:?}", a.next());
}