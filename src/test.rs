use super::cpu::*;

fn _get_size_from_op(size: &OpSize) -> usize {
    match size {
        BYTE => 1,
        WORD => 2,
        LONG => 4,
    }
}

fn _move_imediate_to_mem(cpu: &mut CPU, size: OpSize) {
    let byte_size = _get_size_from_op(&size);
    let inst = Instruction::new(MOVE, size, 
        IMEDIATE_VALUE(vec![0xde, 0xad, 0xbe, 0xef]), MEMORY_ADDR(0x100));
    cpu.execute(&inst);
    assert_eq!(cpu.get_memory_offset(0x100, byte_size), Some(&(vec![0xde, 0xad, 0xbe, 0xef])[(4 - byte_size).. 4]));
}

fn _move_imediate_to_data(cpu: &mut CPU, size: OpSize, i: usize) {
    let byte_size = _get_size_from_op(&size);
    let inst = Instruction::new(MOVE, size,
        IMEDIATE_VALUE(vec![0xde, 0xad, 0xbe, 0xef]), DATA_REGISTER(i));
    cpu.execute(&inst);
    let mut k: usize = 4 - byte_size;
    let v = vec![0xde, 0xad, 0xbe, 0xef];
    let mut ret = vec![0;4];
    while k < 4 {
        ret[k] = v[k]; 
        k += 1;
    }
    assert_eq!(cpu.get_data_reg(i), Some(&ret[..]))
}

fn _move_imediate_to_addr(cpu: &mut CPU, size: OpSize, i: usize) {
    let byte_size = _get_size_from_op(&size);
    let inst = Instruction::new(MOVE, size,
        IMEDIATE_VALUE(vec![0xde, 0xad, 0xbe, 0xef]), ADDRESS_REGISTER(i));
    cpu.execute(&inst);
    let mut k: usize = 4 - byte_size;
    let v = vec![0xde, 0xad, 0xbe, 0xef];
    let mut ret = vec![0;4];
    while k < 4 {
        ret[k] = v[k]; 
        k += 1;
    }
    assert_eq!(cpu.get_addr_reg(i), Some(&ret[..]))
}

fn _check_integrity(cpu: &mut CPU, size: OpSize, i: usize, j: usize) {
    let byte_size = _get_size_from_op(&size);
    let inst = Instruction::new(MOVE, size,
        DATA_REGISTER(i), DATA_REGISTER(j));
    cpu.execute(&inst);
    let mut k: usize = 4 - byte_size;
    let v = vec![0xde, 0xad, 0xbe, 0xef];
    let mut ret = vec![0;4];
    while k < 4 {
        ret[k] = v[k]; 
        k += 1;
    }
    assert_eq!(cpu.get_data_reg(i), Some(&v[..]));
    assert_eq!(cpu.get_data_reg(j), Some(&ret[..]));
}

fn _set_byte_on_vec(size: &OpSize, val1: u8, val2: u8, val3: u8) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut v1: Vec<u8> = vec![0, 0, 0, 0];
    let mut v2: Vec<u8> = vec![0, 0, 0, 0];
    let mut v3: Vec<u8> = vec![0, 0, 0, 0];

    let adj = 4 - _get_size_from_op(&size);

    v1[adj] = val1;
    v2[adj] = val2;
    v3[adj] = val3;

    (v1, v2, v3)
}

fn _check_add_v_flag(size: OpSize) {
    let (v1, v2, v3) = _set_byte_on_vec(&size, 0x71, 0x10, 0x10);

    let mut cpu = CPU::default();
    let inst = Instruction::new(MOVE, size, IMEDIATE_VALUE(v1),
        DATA_REGISTER(1));
    cpu.execute(&inst);
    let inst = Instruction::new(ADD, size, IMEDIATE_VALUE(v2), 
        DATA_REGISTER(1));
    cpu.execute(&inst);

    assert_eq!(cpu.get_ccr(), 0b00001010);

    let inst = Instruction::new(ADD, size, IMEDIATE_VALUE(v3),
        DATA_REGISTER(1));
    
    cpu.execute(&inst);
    
    assert_eq!(cpu.get_ccr(), 0b00001000);
}



#[test]
fn move_imediate_to_mem() {
    let mut cpu: CPU = CPU::default();
    _move_imediate_to_mem(&mut cpu, BYTE);
    assert_eq!(cpu.get_ccr(), 0b00001000);
    let mut cpu: CPU = CPU::default();
    _move_imediate_to_mem(&mut cpu, WORD);
    assert_eq!(cpu.get_ccr(), 0b00001000);
    let mut cpu: CPU = CPU::default();
    _move_imediate_to_mem(&mut cpu, LONG);
    assert_eq!(cpu.get_ccr(), 0b00001000);
}

#[test]
fn move_imediate_to_data() {
    for i in 0..8 {
        let mut cpu: CPU = CPU::default();
        _move_imediate_to_data(&mut cpu, BYTE, i);
        let mut cpu: CPU = CPU::default();
        _move_imediate_to_data(&mut cpu, WORD, i);
        let mut cpu: CPU = CPU::default();
        _move_imediate_to_data(&mut cpu, LONG, i);
    }
}

#[test]
fn move_imediate_to_addr(){
    for i in 0..8 {
        let mut cpu: CPU = CPU::default();
        _move_imediate_to_addr(&mut cpu, BYTE, i);
        let mut cpu: CPU = CPU::default();
        _move_imediate_to_addr(&mut cpu, WORD, i);
        let mut cpu: CPU = CPU::default();
        _move_imediate_to_addr(&mut cpu, LONG, i);
    }
}

#[test]
fn check_move_integrity(){
    let mut cpu: CPU = CPU::default();
    _move_imediate_to_data(&mut cpu, LONG, 0);
    for i in 1..8 {
        _check_integrity(&mut cpu, BYTE, 0, i);
        _check_integrity(&mut cpu, WORD, 0, i);
        _check_integrity(&mut cpu, LONG, 0, i);
    }
}

#[test]
fn move_a_small_part_registers(){
    let mut cpu : CPU = CPU::default();
    let inst = Instruction::new(MOVE, LONG, 
        IMEDIATE_VALUE(vec![0xde, 0xad, 0xbe, 0xef]), DATA_REGISTER(5));
    cpu.execute(&inst);
    let inst = Instruction::new(MOVE, BYTE,
        IMEDIATE_VALUE(vec![0, 0, 0, 0xff]), DATA_REGISTER(5));
    cpu.execute(&inst);

    assert_eq!(cpu.get_data_reg(5), Some(&(vec![0xde, 0xad, 0xbe, 0xff])[..]));

    let inst = Instruction::new(MOVE, WORD,
        IMEDIATE_VALUE(vec![0, 0, 0xff, 0xfe]), DATA_REGISTER(5));
    cpu.execute(&inst);

    assert_eq!(cpu.get_data_reg(5), Some(&(vec![0xde, 0xad, 0xff, 0xfe])[..]))
}

#[test]
fn move_a_small_part_mem(){
    let mut cpu : CPU = CPU::default();
    let inst = Instruction::new(MOVE, LONG, 
        IMEDIATE_VALUE(vec![0xde, 0xad, 0xbe, 0xef]), MEMORY_ADDR(0x50));
    cpu.execute(&inst);
    let inst = Instruction::new(MOVE, BYTE,
        IMEDIATE_VALUE(vec![0, 0, 0, 0xff]), MEMORY_ADDR(0x53));
    cpu.execute(&inst);

    assert_eq!(cpu.get_memory_offset(0x50, 4), Some(&(vec![0xde, 0xad, 0xbe, 0xff])[..]));

    let inst = Instruction::new(MOVE, WORD,
        IMEDIATE_VALUE(vec![0, 0, 0xff, 0xfe]), MEMORY_ADDR(0x52));
    cpu.execute(&inst);

    assert_eq!(cpu.get_memory_offset(0x50, 4), Some(&(vec![0xde, 0xad, 0xff, 0xfe])[..]))
}

#[test]
fn move_zero(){
    let mut cpu = CPU::default();
    let inst = Instruction::new(MOVE, BYTE,
        IMEDIATE_VALUE(vec![0xff, 0xff, 0xff, 0x00]), MEMORY_ADDR(0x6969));
    cpu.execute(&inst);
    assert_eq!(cpu.get_ccr(), 0b00000100);
    
    let inst = Instruction::new(MOVE, LONG, 
        IMEDIATE_VALUE(vec![0x00, 0x00, 0x00, 0x00]), MEMORY_ADDR(0x4242));
    cpu.execute(&inst);
    assert_eq!(cpu.get_ccr(), 0b00000100);

    let inst = Instruction::new(MOVE, WORD,
        IMEDIATE_VALUE(vec![0xff, 0xbe, 0x00, 0x00]), MEMORY_ADDR(0x4242));
    cpu.execute(&inst);
    assert_eq!(cpu.get_ccr(), 0b00000100)
}

#[test]
fn move_non_zero() {
    let mut cpu = CPU::default();
    let inst = Instruction::new(MOVE, BYTE,
        IMEDIATE_VALUE(vec![0x00, 0x00, 0x00, 0x7f]), MEMORY_ADDR(0x6969));
    cpu.execute(&inst);
    assert_eq!(cpu.get_ccr(), 0);
    
    let inst = Instruction::new(MOVE, LONG, 
        IMEDIATE_VALUE(vec![0x00, 0x01, 0x00, 0x00]), MEMORY_ADDR(0x4242));
    cpu.execute(&inst);
    assert_eq!(cpu.get_ccr(), 0);

    let inst = Instruction::new(MOVE, WORD,
        IMEDIATE_VALUE(vec![0xff, 0xbe, 0x02, 0x00]), MEMORY_ADDR(0x4242));
    cpu.execute(&inst);
    assert_eq!(cpu.get_ccr(), 0)
}

#[test]
fn move_neg() {
    let mut cpu = CPU::default();
    let inst = Instruction::new(MOVE, BYTE,
        IMEDIATE_VALUE(vec![0x00, 0x00, 0x00, 0x8f]), MEMORY_ADDR(0x6969));
    cpu.execute(&inst);
    assert_eq!(cpu.get_ccr(), 0b00001000);
    
    let inst = Instruction::new(MOVE, LONG, 
        IMEDIATE_VALUE(vec![0xff, 0x01, 0x00, 0x00]), MEMORY_ADDR(0x4242));
    cpu.execute(&inst);
    assert_eq!(cpu.get_ccr(), 0b00001000);

    let inst = Instruction::new(MOVE, WORD,
        IMEDIATE_VALUE(vec![0xff, 0xbe, 0xe2, 0x00]), MEMORY_ADDR(0x4242));
    cpu.execute(&inst);
    assert_eq!(cpu.get_ccr(), 0b00001000)

}

#[test]
fn move_other_flags_behaviout(){
    let mut cpu = CPU::default();

    cpu.cpu_flag_move_test();
    let inst = Instruction::new(MOVE, BYTE,
        IMEDIATE_VALUE(vec![0x00, 0x00, 0x00, 0x8f]), MEMORY_ADDR(0x6969));
    cpu.execute(&inst);
    assert_eq!(cpu.get_ccr(), 0b00011000);

    cpu.cpu_flag_move_test();
    let inst = Instruction::new(MOVE, LONG, 
        IMEDIATE_VALUE(vec![0xff, 0x01, 0x00, 0x00]), MEMORY_ADDR(0x4242));
    cpu.execute(&inst);
    assert_eq!(cpu.get_ccr(), 0b00011000);

    cpu.cpu_flag_move_test();
    let inst = Instruction::new(MOVE, WORD,
        IMEDIATE_VALUE(vec![0xff, 0xbe, 0xe2, 0x00]), MEMORY_ADDR(0x4242));
    cpu.execute(&inst);
    assert_eq!(cpu.get_ccr(), 0b00011000)
}

#[test]
fn test_tst_functional() {
    let mut cpu = CPU::default();
    let inst = Instruction::new(TST, BYTE, IMEDIATE_VALUE(vec![0x00, 0x00, 0x00, 0x80]), EMPTY);
    cpu.execute(&inst);
    assert_eq!(cpu.get_ccr(), 0b00001000);
    
    let inst = Instruction::new(TST, BYTE, IMEDIATE_VALUE(vec![0x32, 0x41, 0x23, 0x00]), EMPTY);
    cpu.execute(&inst);
    assert_eq!(cpu.get_ccr(), 0b00000100);
}

#[test]
fn test_add_right_result_byte() {
    let mut cpu = CPU::default();
    let inst = Instruction::new(ADD, BYTE, IMEDIATE_VALUE(vec![0, 0, 0, 0xfe]),
        MEMORY_ADDR(0));
    cpu.execute(&inst);
    assert_eq!(cpu.get_memory_offset(0, 1), Some(&(vec![0xfe])[..]));
}

#[test]
fn test_add_right_result_word() {
    let mut cpu = CPU::default();
    let inst = Instruction::new(ADD, WORD, IMEDIATE_VALUE(vec![0, 0, 0xff, 0xfe]),
        MEMORY_ADDR(0));
    cpu.execute(&inst);
    assert_eq!(cpu.get_memory_offset(0, 2), Some(&(vec![0xff, 0xfe])[..]));
}

#[test]
fn test_add_right_result_long() {
    let mut cpu = CPU::default();
    let inst = Instruction::new(ADD, LONG, IMEDIATE_VALUE(vec![0x12, 0, 0xff, 0xfe]),
        MEMORY_ADDR(0));
    cpu.execute(&inst);
    assert_eq!(cpu.get_memory_offset(0, 4), Some(&(vec![0x12, 0, 0xff, 0xfe])[..]));
}

fn _test_add_correct_n_flag(size: OpSize) {
    let (v1, v2, v3) = _set_byte_on_vec(&size, 0x80, 0x10, 0x20);

    let mut cpu = CPU::default();
    let inst = Instruction::new(MOVE, size, IMEDIATE_VALUE(v1),
        DATA_REGISTER(1));
    cpu.execute(&inst);
    let inst = Instruction::new(ADD, size, IMEDIATE_VALUE(v2.to_vec()),
        DATA_REGISTER(1));
    cpu.execute(&inst);

    assert_eq!(cpu.get_ccr(), 0b00001000);

    let inst = Instruction::new(MOVE, size, IMEDIATE_VALUE(v2),
        DATA_REGISTER(1));
    cpu.execute(&inst);
    let inst = Instruction::new(ADD, size, IMEDIATE_VALUE(v3),
        DATA_REGISTER(1));
    cpu.execute(&inst);

    assert_eq!(cpu.get_ccr(), 0);    
}

#[test]
fn test_add_correct_n_flag() {
    _test_add_correct_n_flag(BYTE);
    _test_add_correct_n_flag(WORD);
    _test_add_correct_n_flag(LONG);
}

#[test]
fn test_add_correct_v_flag() {
    _check_add_v_flag(BYTE);
    _check_add_v_flag(WORD);
    _check_add_v_flag(LONG);
}

fn _test_carry_flags(size: OpSize) {
    let (v1, v2, v3) = _set_byte_on_vec(&size, 0xf0, 0x15, 0x20);

    let mut cpu = CPU::default();
    let inst = Instruction::new(MOVE, size, IMEDIATE_VALUE(v1),
        DATA_REGISTER(1));
    cpu.execute(&inst);
    let inst = Instruction::new(ADD, size, IMEDIATE_VALUE(v2),
        DATA_REGISTER(1));
    cpu.execute(&inst);

    assert_eq!(cpu.get_ccr(), 0b00010001);

    let inst = Instruction::new(ADD, size, IMEDIATE_VALUE(v3),
        DATA_REGISTER(1));
    cpu.execute(&inst);

    assert_eq!(cpu.get_ccr(), 0);
}

#[test]
fn test_add_carry_flags() {
    _test_carry_flags(BYTE);
    _test_carry_flags(WORD);
    _test_carry_flags(LONG);
}

fn _test_z_flag(size: OpSize) {
    let (v1, v2, v3) = _set_byte_on_vec(&size, 0xf0, 0x10, 0);

    let mut cpu = CPU::default();
    let inst = Instruction::new(MOVE, size, IMEDIATE_VALUE(v1),
        DATA_REGISTER(1));
    cpu.execute(&inst);
    let inst = Instruction::new(ADD, size, IMEDIATE_VALUE(v2),
        DATA_REGISTER(1));
    cpu.execute(&inst);

    assert_eq!(cpu.get_ccr(), 0b00010101);

    let inst = Instruction::new(ADD, size, IMEDIATE_VALUE(v3),
        DATA_REGISTER(1));
    cpu.execute(&inst);

    assert_eq!(cpu.get_ccr(), 0b00000100);
}

#[test]
fn test_add_z_flag() {
    _test_z_flag(BYTE);
    _test_z_flag(WORD);
    _test_z_flag(LONG);
}
