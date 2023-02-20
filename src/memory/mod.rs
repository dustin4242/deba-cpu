#[macro_use]
mod macros;

pub fn hello_world() -> Vec<u8> {
    memory_init![
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
    ]
}

pub fn for_loop() -> Vec<u8> {
    memory_init![
        set_b!(1),
        set_a_to_mem!(12),
        sub!(),
        set_mem_to_a!(12),
        jump_to_if_zero!(11),
        jump_to!(13),
        halt!(),
        vec![27],
        set_a!(97),
        print_char!(),
        add!(),
        set_mem_to_a!(14),
        jump_to!(2)
    ]
}
