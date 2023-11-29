use std::process::exit;

pub struct Formatter {
    source: String,
    index: usize,
}

impl Formatter {
    pub fn create(source: String) -> Formatter {
        Formatter {
            source,
            index: 0
        }
    }

    pub fn format(&mut self) {

        let c: Vec<char> = self.source.chars().collect();
        while !self.at_end(c.clone()) {
            match c[self.index] {
                '<' => {
                    if self.check_next(c.clone(), 't') {
                        if self.check_next(c.clone(), '>') {
                            self.title(c.clone());
                        } else {
                            eprint!("expected '>' ");
                            self.advance();
                            exit(0)
                        }
                    } else if self.check_next(c.clone(), 'e') {
                        if self.check_next(c.clone(), '>') {
                            self.example(c.clone());
                        } else {
                            eprint!("expected '>' ");
                            self.advance();
                            exit(0)
                        }
                    } else if self.check_next(c.clone(), 'p') {
                            self.point(c.clone());

                    } else if self.check_next(c.clone(), 'i') {
                        if self.check_next(c.clone(), '>') {
                            self.info(c.clone());
                        } else {
                            eprint!("expected '>' ");
                            self.advance();
                            exit(0)
                        }
                    } else if self.check_next(c.clone(), 'b') {
                        if self.check_next(c.clone(), '>') {
                            println!(" ");
                        } else {
                            eprint!("expected '>' ");
                            self.advance();
                            exit(0)
                        }
                    }

                }
                ' ' | '\n' | '\t' | '\r' => {
                    self.advance();
                }
                _ => {
                    self.advance();
                }
            }
        }

    }

    fn title(&mut self, c: Vec<char>) {
        print!("[=========================================================================================]\n");
        if self.check_next(c.clone(), '"') {
            let mut title = String::new();
            while !self.check_next(c.clone(), '"') {
                self.advance();
                title.push(c[self.index]);
            }
            let mut i = 0;
            let mut blanks = String::new();
            while i < (91 - title.len()) / 2  {
                i += 1;
                blanks.push(' ');
            }
            println!("{1}{0}{1} ", title.to_uppercase(), blanks);
        }
        print!("[=========================================================================================]\n");
        self.advance();
    }

    fn example(&mut self, c: Vec<char>) {
        if self.check_next(c.clone(), '"') {
            let mut example = String::new();
            while !self.check_next(c.clone(), '"') {
                self.advance();
                example.push(c[self.index]);
            }
            println!(" //{}", example)
        }

    }
    fn point(&mut self, c: Vec<char>) {
        self.advance();
        let pointline =
        match c[self.index] {
            '1' => {
                " -"
            },
            '2' => {
                "  -"
            },
            '3' => {
                "   -"
            }
            '4' => {
                "    -"
            }
            '5' => {
                "     -"
            }
            '6' => {
                "      -"
            }
            '7' => {
                "       -"
            }
            '8' => {
                "        -"
            }
            '9' => {
                "          -"
            }
            _ => {
                eprint!("undefined point");
                print!("{}", c[self.index]);
                exit(0)}
        };
        if self.check_next(c.clone(), '>') {
            if self.check_next(c.clone(), '"') {
                let mut point = String::new();
                while !self.check_next(c.clone(), '"') {
                    self.advance();
                    point.push(c[self.index]);
                }
                println!("{} {}", pointline, point)
            }
        } else {
            eprint!("expected '>'");
            exit(0)
        }

    }
    fn info(&mut self, c: Vec<char>) {
        if self.check_next(c.clone(), '"') {
            let mut field = String::new();
            while !self.check_next(c.clone(), '"') {
                self.advance();
                field.push(c[self.index]);
            }
            println!(" {}", field)
        }

    }


    fn at_end(&mut self, c: Vec<char>) -> bool {
        if self.index < c.len() {
            return false;
        }
        true
    }

    fn advance(&mut self) {
        self.index += 1;
    }
    fn check_next(&mut self, c: Vec<char>, e: char) -> bool {
        if c[self.index + 1] == e {
            self.advance();
            return true;
        }

        false
    }
}