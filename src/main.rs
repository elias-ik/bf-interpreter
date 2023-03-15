//implementation according to:
//https://en.wikipedia.org/wiki/Brainfuck

use std::{env, fs, convert::identity, io::{stdin, Read, self, Write}};


#[derive(PartialEq)]
enum Instruction{
    IncrementDataPointer,
    DecrementDataPointer,
    IncrementData,
    DecrementData,
    OutputData,
    InputData,
    JumpForward,
    JumpBackward,
}

impl Instruction {
    fn from_char(c: char) -> Option<Instruction> {
        match c {
            '>' => Some(Instruction::IncrementDataPointer),
            '<' => Some(Instruction::DecrementDataPointer),
            '+' => Some(Instruction::IncrementData),
            '-' => Some(Instruction::DecrementData),
            '.' => Some(Instruction::OutputData),
            ',' => Some(Instruction::InputData),
            '[' | '{' | '(' => Some(Instruction::JumpForward),
            ']' | '}' | ')' => Some(Instruction::JumpBackward),
            _ => None
        }
    }
}

struct BrainfuckedState {
    data: Vec<u8>,
    data_ptr: usize,
    code_ptr: usize,
    instructions: Vec<Instruction>,
}

impl BrainfuckedState {

    fn init(is: Vec<Instruction>) -> BrainfuckedState {
        BrainfuckedState {
            data: vec![0],
            data_ptr: 0,
            code_ptr: 0,
            instructions: is,
        }
    }

    fn execute(&mut self){
        while self.code_ptr < self.instructions.len() {
            match self.instructions[self.code_ptr] {
                Instruction::IncrementDataPointer => {
                    self.data_ptr+=1;
                    if self.data.len() <= self.data_ptr {
                        self.data.push(0)
                    }
                },
                Instruction::DecrementDataPointer => {
                    if self.data_ptr == 0{
                        self.data.push(0);
                        for i in 0..(self.data.len()-1) {
                            self.data[i+1] = self.data[i];
                        }
                        self.data[0] = 0;
                    }else{
                        self.data_ptr-=1;
                    }
                },
                Instruction::IncrementData => self.data[self.data_ptr] = self.data[self.data_ptr].wrapping_add(1),
                Instruction::DecrementData => self.data[self.data_ptr] = self.data[self.data_ptr].wrapping_sub(1),
                Instruction::OutputData => {
                    let output_char = self.data[self.data_ptr] as char;
                    io::stdout().write(format!("{}", output_char).as_bytes()).unwrap();
                },
                Instruction::InputData => {
                    io::stdout().flush().unwrap();
                    let mut buffer = [0 as u8];
                    _ = stdin().read(&mut buffer);
                    self.data[self.data_ptr] = buffer[0];
                },
                Instruction::JumpForward => {
                    if self.data[self.data_ptr] == 0 {
                        let mut counter = 1;
                        while counter > 0 {
                            self.code_ptr += 1;
                            match self.instructions[self.code_ptr] {
                                Instruction::JumpForward => counter += 1,
                                Instruction::JumpBackward => counter -= 1,
                                _ => ()
                            }
                        }
                    }
                },
                Instruction::JumpBackward => {
                    if self.data[self.data_ptr] != 0 {
                        let mut counter = 1;
                        while counter > 0 {
                            self.code_ptr -= 1;
                            match self.instructions[self.code_ptr] {
                                Instruction::JumpForward => counter -= 1,
                                Instruction::JumpBackward => counter += 1,
                                _ => ()
                            }
                        }
                    }
                },
            }
            self.code_ptr += 1;
        }
        io::stdout().flush().unwrap();
    }
}

fn ri(fp: &str) -> Vec<Instruction> {
    let code = fs::read_to_string(fp).expect("unable to read inputfile");
    let instructions: Vec<Instruction> = code
        .chars()
        .map(|u| 
            Instruction::from_char(u)
        )
        .filter_map(identity)
        .collect();
    return instructions;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename.bf>", args[0]);
        std::process::exit(1);
    }
    let is = ri(args[1].as_str());
    BrainfuckedState::init(is).execute();
    println!()
}
