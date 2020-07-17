use std::io;
use std::io::Read;

/// A CPU that runs on brainfuck 'opcodes'.
/// It has some restrictions that are not technically part of the spec: the memory pointer can not go 'behind' the first memory location
/// (i.e., 'negative memory values), memory values will wrap at 255
pub struct CPU {
    ir: usize,
    mem_ptr: usize,

    output: u8,

    rom: Mem,
    ram: Mem,
}

impl CPU {
    /// step moves the cpu forward one cycle, which probably includes reading an instruction from rom, executing it, and changing memory values.
    /// If execution passes the end of program rom, we return true.
    pub fn step(&mut self) -> bool {

        if self.output != 0 {
            print!("{}", self.output as char);
            self.output = 0;
        }

        if self.rom.mem.len() <= self.ir {
            println!("ir passed end of program at value {}", self.ir);
            return true
        }

        let op = self.rom.read(self.ir.into());

        match op as char{
            // move mem pointer 'to the right' (add one)
            '>' => {
                self.mem_ptr+=1;
                self.ir+=1;
                // println!("set mem_ptr to {}", self.mem_ptr)
            },
            // move mem pointer to the left
            '<' => {
                self.mem_ptr-=1;
                self.ir+=1;
                // println!("set mem_ptr to {}", self.mem_ptr)
            }
            // increment value at memptr
            '+' => {
                let val = self.ram.read(self.mem_ptr);
                self.ram.write(self.mem_ptr, val+1);
                // println!("set val of {} to {}", self.mem_ptr, self.ram.read(self.mem_ptr));
                self.ir+=1;
            }
            // decrement value at memptr
            '-' => {
                let val = self.ram.read(self.mem_ptr);
                self.ram.write(self.mem_ptr, val-1);
                // println!("set val of {} to {}", self.mem_ptr, self.ram.read(self.mem_ptr));
                self.ir+=1;
            }
            // jump forward if zero
            '[' if self.ram.read(self.mem_ptr) == 0 => {
                let mut skip = 0;
                let mut next_ir = self.ir+1;
                // println!("looking for matching ] at {}", next_ir);
                while self.rom.read(next_ir) as char != ']' || skip != 0 {
                    if self.rom.read(next_ir) as char == '[' {
                        skip += 1;
                    } else if self.rom.read(next_ir) as char == ']' {
                        skip -= 1;
                    };
                    next_ir+=1;
                }
                self.ir = next_ir;
            }
            // jump backward if not zero
            ']' if self.ram.read(self.mem_ptr) != 0 => {
                let mut skip = 0;
                let mut next_ir = self.ir-1;
                // println!("looking for matching [ at {}", next_ir);
                while self.rom.read(next_ir) as char != '[' || skip != 0 {
                    if self.rom.read(next_ir) as char == ']' {
                        skip += 1;
                    } else if self.rom.read(next_ir) as char == '[' {
                        skip -= 1;
                    };
                    next_ir-=1;
                }
                self.ir = next_ir;
            }
            // output character at ptr
            '.' => {
                // println!("printing");
                self.output = self.ram.read(self.mem_ptr);
                self.ir+=1;
            }
            // read input
            ',' => {
                // println!("reaing");
                let c = io::stdin().bytes().next();
                match c {
                    None => self.ir+=1, // EOF case I think
                    Some(r) => {
                        let k = r.unwrap_or(0);
                        self.ram.write(self.mem_ptr, k)
                    },
                }
                // self.ram.write(self.mem_ptr, c);
                // self.ir+=1;
            }
            // characters unspecified in the spec are noops
            _ => {
                // println!("{} is a noop", op as char);
                self.ir+=1;
            },
        };

        false
    }

    /// load_program replaces cpu rom with the program. It does not zero memory so watch out for that.
    pub fn load_program(&mut self, prog: &str) {
        let p = prog.as_bytes();
        self.rom = Mem{
            mem: p.to_vec()
        };
        self.mem_ptr = 0;
        self.ir = 0;
    }
}

pub fn new_cpu(rom_size: usize, ram_size: usize) -> CPU {
    CPU{
        ir: 0,
        mem_ptr: 0,

        output: 0,

        rom: new_mem(rom_size),
        ram: new_mem(ram_size),
    }
}

pub struct Mem {
    mem: Vec<u8>
}

impl Mem {
    pub fn read(&self, addr: usize) -> u8 {
        return self.mem[addr]
    }

    pub fn write(&mut self, addr: usize, val: u8) {
        self.mem[addr] = val
    }
}

pub fn new_mem(sz: usize) -> Mem {
    Mem{
        mem: vec![0; sz]
    }
}
