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

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    let brainfuck = BrainFuck::new(&source_buf);
    brainfuck.process_instructions(stdin, stdout);
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

    fn next(&mut self) -> Option<char> {
        if self.idx == self.chars.len() {
            self.idx = self.chars.len();
            None
        } else {
            self.idx += 1;
            self.chars.get(self.idx - 1).cloned()
        }
    }

    fn current_location(&self) -> usize {
        self.idx
    }

    fn goto(&mut self, location: usize) {
        self.idx = location;
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

    fn process_instructions (&mut self, mut stdin: Stdin, mut stdout: Stdout) {
        while let Some(instr) = self.instr_pointer.next() {
            match instr {
                '+' => self.data_pointer.inc(),
                '-' => self.data_pointer.dec(),
                '>' => self.data_pointer.advance(),
                '<' => self.data_pointer.retreat(),
                '.' => {
                    let c = from_u32(self.data_pointer.get()).unwrap();
                    let mut buf: [u8; 4] = [0,0,0,0];
                    let bytes = c.encode_utf8(&mut buf).as_bytes();
                    stdout.write(bytes).unwrap();
                },
                _ => {}
            }
        }
    }
}
