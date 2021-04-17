mod cpu_actions;
mod instruction;

use std::fmt::Display;
use cpu_actions::*;
pub use instruction::*;
pub use DataContainer::*;
pub use OpSize::*;
pub use Mnemonic::*;

fn write_byte_array(f: &mut std::fmt::Formatter<'_>, b_array: &Vec<u8>) -> std::result::Result<(), std::fmt::Error> {
    for byte_ in b_array {
        write!(f, "{:02x}", byte_)?;
    };
    write!(f, "\n")?;
    Ok(())
}

pub struct CPU {
    //Big endian
    pc: Vec<u8>,
    usp: Vec<u8>,
    data_register: Vec<Vec<u8>>,
    address_register: Vec<Vec<u8>>,
    memory: Vec<u8>,
    sr: Vec<u8>,
    cache: Vec<u8>,
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            pc: vec![0;4],
            usp: vec![0;4],
            data_register: vec![vec![0;4];8],
            address_register: vec![vec![0;4];8],
            memory: vec![0;0x1000000],
            sr: vec![0;2],
            cache: vec![0;4],
        }
    }
}


impl Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "* PC = 0x")?;
        write_byte_array(f, &(self.pc))?;
        write!(f, "* USP = 0x")?;
        write_byte_array(f, &(self.usp))?;
        write!(f, "* Data Registers: \n")?;
        for (n, x) in self.data_register.iter().enumerate() {
            write!(f, "\t* D[{}] = 0x", n)?;
            write_byte_array(f, &x)?;
        }
        write!(f, "* Address Registers: \n")?;
        for (n, x) in self.address_register.iter().enumerate() {
            write!(f, "\t* A[{}] = 0x", n)?;
            write_byte_array(f, &x)?;
        }
        write!(f, "SR = ")?;
        for x in &(self.sr) {
            write!(f, "{:08b}", x)?;
        }
        write!(f, "\n")?;
        write!(f, "Cache = 0x")?;
        write_byte_array(f, &(self.cache))?;
        write!(f, "\n")?;
        Ok(())
    }
}

impl CPU {

    pub fn foo(&mut self) {
        self.set_x_flag();
        self.set_z_flag();
        self.set_v_flag();
        self.set_c_flag();
        self.clear_n_flag();
        println!("{}", self);
    }

    //Set flags:

    fn set_c_flag(&mut self) {
        self.sr[1] = self.sr[1] | 0b00000001;
    }

    fn set_v_flag(&mut self) {
        self.sr[1] = self.sr[1] | 0b00000010;
    }

    fn set_z_flag(&mut self) {
        self.sr[1] = self.sr[1] | 0b00000100;
    }

    fn set_n_flag(&mut self) {
        self.sr[1] = self.sr[1] | 0b00001000;
    }

    fn set_x_flag(&mut self) {
        self.sr[1] = self.sr[1] | 0b00010000;
    }

    //Clear Flags: 

    fn clear_c_flag(&mut self) {
        self.sr[1] = self.sr[1] & 0b11111110
    }

    fn clear_v_flag(&mut self) {
        self.sr[1] = self.sr[1] & 0b11111101;
    }

    fn clear_z_flag(&mut self) {
        self.sr[1] = self.sr[1] & 0b11111011;
    }

    fn clear_n_flag(&mut self) {
        self.sr[1] = self.sr[1] & 0b11110111;
    }

    fn clear_x_flag(&mut self) {
        self.sr[1] = self.sr[1] & 0b11101111;
    }

    //Other specific funcs:

    fn cache_value<'a>(&'a mut self, val: &'a Vec<u8>, n: usize) -> &'a mut [u8] {
        self.cache = val.clone();
        &mut self.cache[n..]
    }

    pub fn get_ccr(&self) -> u8 {
        self.sr[1]
    }

    pub fn get_data_reg(&self, i: usize) -> Option<&[u8]> {
        if i < 8 {
            Some(&(self.data_register[i])[..])
        }
        else {
            None
        }
    }

    pub fn get_addr_reg(&self, i: usize) -> Option<&[u8]> {
        if i < 8 {
            Some(&(self.address_register[i])[..])
        }
        else {
            None
        }
    }

    pub fn get_memory_offset(&self, offset: usize, len: usize) -> Option<&[u8]> {
        if offset >= 0x1000000 || offset + len >= 0x1000000 {
            None
        }
        else {
            Some(&self.memory[offset..(offset + len)])
        }
    }

    pub fn print_mem(&self, offset: usize, len: usize) {
        let mem = self.get_memory_offset(offset, len);
        if let Some(x) = mem {
            print!("[0x{:x}] = 0x", offset);
            for e in x {
                print!("{:02x}", e);
            }
            println!("");
        }
        else { 
            println!("print_mem: None");
        }
    }

    pub fn execute(&mut self, inst: &Instruction) {
        match inst.get_op() {
            MOVE => self.perform_move(inst),
            _ => (),
        }
    }

    //test_purposes, felt cute, might delete late
    pub fn cpu_flag_move_test(&mut self) {
        self.set_n_flag();
        self.set_c_flag();
        self.set_x_flag();
    }
}
