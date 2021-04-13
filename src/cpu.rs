use std::fmt::Display;

pub struct CPU {
    pc: u32,
    usp: u32,
    data_register: Vec<u32>,
    address_register: Vec<u32>,
    memory: Vec<u8>,
    sr: u16,
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            pc: 0,
            usp: 0,
            data_register: vec![0; 8],
            address_register: vec![0; 8],
            memory: vec![0; 0x1000000],
            sr: 0,
        }
    }
}

impl Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "* PC : 0x{:08x}\n* USP: 0x{:08x}\n* data_registers:\n", self.pc, self.usp)?;
        for (n, x) in self.data_register.iter().enumerate() {
            write!(f, "\t* D[{}] = 0x{:08x}\n", n, x)?;
        }
        write!(f, "* address_register\n")?;
        for (n, x) in self.address_register.iter().enumerate() {
            write!(f, "\t* A[{}] = 0x{:08x}\n", n, x)?;
        }
        write!(f, "* CCR: {:016b}\n", self.sr)?;
        Ok(())
    }
}

impl CPU {
    pub fn foo(&self) {
        println!("{}", self);
    }

    pub fn set_c_flag(&mut self) {
        self.sr = self.sr | 0b00000000_00000001;
    }

    pub fn set_v_flag(&mut self) {
        self.sr = self.sr | 0b00000000_00000010;
    }

    pub fn set_z_flag(&mut self) {
        self.sr = self.sr | 0b00000000_00000100;
    }

    pub fn set_n_flag(&mut self) {
        self.sr = self.sr | 0b00000000_00001000;
    }

    pub fn set_x_flag(&mut self) {
        self.sr = self.sr | 0b00000000_00010000;
    }
}
