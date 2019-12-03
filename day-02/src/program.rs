use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
enum Opcode
{
    Add = 1,
    Mul = 2,
    Halt = 99
}

pub struct Program
{
    memory: Vec<i32>,
    instruction_pointer: usize
}

impl Program
{
    pub fn new_from_str(initial_setup: String) -> Self
    {
        let mut intcode = Vec::<i32>::new();
        for value in initial_setup.split(',')
        {
            intcode.push(value.parse::<i32>().unwrap());
        }

        Self
        {
            memory: intcode,
            instruction_pointer: 0
        }
    }

    pub fn run(&mut self)
    {
        loop
        {
            match FromPrimitive::from_i32(self.memory[self.instruction_pointer])
            {
                Some(Opcode::Halt) => break,
                _ => self.step()
            }
        }
    }

    pub fn step(&mut self)
    {
        match FromPrimitive::from_i32(self.memory[self.instruction_pointer])
        {
            Some(Opcode::Add) => self.preform_add(),
            Some(Opcode::Mul) => self.preform_nul(),
            Some(Opcode::Halt) => return,
            None => panic!("Invalid opcode: {} at address: {}", self.memory[self.instruction_pointer], self.instruction_pointer)
        }
    }

    fn preform_add(&mut self)
    {
        if self.memory.len() - self.instruction_pointer < 4
        {
            panic!("Malformed program - Add opcode too short");
        }

        let arg1_pos = self.memory[self.instruction_pointer + 1] as usize;
        let arg2_pos = self.memory[self.instruction_pointer + 2] as usize;
        let result_pos = self.memory[self.instruction_pointer + 3] as usize;

        if (arg1_pos >= self.memory.len()) || (arg2_pos >= self.memory.len()) ||
           (result_pos >= self.memory.len())
        {
            panic!("Malformed program - address out of pounds");
        }

        self.memory[result_pos] = self.memory[arg1_pos] + self.memory[arg2_pos];

        self.instruction_pointer += 4;
    }

    fn preform_nul(&mut self)
    {
        if self.memory.len() - self.instruction_pointer < 4
        {
            panic!("Malformed program - Mul opcode too short");
        }

        let arg1_pos = self.memory[self.instruction_pointer + 1] as usize;
        let arg2_pos = self.memory[self.instruction_pointer + 2] as usize;
        let result_pos = self.memory[self.instruction_pointer + 3] as usize;

        if (arg1_pos >= self.memory.len()) || (arg2_pos >= self.memory.len()) ||
           (result_pos >= self.memory.len())
        {
            panic!("Malformed program - address out of pounds");
        }

        self.memory[result_pos] = self.memory[arg1_pos] * self.memory[arg2_pos];

        self.instruction_pointer += 4;
    }

    pub fn get_value(&self, address: usize) -> i32
    {
        return self.memory[address];
    }
}
