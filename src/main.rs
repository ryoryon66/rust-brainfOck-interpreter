// reference https://en.wikipedia.org/wiki/Brainfuck
use core::panic;
use std::fs;
use std::io;

fn read_program_from_file(file_name: String) -> String {
    match fs::read_to_string(file_name) {
        Ok(program) => program.replace("\n", "").replace(" ", ""),
        Err(e) => {
            println!("{:?}", e);
            panic!("failed to load the specified program...")
        }
    }
}

#[derive(Debug)]
struct Brainfuck {
    memory: Vec<u8>,
    curr_ptr: usize,
    program: Vec<char>,
    pc: usize,
}

impl Brainfuck {
    fn new(program: String) -> Self {
        let program: Vec<char> = program.chars().collect();

        Brainfuck {
            memory: vec![0; 4096],
            curr_ptr: 0,
            program: program,
            pc: 0,
        }
    }

    fn run(&mut self) {
        while self.pc < self.program.len() {
            self.step()
        }
    }

    fn step(&mut self) {
        let instraction = self.program.get(self.pc).unwrap();

        match instraction {
            '>' => {
                self.curr_ptr += 1;
                self.pc += 1;
            }
            '<' => {
                self.curr_ptr -= 1;
                self.pc += 1;
            }
            '+' => {
                self.memory[self.curr_ptr] = match self.memory[self.curr_ptr].checked_add(1) {
                    Some(v) => v,
                    None => 0,
                };
                self.pc += 1
            }
            '-' => {
                self.memory[self.curr_ptr] = match self.memory[self.curr_ptr].checked_sub(1) {
                    Some(v) => v,
                    None => 255,
                };
                self.pc += 1
            }
            '.' => {
                print!("{}", char::from(self.memory[self.curr_ptr]));
                self.pc += 1;
            }
            ',' => {
                //文字入力する仕様
                let mut user_input = String::new();
                io::stdin().read_line(&mut user_input).ok();
                let user_input = user_input.chars().next().unwrap();

                self.memory[self.curr_ptr] = user_input as u8;

                // print!("{}", char::from(self.memory[self.curr_ptr]));

                self.pc += 1;
            }
            '[' => match self.memory[self.curr_ptr] {
                0 => {
                    let mut depth = 0;

                    loop {
                        self.pc += 1;
                        let c = self.program[self.pc];

                        match c {
                            '[' => {
                                depth += 1;
                            }
                            ']' => {
                                if depth == 0 {
                                    break;
                                } else {
                                    depth -= 1;
                                }
                            }
                            _ => {}
                        }
                    }
                    self.pc += 1;
                }
                _ => {
                    self.pc += 1;
                }
            },
            ']' => match self.memory[self.curr_ptr] {
                0 => {
                    self.pc += 1;
                }
                _ => {
                    let mut depth = 0;

                    loop {
                        self.pc -= 1;
                        let c = self.program[self.pc];

                        match c {
                            ']' => {
                                depth += 1;
                            }
                            '[' => {
                                if depth == 0 {
                                    break;
                                } else {
                                    depth -= 1;
                                }
                            }
                            _ => {}
                        }
                    }

                    self.pc += 1;
                }
            },
            _ => {
                println!("invalid:{}", instraction);
                panic!("invalid instraction")
            }
        }
    }
}

fn main() {
    let program = read_program_from_file("program.txt".to_string());
    let mut interpreter = Brainfuck::new(program);
    interpreter.run();
}
