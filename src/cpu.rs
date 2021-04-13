use std::fmt::Display;

pub struct CPU {
    pc: u32,
    usp: u32,
    data_register: Vec<u32>,
    address_register: Vec<u32>,
    memory: Vec<u8>,
    ccr: u16,
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            pc: 0,
            usp: 0,
            data_register: vec![0; 8],
            address_register: vec![0; 8],
            memory: vec![0; 0x1000000],
            ccr: 0,
        }
    }
}

impl Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "PC : 0x{:08x}\nUSP: 0x{:08x}\ndata_registers:\n  A0: 0x{:08x}\n  A1: 0x{:08x}\n", self.pc, self.usp, self.data_register[0], self.data_register[1])?;
        Ok(())
    }
}

impl CPU {
    pub fn foo(&self) {
        println!("{}", self);
    }
}
