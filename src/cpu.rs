pub struct DB8 {
    register_a: u8,
    register_b: u8,
}

impl DB8 {
    // Usable Functions
    pub fn new() -> DB8 {
        DB8 {
            register_a: 0,
            register_b: 0,
        }
    }
    pub fn exec(&mut self, mut memory: Vec<u8>) {
        let mut i = 0;
        while i < memory.len() {
            match memory[i] {
                0 => (),
                1 => {
                    self.set_a(memory[i + 1]);
                    i += 1;
                }
                2 => {
                    self.set_b(memory[i + 1]);
                    i += 1;
                }
                3 => {
                    self.set_a_to_mem(&memory, memory[i + 1]);
                    i += 1;
                }
                4 => {
                    let position = memory[i + 1];
                    self.set_mem_to_a(&mut memory, position);
                    i += 1;
                }
                5 => self.add(),
                6 => self.sub(),
                7 => i = self.jump_to(memory[i + 1]),
                8 => i = self.jump_to_if_zero(memory[i + 1], i),
                9 => self.print(),
                10 => self.print_char(),
                11 => return,
                _ => panic!("{}", memory[i]),
            }
            i += 1;
        }
    }

    // Instruction Functions
    fn set_a(&mut self, value: u8) {
        self.register_a = value
    }
    fn set_b(&mut self, value: u8) {
        self.register_b = value
    }
    fn set_a_to_mem(&mut self, memory: &Vec<u8>, position: u8) {
        self.register_a = memory[position as usize];
    }
    fn set_mem_to_a(&mut self, memory: &mut Vec<u8>, position: u8) {
        memory[position as usize] = self.register_a;
    }
    fn add(&mut self) {
        self.register_a = self.register_a + self.register_b
    }
    fn sub(&mut self) {
        self.register_a = self.register_a - self.register_b
    }
    fn jump_to(&mut self, position: u8) -> usize {
        position as usize - 1
    }
    fn jump_to_if_zero(&mut self, position: u8, current_position: usize) -> usize {
        if self.register_a == 0 {
            return position as usize - 1;
        }
        current_position + 1
    }
    fn print(&mut self) {
        print!("{}", self.register_a)
    }
    fn print_char(&mut self) {
        print!("{}", char::from(self.register_a));
    }
}
