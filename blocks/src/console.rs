pub struct Console {
    input: String,
    output: String,
}

impl Console {
    #[inline]
    pub fn new() -> Console {
        Console {
            input: String::new(),
            output: String::new(),
        }
    }

    #[inline]
    pub fn write(&mut self, c: char) {
        match c {
            '\u{8}' => {
                self.input.pop();
            }
            _ => {
                self.input.push(c);
            }
        }
    }

    pub fn parse_commands<F: FnMut(Command)>(&mut self, mut f: F) {
        let mut trim_index = 0;
        let mut state = Parser::Start;
        for (index, character) in self.input.char_indices() {
            match state {
                Parser::Start => {
                    match character {
                        ' ' | '\r' | '\n' => {
                            // Remain in Parser::Start state.
                        }
                        '/' => {
                            state = Parser::S;
                        }
                        _ => {
                            state = Parser::Error;
                        }
                    }
                }
                Parser::Error => match character {
                    '\r' | '\n' => {
                        let s = &self.input[trim_index..index];
                        f(Command::Invalid(String::from(s)));
                        trim_index = index + 1;
                    }
                    _ => {
                        // Remain in Parser::Error state.
                    }
                },
                Parser::S => match character {
                    'q' => {
                        state = Parser::SQ;
                    }
                    '\r' | '\n' => {
                        let s = &self.input[trim_index..index];
                        f(Command::Invalid(String::from(s)));
                        trim_index = index + 1;
                    }
                    _ => {
                        state = Parser::Error;
                    }
                },
                Parser::SQ => match character {
                    'u' => {
                        state = Parser::SQU;
                    }
                    '\r' | '\n' => {
                        let s = &self.input[trim_index..index];
                        f(Command::Invalid(String::from(s)));
                        trim_index = index + 1;
                    }
                    _ => {
                        state = Parser::Error;
                    }
                },
                Parser::SQU => match character {
                    'i' => {
                        state = Parser::SQUI;
                    }
                    '\r' | '\n' => {
                        let s = &self.input[trim_index..index];
                        f(Command::Invalid(String::from(s)));
                        trim_index = index + 1;
                    }
                    _ => {
                        state = Parser::Error;
                    }
                },
                Parser::SQUI => match character {
                    't' => {
                        state = Parser::SQUIT;
                    }
                    '\r' | '\n' => {
                        let s = &self.input[trim_index..index];
                        f(Command::Invalid(String::from(s)));
                        trim_index = index + 1;
                    }
                    _ => {
                        state = Parser::Error;
                    }
                },
                Parser::SQUIT => match character {
                    ' ' => {
                        // Remain in Parser::SQUIT state.
                    }
                    '\r' | '\n' => {
                        f(Command::Quit);
                        trim_index = index + 1;
                    }
                    _ => {
                        state = Parser::Error;
                    }
                },
            }
        }
        self.output.push_str(&self.input[0..trim_index]);
        self.input.drain(0..trim_index);
    }

    #[inline]
    pub fn input(&self) -> &str {
        self.input.as_ref()
    }

    #[inline]
    pub fn output(&self) -> &str {
        self.output.as_ref()
    }
}

pub enum Command {
    Invalid(String),
    Quit,
}

pub enum Parser {
    Start,
    Error,
    S,
    SQ,
    SQU,
    SQUI,
    SQUIT,
    // SF,
    // SFO,
    // SFON,
    // SFONT,
    // SFONT_,
    // SFONT_S,
    // SFONT_SI,
    // SFONT_SIZ,
    // SFONT_SIZE,
    // SFONT_SIZE_,
    // SFONT_SIZE_D,

}
