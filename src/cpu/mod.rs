pub struct DB8 {
    registers: [u8; 2],
}

impl DB8 {
    // Usable Functions
    pub fn new() -> DB8 {
        DB8 { registers: [0, 0] }
    }
    pub fn exec(&mut self, mut memory: Vec<u8>) {
        let mut i = 0;
        while i < memory.len() {
            match memory[i] {
                0 => (),
                1 => {
                    self.set_register(memory[i + 1] as usize, memory[i + 2]);
                    i += 2;
                }
                2 => {
                    self.set_reg_to_mem(memory[i + 1], &memory, memory[i + 2]);
                    i += 2;
                }
                3 => {
                    let position = memory[i + 1];
                    let register = memory[i + 2];
                    self.set_mem_to_reg(&mut memory, position, register);
                    i += 2;
                }
                4 => {
                    self.add(memory[i + 1], memory[i + 2]);
                    i += 2;
                }
                5 => {
                    self.sub(memory[i + 1], memory[i + 2]);
                    i += 2;
                }
                6 => i = self.jump_to(memory[i + 1]),
                7 => i = self.jump_to_if_zero(memory[i + 1], i, 0),
                8 => {
                    self.print(memory[i + 1] as usize);
                    i += 1;
                }
                9 => return,
                _ => panic!("{}", memory[i]),
            }
            i += 1;
        }
    }

    // Instruction Functions
    fn set_register(&mut self, register: usize, value: u8) {
        self.registers[register] = value
    }
    fn set_reg_to_mem(&mut self, register: u8, memory: &Vec<u8>, position: u8) {
        self.registers[register as usize] = memory[position as usize];
    }
    fn set_mem_to_reg(&mut self, memory: &mut Vec<u8>, position: u8, register: u8) {
        memory[position as usize] = self.registers[register as usize];
    }
    fn add(&mut self, register_1: u8, register_2: u8) {
        self.registers[register_1 as usize] =
            self.registers[register_1 as usize] + self.registers[register_2 as usize]
    }
    fn sub(&mut self, register_1: u8, register_2: u8) {
        self.registers[register_1 as usize] =
            self.registers[register_1 as usize] - self.registers[register_2 as usize]
    }
    fn jump_to(&mut self, position: u8) -> usize {
        position as usize - 1
    }
    fn jump_to_if_zero(&mut self, position: u8, current_position: usize, register: usize) -> usize {
        if self.registers[register] == 0 {
            return position as usize - 1;
        }
        current_position + 1
    }
    fn print(&mut self, register: usize) {
        print!("{}", char::from(self.registers[register]));
    }
}
