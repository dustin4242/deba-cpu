#[macro_use]
mod macros;

pub fn hello_world() -> Vec<u8> {
    memory_init![
        set_register!(1, 1),
        set_reg_to_mem!(0, 22),
        jump_to_if_zero!(21),
        print!(0),
        set_reg_to_mem!(0, 5),
        add!(0, 1),
        set_mem_to_reg!(5, 0),
        jump_to!(3),
        halt!(),
        vec![
            72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33, // "Hello World!"
            0   // End Of String
        ]
    ]
}
