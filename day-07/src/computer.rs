use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

pub type IOQueue = Rc<RefCell<VecDeque<String>>>;

enum OpcodeType
{
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt
}

impl OpcodeType
{
    fn from_str(s: &str) -> Option<Self>
    {
        match s.as_ref()
        {
            "01" => Some(Self::Add),
            "02" => Some(Self::Mul),
            "03" => Some(Self::Input),
            "04" => Some(Self::Output),
            "05" => Some(Self::JumpIfTrue),
            "06" => Some(Self::JumpIfFalse),
            "07" => Some(Self::LessThan),
            "08" => Some(Self::Equals),
            "99" => Some(Self::Halt),
            _ => None
        }
    }
}

enum ParameterMode
{
    Position,
    Immediate
}

impl ParameterMode
{
    fn from_char(c: char) -> Option<Self>
    {
        match c
        {
            '0' => Some(Self::Position),
            '1' => Some(Self::Immediate),
            _ => None
        }
    }
}

enum Opcode
{
    Add(ParameterMode, ParameterMode),
    Mul(ParameterMode, ParameterMode),
    Input,
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode),
    Halt
}

impl Opcode
{
    fn from_intcode(intcode: i32) -> Option<Self>
    {
        if intcode < 0
        {
            panic!("Invalid Opcode");
        }

        let chars: Vec<_> = format!("{:05}", intcode).chars().collect();

        let opcode_string: String = chars[3..].into_iter().collect();

        let opcode_type = match OpcodeType::from_str(opcode_string.as_ref())
        {
            Some(o) => o,
            None => return None
        };

        let param_1_type = match ParameterMode::from_char(chars[2])
        {
            Some(p) => p,
            None => return None
        };
        let param_2_type = match ParameterMode::from_char(chars[1])
        {
            Some(p) => p,
            None => return None
        };

        match opcode_type
        {
            OpcodeType::Add => Some(Self::Add(param_1_type, param_2_type)),
            OpcodeType::Mul => Some(Self::Mul(param_1_type, param_2_type)),
            OpcodeType::Input => Some(Self::Input),
            OpcodeType::Output => Some(Self::Output(param_1_type)),
            OpcodeType::JumpIfTrue => Some(Self::JumpIfTrue(param_1_type, param_2_type)),
            OpcodeType::JumpIfFalse => Some(Self::JumpIfFalse(param_1_type, param_2_type)),
            OpcodeType::LessThan => Some(Self::LessThan(param_1_type, param_2_type)),
            OpcodeType::Equals => Some(Self::Equals(param_1_type, param_2_type)),
            OpcodeType::Halt => Some(Self::Halt)
        }
    }
}

#[derive(Clone)]
pub struct Program
{
    memory: Vec<i32>,
}

impl Program
{
    pub fn new() -> Self
    {
        Self
        {
            memory: Vec::<i32>::new()
        }
    }

    pub fn new_from_str(initial_setup: String) -> Self
    {
        let mut intcode = Vec::<i32>::new();
        for value in initial_setup.split(',')
        {
            intcode.push(value.parse::<i32>().unwrap());
        }

        Self
        {
            memory: intcode
        }
    }

    pub fn len(&self) -> usize
    {
        return self.memory.len();
    }
}

impl std::ops::Index<usize> for Program
{
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output
    {
        return &self.memory[index];
    }
}

impl std::ops::IndexMut<usize> for Program
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output
    {
        return &mut self.memory[index];
    }
}

pub struct Computer
{
    memory: Program,
    instruction_pointer: usize,
    input: IOQueue,
    output: IOQueue
}

impl Computer
{
    pub fn new() -> Self
    {
        Self
        {
            memory: Program::new(),
            instruction_pointer: 0,
            input: Rc::new(RefCell::new(VecDeque::new())),
            output: Rc::new(RefCell::new(VecDeque::new()))
        }
    }

    pub fn load_program(&mut self, program: &Program)
    {
        self.memory = program.clone();
        self.instruction_pointer = 0;
    }

    pub fn set_input(&mut self, input: &IOQueue)
    {
        self.input = Rc::clone(input);
    }

    pub fn set_output(&mut self, output: &IOQueue)
    {
        self.output = Rc::clone(output);
    }

    pub fn is_halted(&self) -> bool
    {
        if self.memory.len() == 0
        {
            return true;
        }
        else
        {
            return match Opcode::from_intcode(self.memory[self.instruction_pointer])
            {
                Some(Opcode::Halt) => true,
                _ => false
            };
        }
    }

    pub fn run(&mut self)
    {
        loop
        {
            match self.is_halted()
            {
                true => break,
                false => if self.step() {return}
            }
        }
    }

    pub fn step(&mut self) -> bool
    {
        if self.memory.len() == 0
        {
            return true;
        }

        match Opcode::from_intcode(self.memory[self.instruction_pointer])
        {
            Some(s) => match s
            {
                Opcode::Add(a, b) => self.preform_add(a, b),
                Opcode::Mul(a, b) => self.preform_nul(a, b),
                Opcode::Input => if !self.get_input() {return true},
                Opcode::Output(s) => self.write_output(s),
                Opcode::JumpIfTrue(z, d) => self.jump_if_true(z, d),
                Opcode::JumpIfFalse(z, d) => self.jump_if_false(z, d),
                Opcode::LessThan(a, b) => self.less_than(a, b),
                Opcode::Equals(a, b) => self.equals(a, b),
                Opcode::Halt => return true
            },
            None => panic!("Invalid Opcode")
        }

        return false;
    }

    fn get_param_value(&self, pm: ParameterMode, p: usize) -> i32
    {
        match pm
        {
            ParameterMode::Position => {
                let address = self.memory[p];
                if address < 0
                {
                    panic!("Invalid Opcode");
                }
                else if (address as usize) >= self.memory.len()
                {
                    panic!("Malformed program - address out of pounds");
                }
                else
                {
                    self.memory[address as usize]
                }
            },
            ParameterMode::Immediate => self.memory[p]
        }
    }

    fn preform_add(&mut self, a: ParameterMode, b: ParameterMode)
    {
        if self.memory.len() - self.instruction_pointer < 4
        {
            panic!("Malformed program - Add opcode too short");
        }

        let arg1 = self.get_param_value(a, self.instruction_pointer + 1);
        let arg2 = self.get_param_value(b, self.instruction_pointer + 2);
        let result_address = self.memory[self.instruction_pointer + 3];

        if result_address < 0
        {
            panic!("Malformed program - address out of pounds");
        }

        if (result_address as usize) >= self.memory.len()
        {
            panic!("Malformed program - address out of pounds");
        }

        self.memory[result_address as usize] = arg1 + arg2;

        self.instruction_pointer += 4;
    }

    fn preform_nul(&mut self, a: ParameterMode, b: ParameterMode)
    {
        if self.memory.len() - self.instruction_pointer < 4
        {
            panic!("Malformed program - Mul opcode too short");
        }

        let arg1 = self.get_param_value(a, self.instruction_pointer + 1);
        let arg2 = self.get_param_value(b, self.instruction_pointer + 2);
        let result_address = self.memory[self.instruction_pointer + 3];

        if result_address < 0
        {
            panic!("Malformed program - address out of pounds");
        }

        if (result_address as usize) >= self.memory.len()
        {
            panic!("Malformed program - address out of pounds");
        }

        self.memory[result_address as usize] = arg1 * arg2;

        self.instruction_pointer += 4;
    }

    fn get_input(&mut self) -> bool
    {
        if self.memory.len() - self.instruction_pointer < 2
        {
            panic!("Malformed program - Input opcode too short");
        }

        let destination_address = self.memory[self.instruction_pointer + 1];

        if destination_address < 0
        {
            panic!("Malformed program - address out of pounds");
        }

        if (destination_address as usize) >= self.memory.len()
        {
            panic!("Malformed program - address out of pounds");
        }

        if let Some(input) = (*self.input).borrow_mut().pop_back()
        {
            let number = input.parse::<i32>().unwrap();
            self.memory[destination_address as usize] = number;

            self.instruction_pointer += 2;

            return true;
        }
        else
        {
            return false;
        }
    }

    fn write_output(&mut self, pm: ParameterMode)
    {
        if self.memory.len() - self.instruction_pointer < 2
        {
            panic!("Malformed program - Input opcode too short");
        }

        let output = self.get_param_value(pm, self.instruction_pointer + 1);

        (*self.output).borrow_mut().push_front(output.to_string());

        self.instruction_pointer += 2;
    }

    fn jump_if_true(&mut self, z: ParameterMode, d: ParameterMode)
    {
        if self.memory.len() - self.instruction_pointer < 3
        {
            panic!("Malformed program - JumpIfTrue opcode too short");
        }

        let arg1 = self.get_param_value(z, self.instruction_pointer + 1);
        let arg2 = self.get_param_value(d, self.instruction_pointer + 2);

        if arg2 < 0
        {
            panic!("Malformed program - address out of pounds");
        }

        if arg1 != 0
        {
            self.instruction_pointer = arg2 as usize;
        }
        else
        {
            self.instruction_pointer += 3;
        }
    }

    fn jump_if_false(&mut self, z: ParameterMode, d: ParameterMode)
    {
        if self.memory.len() - self.instruction_pointer < 3
        {
            panic!("Malformed program - JumpIfFalse opcode too short");
        }

        let arg1 = self.get_param_value(z, self.instruction_pointer + 1);
        let arg2 = self.get_param_value(d, self.instruction_pointer + 2);

        if arg2 < 0
        {
            panic!("Malformed program - address out of pounds");
        }

        if arg1 == 0
        {
            self.instruction_pointer = arg2 as usize;
        }
        else
        {
            self.instruction_pointer += 3;
        }
    }

    fn less_than(&mut self, a: ParameterMode, b: ParameterMode)
    {
        if self.memory.len() - self.instruction_pointer < 4
        {
            panic!("Malformed program - Mul opcode too short");
        }

        let arg1 = self.get_param_value(a, self.instruction_pointer + 1);
        let arg2 = self.get_param_value(b, self.instruction_pointer + 2);
        let result_address = self.memory[self.instruction_pointer + 3];

        if result_address < 0
        {
            panic!("Malformed program - address out of pounds");
        }

        if (result_address as usize) >= self.memory.len()
        {
            panic!("Malformed program - address out of pounds");
        }

        self.memory[result_address as usize] = if arg1 < arg2 {1} else {0};

        self.instruction_pointer += 4;
    }

    fn equals(&mut self, a: ParameterMode, b: ParameterMode)
    {
        if self.memory.len() - self.instruction_pointer < 4
        {
            panic!("Malformed program - Mul opcode too short");
        }

        let arg1 = self.get_param_value(a, self.instruction_pointer + 1);
        let arg2 = self.get_param_value(b, self.instruction_pointer + 2);
        let result_address = self.memory[self.instruction_pointer + 3];

        if result_address < 0
        {
            panic!("Malformed program - address out of pounds");
        }

        if (result_address as usize) >= self.memory.len()
        {
            panic!("Malformed program - address out of pounds");
        }

        self.memory[result_address as usize] = if arg1 == arg2 {1} else {0};

        self.instruction_pointer += 4;
    }

    #[allow(dead_code)]
    pub fn get_value(&self, address: usize) -> i32
    {
        return self.memory[address];
    }
}
