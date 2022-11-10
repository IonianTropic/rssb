use core::fmt;
use std::{ops::{IndexMut, Index}, fmt::Write, path::Path, fs::File, io::Read, env};


fn main() {
    let mut cpu = Rssb::new();
    let args: Vec<String> = env::args().collect();
    // cpu._load_sample_program();
    cpu.load(&args[1]);
    cpu.run();
}


#[derive(Debug)]
struct Rssb {
    mem: Memory,
    mar: usize,
    mdr: usize,
    borrow: bool,
}


impl Rssb {
    fn new() -> Self {
        Self {
            mem: Memory::new(),
            mar: 0,
            mdr: 0,
            borrow: false,
        }
    }

    fn run(&mut self) {
        let mut eof1;
        let mut eof2 = false;
        loop {
            self.mar = self.mem[0] as usize;          // PC
            self.mdr = self.mem[self.mar] as usize;    // x
        
            (self.mem[1], self.borrow) = self.mem[self.mdr].overflowing_sub(self.mem[1]); // A = mem[x] - A
    
            self.mem[self.mdr] = self.mem[1]; // mem[x] = A
    
            (self.mem[0], eof1) = self.mem[0].overflowing_add(1); // PC += 1
    
            if self.borrow {
                (self.mem[0], eof2) = self.mem[0].overflowing_add(1); // PC += 1
            }
    
            if eof1 || eof2 { break; } // if PC overflow exit
            if self.mem[0] == 0 && self.mem[1] == 0 { break; } // if loop type 1 exit
            if self.mem[0] == 2 && self.mem[1] == 1 { break; } // if loop type 2 exit
        }
        println!("***********************************************************************");
        println!("--REGISTERS--\n{}", self);
        print!("--MEMORY--\n{}", self.mem.hexdump());
        println!("***********************************************************************");
    }

    fn load<P: AsRef<Path>>(&mut self, path: P) {
        let mut f = File::open(path).unwrap();
        f.read_exact(&mut self.mem.raw).unwrap();
    }

    fn _load_sample_program(&mut self) {
        // registers
        self.mem[0] = 16; // PC
        self.mem[1] = 65; // A

        // data
        self.mem[8] = 66; // 8 := temp
        self.mem[9] = 67; // 9 := x 
        self.mem[10] = 68; // 10 := y
        self.mem[11] = 69; // 11 := z

        // text
        self.mem[16] = 8;
        self.mem[17] = 8;
        self.mem[18] = 8;
        self.mem[19] = 9;
        self.mem[20] = 9;
        self.mem[21] = 10;
        self.mem[22] = 8;
        self.mem[23] = 8;
        self.mem[24] = 9;
        self.mem[25] = 8;
        self.mem[26] = 8;
        self.mem[27] = 8;
        self.mem[28] = 11;
        self.mem[29] = 9;
    }
}


impl fmt::Display for Rssb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "PC:\t{:02x}\nA:\t{:02x}\nZERO:\t{:02x}\nMAR:\t{:02x}\nMDR:\t{:02x}\nBORROW\t{}\n",
            self.mem[0],
            self.mem[1],
            self.mem[2],
            self.mar,
            self.mdr,
            self.borrow,
        )
    }
}


#[derive(Debug)]
struct Memory {
    raw: [u8; 256],
    null: u8,
}


impl Memory {
    fn new() -> Self {
        Self {
            raw: [0; 256],
            null: 0,
        }
    }

    fn hexdump(&self) -> String {
        let row_len = 16;
        let mem_matrix = self.raw.chunks(row_len);
        let mut w = String::new();
        let mut matched = false;
        let mut double_matched = false;
        let mut previous: &[u8] = &[];
        w.push_str("    x0 x1 x2 x3 x4 x5 x6 x7 x8 x9 xa xb xc xd xe xf\n");
        for (idx, row) in mem_matrix.into_iter().enumerate() {
            if previous == row {
                match matched {
                    true => {
                        double_matched = true;
                    }
                    false => {
                        matched = true;
                    }
                }
            } else {
                matched = false;
                double_matched = false;
            }
            if !matched {
                let mut hexstr = String::new();
                let mut asciistr = String::new();
                for item in row {
                    write!(&mut hexstr, " {:02x}", item).unwrap();
                }
                for item in row {
                    if item.is_ascii_alphabetic() || item.is_ascii_graphic() {
                        asciistr.push(*item as char);
                    } else if item.is_ascii_whitespace() {
                        asciistr.push(' ');
                    } else {
                        asciistr.push('.');
                    }
                }
                writeln!(&mut w, "{:02x} {}  |{}|", idx*row_len, hexstr, asciistr).unwrap();
            } else if matched  && !double_matched {
                writeln!(&mut w, "*").unwrap();
            }
            previous = row;
        }
        w
    }
}


impl Index<usize> for Memory {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            2_usize => &0_u8,
            _ => &self.raw[index],
        }
    }
}


impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            2_usize => &mut self.null,
            _ => &mut self.raw[index],
        }
    }
}
