# Deba Cpu Emulator
---
This is a idea for a project that i've had for a very long time and I've finally made a emulator for it.

## How to initialize memory:
Every Deba cpu has a memory sequence, and the way we initialize said memory is using the `memory_init![]` macro inside of the `src/memory/mod.rs` file.
This macro takes in `Vec<u8>` lists and concats each of them to make a final number sequence that passes into the cpu.

**Ex:**
```rust
pub fn example() -> Vec<u8> {
	memory_init![
		vec![11], // Halt Instruction
	]
}
```
***Note:*** *make sure memory is mutable for the `set_mem_to_a!()` instuction*

## How to use instructions:
Now we can get into the fun part: **Programming**. Programming a Deba cpu's memory is relatively easy however it can be easy to get lost in the code since it is in a assembly-like syntax. To make programming the memory easier I have provided many macros for you guys to use:
```
set_a!(value)
set_b!(value)
set_a_to_mem!(mem_pos)
set_mem_to_a!(mem_pos)
add!()
sub!()
jump_to!(mem_pos)
jump_to_if_zero!(mem_pos)
print!()
print_char!()
halt!()
```
***Notes:***
- `value` & `mem_pos` is of type `u8` to respect the memory's layout
- `add` & `subtract` respectively use `a + b` & `a - b`
- `jump_to_if_zero!`, `print!`, & `print_char!` are respective to the a register's value

To use these macros and easily set the memory just follow the layout below:
```rust
pub fn example() -> Vec<u8> {
	memory_init![
		set_a!(119), // Ascii Char "w"
		set_b!(32),
		sub!(), // Will subtract B Register from A Register
		print_char!(), // Will print "W" in A Register
	]
}
```

## How to execute memory:
We now need to initialize a new cpu to execute this memory we set and we can easily do that by using the Deba type in our `src/main.rs` file:
```rust
let mut cpu = DB8::new();
```

And then pass in the memory function to the `exec()` function on the cpu:
```rust
cpu.exec(memory::example());
```

And that's it if you want the simple functionality of this cpu.

## Extra Notes:
- Just because we are using macros doesnt mean that it doesnt expand into numbers itself, make sure you keep that in mind when using any macros that use `mem_pos` stated above. Ex: 
```rust
let x: Vec<u8> = set_a!(value);
// expands into: 
let x: Vec<u8> = vec![1, value];
```

Happy Coding! - Dustin
