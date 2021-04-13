mod cpu;
use cpu::*;

fn main() {
    let cpu = CPU::default();
    cpu.foo();
}
