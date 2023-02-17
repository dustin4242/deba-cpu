mod cpu;
mod memory;
use cpu::DB8;

fn main() {
    let mut cpu = DB8::new();
    cpu.exec(memory::hello_world());
}
