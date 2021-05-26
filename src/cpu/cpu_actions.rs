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

fn _is_null(val: &[u8]) -> bool {
    for byte_ in val {
        if *byte_ != 0 {
            return false;
        }
    }
    return true;
}

fn _is_negative(val: &[u8]) -> bool {
    if val[0] > 0x7f {
        true
    }
    else {
        false
    }
}

fn _perform_add(v1: &[u8], v2: &[u8]) -> (bool, Vec<u8>) {
    let mut c = 0;
    let mut result = v1.iter().rev().zip(v2.iter().rev())
                    .map(|(x, y)| {
                        let y = match y.checked_add(c) {
                            Some(r) => {c = 0; r},
                            None => (((*y as u16) + (c as u16)) % 0x100) as u8,
                        };
                        match x.checked_add(y) {
                            Some(r) => {c = 0; r},
                            None => {c = 1; 
                                (((*x as u16) + (y as u16)) % 0x100) as u8   
                            },
                        }
                    })
                    .collect::<Vec<u8>>();
    while result.len() < 4 {
        result.push(0);
    }
    result.reverse();
    return (c == 1, result)
}

mod internal_tests {
    use super::*;
    #[test]
    fn test_perform_add_basic() {
        let v1 = vec![0xff];
        let v2 = vec![0x1];
        let v3 = vec![0x2];

        let (flag, r) = _perform_add(&v1[..], &v2[..]);
        assert_eq!(r, vec![0, 0, 0, 0]);
        assert_eq!(flag, true);
        let (flag, r) = _perform_add(&v2[..], &v3[..]);
        assert_eq!(r, vec![0x00, 0x00, 0x00, 0x3]);
        assert_eq!(flag, false);

        let (flag, r) = _perform_add(&v2[..], &(vec![0])[..]);
        assert_eq!(r, vec![0x00, 0x00, 0x00, 0x1]);
        assert_eq!(flag, false);
    }

    #[test]
    fn test_perform_add_report() {
        let v1 = vec![0x00, 0x00, 0x00, 0xff];
        let v2 = vec![0x00, 0x00, 0x00, 0x01];

        let (flag, r) = _perform_add(&v1[..], &v2[..]);
        assert_eq!(r, vec![0x00, 0x00, 0x01, 0x00]);
        assert_eq!(flag, false);

        let (flag, r) = _perform_add(&v1[1..], &v2[1..]);
        assert_eq!(r, vec![0x00, 0x00, 0x01, 0x00]);
        assert_eq!(flag, false);

        let (flag, r) = _perform_add(&v1[2..], &v2[2..]);
        assert_eq!(r, vec![0x00, 0x00, 0x01, 0x00]);
        assert_eq!(flag, false);
    }

    #[test]
    fn test_perform_add_max() {
        let v1 = vec![0xff, 0x00, 0x00, 0xff];
        let v2 = vec![0x01, 0x00, 0x00, 0x01];

        let (flag, r) = _perform_add(&v1[..], &v2[..]);
        assert_eq!(r, vec![0x00, 0x00, 0x01, 0x00]);
        assert_eq!(flag, true);

        let (flag, r) = _perform_add(&v1[1..], &v2[1..]);
        assert_eq!(r, vec![0x00, 0x00, 0x01, 0x00]);
        assert_eq!(flag, false);
    }
}

impl super::CPU {

    fn get_target_mut<'a>(&'a mut self, data : &'a DataContainer, op_size : &'a OpSize) -> &'a mut [u8] {
        let adjust = _get_usize(&op_size);
        match data {
            DATA_REGISTER(ui) => &mut self.data_register[*ui][adjust..],
            ADDRESS_REGISTER(ui) => &mut self.address_register[*ui][adjust..],
            IMEDIATE_VALUE(_) => panic!("Imediate Value is immutable !"),
            MEMORY_ADDR(addr) => &mut self.memory[*addr..(*addr + 4 - adjust)],
            SR => &mut self.sr,
            CCR => &mut self.sr[1..],
            EMPTY => panic!("TODO: implement behaviour when instruction has empty args"),
        }
    }

    fn get_target<'a>(&'a self, data : &'a DataContainer, op_size : &'a OpSize) -> &'a [u8] {
        let adjust = _get_usize(&op_size);
        match data {
            DATA_REGISTER(ui) => &self.data_register[*ui][adjust..],
            ADDRESS_REGISTER(ui) => &self.address_register[*ui][adjust..],
            IMEDIATE_VALUE(vect) => &vect[adjust..],
            MEMORY_ADDR(addr) => &self.memory[*addr..(*addr + 4 - adjust)],
            SR => &self.sr,
            CCR => &self.sr[1..],
            EMPTY => panic!("TODO: implement behaviour when instruction has empty args"),
        }
    }

    pub fn perform_move(&mut self, inst: &Instruction) {
        //kills the instruction ? I don't think so #loops
        self.clear_c_flag();
        self.clear_v_flag();
        
        let lhs = self.get_target(inst.get_lhs(), inst.get_size());
        let mut tmp: Vec<u8> = vec![0;lhs.len()];
        tmp.copy_from_slice(lhs);
        
        if _is_negative(&tmp[..]) {
            self.set_n_flag();
        }
        else {
            self.clear_n_flag();
        }

        if _is_null(&tmp[..]) {
            self.set_z_flag();
        }
        else {
            self.clear_z_flag();
        }
        
        let trg = self.get_target_mut(inst.get_trg(), inst.get_size());
        let mut i: usize = 0; 
        while i < trg.len() {
            trg[i] = tmp[i];
            i += 1;
        }

    }

    pub fn perform_tst(&mut self, inst : &Instruction) {
        let elt = self.get_target(inst.get_lhs(), inst.get_size());
        let neg = _is_negative(&elt);
        let zero = _is_null(&elt);
        self.clear_v_flag();
        self.clear_c_flag();
        if neg {
            self.set_n_flag();
        }
        else {
            self.clear_n_flag();
        }
        if zero {
            self.set_z_flag();
        }
        else {
            self.clear_z_flag();
        }
    }
    
    pub fn perform_add(&mut self, inst: &Instruction) {
        //Needs testing !!!
        let lhs = self.get_target(inst.get_lhs(), inst.get_size());
        let trg = self.get_target(inst.get_trg(), inst.get_size());
        let mut v_flag = false;
        let adjust = _get_usize(inst.get_size());
        
        let (set_c_flag, result) = _perform_add(lhs, trg);
        let cpy_result = result.to_vec();

        let trg_container = match inst.get_trg() {
            DATA_REGISTER(x) => DATA_REGISTER(*x),
            ADDRESS_REGISTER(x) => ADDRESS_REGISTER(*x),
            MEMORY_ADDR(x) => MEMORY_ADDR(*x),
            SR => SR,
            CCR => CCR,
            x => panic!("Invalid data_container for add_target: {:?}", x),
        };

        
        if ((_is_negative(lhs) && _is_negative(trg) && !_is_negative(&result[adjust..])) ||
        (!_is_negative(lhs) && !_is_negative(trg) && _is_negative(&result[adjust..]))) {
            v_flag = true;
        }

        self.perform_move(&Instruction::new(MOVE, *inst.get_size(), 
            IMEDIATE_VALUE(cpy_result), trg_container));
        
        //n and z flags handled by move

        if set_c_flag {
            self.set_c_flag();
            self.set_x_flag();
        }
        else {
            self.clear_c_flag();
            self.clear_x_flag();
        }

        if v_flag {
            self.set_v_flag();
        }
        else {
            self.clear_v_flag();
        }

    }
}
