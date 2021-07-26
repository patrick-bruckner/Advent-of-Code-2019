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
    RelativeBaseOffset,
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
            "09" => Some(Self::RelativeBaseOffset),
            "99" => Some(Self::Halt),
            _ => None
        }
    }
}

#[derive(Debug)]
enum ParameterMode
{
    Position,
    Immediate,
    Relative
}

impl ParameterMode
{
    fn from_char(c: char) -> Option<Self>
    {
        match c
        {
            '0' => Some(Self::Position),
            '1' => Some(Self::Immediate),
            '2' => Some(Self::Relative),
            _ => None
        }
    }
}

#[derive(Debug)]
enum Opcode
{
    Add(ParameterMode, ParameterMode, ParameterMode),
    Mul(ParameterMode, ParameterMode, ParameterMode),
    Input(ParameterMode),
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode, ParameterMode),
    RelativeBaseOffset(ParameterMode),
    Halt
}

impl Opcode
{
    fn from_intcode(intcode: i64) -> Option<Self>
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
        let param_3_type = match ParameterMode::from_char(chars[0])
        {
            Some(p) => p,
            None => return None
        };

        match opcode_type
        {
            OpcodeType::Add => Some(Self::Add(param_1_type, param_2_type, param_3_type)),
            OpcodeType::Mul => Some(Self::Mul(param_1_type, param_2_type, param_3_type)),
            OpcodeType::Input => Some(Self::Input(param_1_type)),
            OpcodeType::Output => Some(Self::Output(param_1_type)),
            OpcodeType::JumpIfTrue => Some(Self::JumpIfTrue(param_1_type, param_2_type)),
            OpcodeType::JumpIfFalse => Some(Self::JumpIfFalse(param_1_type, param_2_type)),
            OpcodeType::LessThan => Some(Self::LessThan(param_1_type, param_2_type, param_3_type)),
            OpcodeType::Equals => Some(Self::Equals(param_1_type, param_2_type, param_3_type)),
            OpcodeType::RelativeBaseOffset => Some(Self::RelativeBaseOffset(param_1_type)),
            OpcodeType::Halt => Some(Self::Halt)
        }
    }
}

#[derive(Clone)]
pub struct Program
{
    memory: Vec<i64>,
}

impl Program
{
    pub fn new() -> Self
    {
        Self
        {
            memory: Vec::<i64>::new()
        }
    }

    pub fn new_from_str(initial_setup: String) -> Self
    {
        let mut intcode = Vec::<i64>::new();
        for value in initial_setup.split(',')
        {
            intcode.push(value.parse::<i64>().unwrap());
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
    type Output = i64;

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

pub struct Computer<'a>
{
    memory: Program,
    instruction_pointer: usize,
    relative_base_offset: usize,
    input_fn: Option<Box<dyn Fn() -> i64 + 'a>>,
    output_fn: Option<Box<dyn FnMut(i64) + 'a>>
}

impl<'a> Computer<'a>
{
    pub fn new() -> Self
    {
        Self
        {
            memory: Program::new(),
            instruction_pointer: 0,
            relative_base_offset: 0,
            input_fn: None,
            output_fn: None
        }
    }

    pub fn load_program(&mut self, program: &Program)
    {
        self.memory = program.clone();
        self.instruction_pointer = 0;
    }

    #[allow(dead_code)]
    pub fn set_input<T: Fn() -> i64 + 'a>(&mut self, input_fn: T)
    {
        self.input_fn = Some(Box::new(input_fn));
    }

    #[allow(dead_code)]
    pub fn set_output<T: FnMut(i64) + 'a>(&mut self, output_fnt: T)
    {
        self.output_fn = Some(Box::new(output_fnt));
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
            Some(s) => {
                // println!("{:?}", s);
                match s
                {
                    Opcode::Add(a, b, c) => self.preform_add(a, b, c),
                    Opcode::Mul(a, b, c) => self.preform_nul(a, b, c),
                    Opcode::Input(a) => if !self.get_input(a) {return true},
                    Opcode::Output(s) => self.write_output(s),
                    Opcode::JumpIfTrue(z, d) => self.jump_if_true(z, d),
                    Opcode::JumpIfFalse(z, d) => self.jump_if_false(z, d),
                    Opcode::LessThan(a, b, c) => self.less_than(a, b, c),
                    Opcode::Equals(a, b, c) => self.equals(a, b, c),
                    Opcode::RelativeBaseOffset(a) => self.adjust_relative_base_offset(a),
                    Opcode::Halt => return true
                }
            },
            None => panic!("Invalid Opcode")
        }

        return false;
    }

    fn get_param_value_interpreted(&mut self, pm: ParameterMode, p: usize) -> i64
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
                    self.memory.memory.resize((address + 1) as usize, 0);
                }

                self.memory[address as usize]
            },
            ParameterMode::Immediate => self.memory[p],
            ParameterMode::Relative => {
                let address = self.memory[p] + (self.relative_base_offset as i64);

                if address < 0
                {
                    panic!("Malformed program - address out of bounds");
                }

                if address as usize >= self.memory.len()
                {
                    self.memory.memory.resize((address + 1) as usize, 0);
                }

                self.memory[address as usize]
            }
        }
    }

    fn get_param_value_literal(&mut self, pm: ParameterMode, p: usize) -> i64
    {
        match pm
        {
            ParameterMode::Position => self.get_param_value_interpreted(ParameterMode::Immediate, p),
            ParameterMode::Immediate => self.get_param_value_interpreted(ParameterMode::Immediate, p),
            ParameterMode::Relative => {
                let address = self.memory[p] + (self.relative_base_offset as i64);

                if address < 0
                {
                    panic!("Malformed program - address out of bounds");
                }

                if address as usize >= self.memory.len()
                {
                    self.memory.memory.resize((address + 1) as usize, 0);
                }

                return address;
            }
        }
    }

    fn preform_add(&mut self, a: ParameterMode, b: ParameterMode, c: ParameterMode)
    {
        if self.memory.len() - self.instruction_pointer < 4
        {
            panic!("Malformed program - Add opcode too short");
        }

        let arg1 = self.get_param_value_interpreted(a, self.instruction_pointer + 1);
        let arg2 = self.get_param_value_interpreted(b, self.instruction_pointer + 2);
        let result_address = self.get_param_value_literal(c, self.instruction_pointer + 3);

        if result_address < 0
        {
            panic!("Malformed program - address out of pounds");
        }

        if (result_address as usize) >= self.memory.len()
        {
            self.memory.memory.resize((result_address + 1) as usize, 0);
        }

        self.memory[result_address as usize] = arg1 + arg2;

        self.instruction_pointer += 4;
    }

    fn preform_nul(&mut self, a: ParameterMode, b: ParameterMode, c: ParameterMode)
    {
        if self.memory.len() - self.instruction_pointer < 4
        {
            panic!("Malformed program - Mul opcode too short");
        }

        let arg1 = self.get_param_value_interpreted(a, self.instruction_pointer + 1);
        let arg2 = self.get_param_value_interpreted(b, self.instruction_pointer + 2);
        let result_address = self.get_param_value_literal(c, self.instruction_pointer + 3);

        if result_address < 0
        {
            panic!("Malformed program - address out of pounds");
        }

        if (result_address as usize) >= self.memory.len()
        {
            self.memory.memory.resize((result_address + 1) as usize, 0);
        }

        self.memory[result_address as usize] = arg1 * arg2;

        self.instruction_pointer += 4;
    }

    fn get_input(&mut self, a: ParameterMode) -> bool
    {
        if self.memory.len() - self.instruction_pointer < 2
        {
            panic!("Malformed program - Input opcode too short");
        }

        let destination_address = self.get_param_value_literal(a, self.instruction_pointer + 1);

        if destination_address < 0
        {
            panic!("Malformed program - address out of pounds");
        }

        if (destination_address as usize) >= self.memory.len()
        {
            self.memory.memory.resize((destination_address + 1) as usize, 0);
        }

        if let Some(input_f) = &self.input_fn
        {
            let number = input_f();
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

        let output = self.get_param_value_interpreted(pm, self.instruction_pointer + 1);

        if let Some(output_f) = &mut self.output_fn {
            output_f(output);
        }

        self.instruction_pointer += 2;
    }

    fn jump_if_true(&mut self, z: ParameterMode, d: ParameterMode)
    {
        if self.memory.len() - self.instruction_pointer < 3
        {
            panic!("Malformed program - JumpIfTrue opcode too short");
        }

        let arg1 = self.get_param_value_interpreted(z, self.instruction_pointer + 1);
        let arg2 = self.get_param_value_interpreted(d, self.instruction_pointer + 2);

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

        let arg1 = self.get_param_value_interpreted(z, self.instruction_pointer + 1);
        let arg2 = self.get_param_value_interpreted(d, self.instruction_pointer + 2);

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

    fn less_than(&mut self, a: ParameterMode, b: ParameterMode, c: ParameterMode)
    {
        if self.memory.len() - self.instruction_pointer < 4
        {
            panic!("Malformed program - LessThan opcode too short");
        }

        let arg1 = self.get_param_value_interpreted(a, self.instruction_pointer + 1);
        let arg2 = self.get_param_value_interpreted(b, self.instruction_pointer + 2);
        let result_address = self.get_param_value_literal(c, self.instruction_pointer + 3);

        if result_address < 0
        {
            panic!("Malformed program - address out of pounds");
        }

        if (result_address as usize) >= self.memory.len()
        {
            self.memory.memory.resize((result_address + 1) as usize, 0);
        }

        self.memory[result_address as usize] = if arg1 < arg2 {1} else {0};

        self.instruction_pointer += 4;
    }

    fn equals(&mut self, a: ParameterMode, b: ParameterMode, c: ParameterMode)
    {
        if self.memory.len() - self.instruction_pointer < 4
        {
            panic!("Malformed program - Equals opcode too short");
        }

        let arg1 = self.get_param_value_interpreted(a, self.instruction_pointer + 1);
        let arg2 = self.get_param_value_interpreted(b, self.instruction_pointer + 2);
        let result_address = self.get_param_value_literal(c, self.instruction_pointer + 3);

        if result_address < 0
        {
            panic!("Malformed program - address out of pounds");
        }

        if (result_address as usize) >= self.memory.len()
        {
            self.memory.memory.resize((result_address + 1) as usize, 0);
        }

        self.memory[result_address as usize] = if arg1 == arg2 {1} else {0};

        self.instruction_pointer += 4;
    }

    fn adjust_relative_base_offset(&mut self, a: ParameterMode)
    {
        if self.memory.len() - self.instruction_pointer < 2
        {
            panic!("Malformed program - RelativeBaseOffset opcode too short");
        }

        let arg1 = self.get_param_value_interpreted(a, self.instruction_pointer + 1);

        let new_value = (self.relative_base_offset as i64) + arg1;
        if new_value < 0
        {
            panic!("Malformed program - RelativeBaseOffset can only be set to positive numbers");
        }

        self.relative_base_offset = new_value as usize;

        self.instruction_pointer += 2;
    }

    #[allow(dead_code)]
    pub fn get_value(&self, address: usize) -> i64
    {
        return self.memory[address];
    }

    #[allow(dead_code)]
    pub fn set_value(&mut self, address: usize, val: i64)
    {
        self.memory[address] = val;
    }
}
