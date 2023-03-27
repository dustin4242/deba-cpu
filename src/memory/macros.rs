#[macro_export]
macro_rules! no_op {
    () => {
        vec![0]
    };
}
#[macro_export]
macro_rules! set_register {
    ($register: expr, $value: expr) => {
        vec![1, $register, $value]
    };
}
#[macro_export]
macro_rules! set_reg_to_mem {
    ($register:expr, $pos: expr) => {
        vec![2, $register, $pos]
    };
}
#[macro_export]
macro_rules! set_mem_to_reg {
    ($pos: expr, $register: expr) => {
        vec![3, $pos, $register]
    };
}
#[macro_export]
macro_rules! add {
    ($register_1: expr, $register_2: expr) => {
        vec![4, $register_1, $register_2]
    };
}
#[macro_export]
macro_rules! sub {
    ($register_1: expr, $register_2: expr) => {
        vec![5, $register_1, $register_2]
    };
}
#[macro_export]
macro_rules! jump_to {
    ($value: expr) => {
        vec![6, $value]
    };
}
#[macro_export]
macro_rules! jump_to_if_zero {
    ($value: expr) => {
        vec![7, $value]
    };
}
#[macro_export]
macro_rules! print {
    ($register: expr) => {
        vec![8, $register]
    };
}
#[macro_export]
macro_rules! halt {
    () => {
        vec![9]
    };
}
#[macro_export]
macro_rules! memory_init {
    ($($vec: expr),*) => {
        {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.append(&mut $vec);
        )*
        temp_vec
    }
    }
}
