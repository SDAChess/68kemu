use super::instruction::*;
use DataContainer::*;
use Mnemonic::*;
use OpSize::*;
use std::ops::Deref;


fn _get_usize(op_size: &OpSize) -> usize {
    match op_size {
        BYTE => 3,
        WORD => 2,
        LONG => 0,
    }
}

fn _is_null(val: &Vec<u8>) -> bool {
    for byte_ in &val[..] {
        if *byte_ != 0 {
            return false;
        }
    }
    return true;
}

fn _is_negative(val: &Vec<u8>) -> bool {
    if val[0] > 0x7f {
        true
    }
    else {
        false
    }
}

impl super::CPU {

    fn get_target<'a>(&'a mut self, data : &'a DataContainer, op_size : &'a OpSize) -> &'a mut [u8] {
        let adjust = _get_usize(&op_size);
        match data {
            DATA_REGISTER(ui) => &mut self.data_register[*ui][adjust..],
            ADDRESS_REGISTER(ui) => &mut self.address_register[*ui][adjust..],
            IMEDIATE_VALUE(vect) => self.cache_value(vect, adjust),
            MEMORY_ADDR(addr) => &mut self.memory[*addr..(*addr + 4 - adjust)],
            SR => &mut self.sr,
            CCR => &mut self.sr[1..],
        }
    }

    pub fn perform_move(&mut self, inst: &Instruction) {
        //kills the instruction ? I don't think so #loops
        self.clear_c_flag();
        self.clear_v_flag();
        
        let lhs = self.get_target(inst.get_lhs(), inst.get_size());
        let mut tmp: Vec<u8> = vec![0;lhs.len()];
        println!("{}", lhs.len());
        tmp.copy_from_slice(lhs);
        
        if _is_negative(&tmp) {
            self.set_n_flag();
        }
        else {
            self.clear_n_flag();
        }

        if _is_null(&tmp) {
            self.set_z_flag();
        }
        else {
            self.clear_z_flag();
        }
        
        let trg = self.get_target(inst.get_trg(), inst.get_size());
        let mut i: usize = 0; 
        while i < trg.len() {
            trg[i] = tmp[i];
            i += 1;
        }
        
    }
}
