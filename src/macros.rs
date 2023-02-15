#[macro_export]
macro_rules! no_op {
    () => {
        vec![0]
    };
}
#[macro_export]
macro_rules! set_a {
    ($value: expr) => {
        vec![1, $value]
    };
}
#[macro_export]
macro_rules! set_b {
    ($value: expr) => {
        vec![2, $value]
    };
}
#[macro_export]
macro_rules! set_a_to_mem {
    ($value: expr) => {
        vec![3, $value]
    };
}
#[macro_export]
macro_rules! set_mem_to_a {
    ($value: expr) => {
        vec![4, $value]
    };
}
#[macro_export]
macro_rules! add {
    () => {
        vec![5]
    };
}
#[macro_export]
macro_rules! sub {
    () => {
        vec![6]
    };
}
#[macro_export]
macro_rules! jump_to {
    ($value: expr) => {
        vec![7, $value]
    };
}
#[macro_export]
macro_rules! jump_to_if_zero {
    ($value: expr) => {
        vec![8, $value]
    };
}
#[macro_export]
macro_rules! print {
    () => {
        vec![9]
    };
}
#[macro_export]
macro_rules! print_char {
    () => {
        vec![10]
    };
}
#[macro_export]
macro_rules! halt {
    () => {
        vec![11]
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
