mod cpu;
mod macros;
use cpu::DB8;

fn main() {
    let memory: Vec<u8> = memory_init![
        set_b!(1),
        set_a_to_mem!(15),
        jump_to_if_zero!(14),
        print_char!(),
        set_a_to_mem!(3),
        add!(),
        set_mem_to_a!(3),
        jump_to!(2),
        halt!(),
        vec![
            72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33, // "Hello World!"
            0   // End Of String
        ]
    ];
    let mut cpu = DB8::new();

    cpu.exec(memory);
}
