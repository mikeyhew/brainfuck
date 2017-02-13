use std::io::{Read, Write, Stdin, Stdout};
use std::mem;
use std::collections::VecDeque;

use std::char::from_u32;

fn main() {
    let mut args = std::env::args();
    let source_filename = args.nth(1).expect("requires source file name");
    let mut source_file = std::fs::File::open(source_filename).unwrap();
    let mut source_buf = String::new();
    source_file.read_to_string(&mut source_buf).unwrap();

    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    let mut brainfuck = BrainFuck::new(&source_buf);
    brainfuck.process_instructions(&mut stdin, &mut stdout);
}

struct InstructionPointer {
    chars: Vec<char>,
    idx: usize
}


impl InstructionPointer {
    fn new(source: &str) -> InstructionPointer {
        let chars = source.chars().collect::<Vec<char>>();
        InstructionPointer {
            chars: chars,
            idx: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.idx).cloned()
    }

    /* for advance and retreat, return value is whether it moved or not */

    fn advance(&mut self) -> bool {
        if self.idx < self.chars.len() {
            self.idx += 1;
            true
        } else {
            false
        }
    }

    fn retreat(&mut self) -> bool {
        if self.idx > 0 {
            self.idx -= 1;
            true
        } else {
            false
        }
    }

    fn next(&mut self) -> Option<char> {
        let instr = self.peek();
        self.advance();
        instr
    }
}

struct DataPointer {
    cells: VecDeque<u32>,
    idx: isize,
    cells_offset: isize
}

impl DataPointer {
    fn new() -> DataPointer {
        let mut cells = VecDeque::new();
        cells.push_back(0);
        DataPointer {
            cells: cells,
            idx: 0,
            cells_offset: 0,
        }
    }

    fn advance(&mut self) {
        self.idx += 1;
        if (self.cells_offset + self.idx) as usize  == self.cells.len() {
            self.cells.push_back(0);
        }
    }

    fn retreat(&mut self) {
        self.idx -= 1;
        if self.cells_offset + self.idx < 0 {
            self.cells.push_front(0);
            self.cells_offset += 1;
        }
    }

    fn inc(&mut self) {
        self.cells[(self.cells_offset + self.idx) as usize] += 1;
    }

    fn dec(&mut self) {
        self.cells[(self.cells_offset + self.idx) as usize] -= 1;
    }

    fn set(&mut self, value: u32) {
        self.cells[(self.cells_offset + self.idx) as usize] = value;
    }

    fn get(&self) -> u32 {
        self.cells[(self.cells_offset + self.idx) as usize]
    }

    fn is_zero(&self) -> bool {
        self.cells[(self.cells_offset + self.idx) as usize] == 0
    }
}

struct BrainFuck {
    data_pointer: DataPointer,
    instr_pointer: InstructionPointer,
}

impl BrainFuck {
    fn new(source: &str) -> BrainFuck {
        BrainFuck {
            data_pointer: DataPointer::new(),
            instr_pointer: InstructionPointer::new(&source),
        }
    }

    fn process_instructions (&mut self, stdin: &mut Stdin, stdout: &mut Stdout) {
        while let Some(instr) = self.instr_pointer.next() {
            match instr {
                '+' => self.data_pointer.inc(),
                '-' => self.data_pointer.dec(),
                '>' => self.data_pointer.advance(),
                '<' => self.data_pointer.retreat(),
                '.' => {
                    let byte = [self.data_pointer.get() as u8];
                    stdout.write(&byte);
                },
                ',' => {
                    let mut byte: [u8; 1] = [0];
                    stdin.read(&mut byte);
                    self.data_pointer.set(byte[0] as u32);
                },
                '[' => {
                    // println!("1st LBRACK");
                    if self.data_pointer.is_zero() {
                        let mut num_lbrackets = 0;
                        loop {
                            let instr = self.instr_pointer.next().expect("no matching ] before end of file");
                            match instr {
                                '[' => {
                                    // println!("LBRACK");
                                    num_lbrackets += 1
                                },
                                ']' => {
                                    // println!("RBRACK");
                                    if num_lbrackets == 0 { break }
                                    num_lbrackets -= 1
                                },
                                _ => {}
                            }
                        }
                        // println!("DONE")
                    }
                },
                ']' => {
                    // println!("1st RBRACK");
                    if !self.data_pointer.is_zero() {
                        let mut num_rbrackets = 0;
                        // extra retreat needed so we don't read the same ] again
                        self.instr_pointer.retreat();
                        loop {
                            if self.instr_pointer.retreat() {
                                match self.instr_pointer.peek().unwrap() {
                                    ']' => {
                                        // println!("RBRACK");
                                        num_rbrackets += 1
                                    },
                                    '[' => {
                                        // println!("LBRACK");
                                        if num_rbrackets == 0 { break }
                                        num_rbrackets -= 1
                                    },
                                    _ => {}
                                }
                            } else {
                                panic!("no matching [ in file");
                            }
                        }
                        // println!("DONE")
                    }
                },
                _ => {}
            }
        }
    }
}
