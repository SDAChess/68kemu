mod cpu;
#[cfg(test)]
mod test;

use cpu::*;

fn main() {
    let mut cpu = CPU::default();
    cpu.foo();
}
