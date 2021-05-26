#[derive(Debug)]
pub enum OpSize {
    BYTE,
    WORD,
    LONG
}

#[derive(Debug)]
pub enum Mnemonic {
    MOVE,
    MOVEA,
    ADD,
    ADDA,
    LEA,
    TST,
}

#[derive(Debug)]
pub enum DataContainer {
    DATA_REGISTER(usize),
    ADDRESS_REGISTER(usize),
    IMEDIATE_VALUE(Vec<u8>), //4 u8 big endian
    MEMORY_ADDR(usize),
    SR,
    CCR,
    EMPTY,
}

pub struct Instruction {
    op: Mnemonic,
    size: OpSize,
    lhs: DataContainer,
    trg: DataContainer,
}

impl Copy for OpSize { }

impl Clone for OpSize {
    fn clone(&self) -> OpSize {
        *self
    }
}

impl Instruction {
    pub fn new(op: Mnemonic, size: OpSize, lhs: DataContainer, trg: DataContainer) -> Instruction {
        // Need to implement safety for the 
        // indexes of the vects of 
        // on the DataContainer ?
        Instruction {
            op,
            size,
            lhs,
            trg
        }
    }

    pub fn get_op(&self) -> &Mnemonic {
        &self.op
    }

    pub fn get_size(& self) -> &OpSize {
        &self.size
    }

    pub fn get_lhs(&self) -> &DataContainer {
        &self.lhs
    }

    pub fn get_trg(&self) -> &DataContainer {
        &self.trg
    }
}
