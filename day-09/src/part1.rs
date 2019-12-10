use std::fs::read_to_string;
use crate::computer::{Computer, Program, IOQueue};

pub fn part1()
{
    let mut intcode_str = read_to_string("input/part1.txt").unwrap();
    intcode_str = intcode_str.trim_end().to_string();
    let program = Program::new_from_str(intcode_str);

    let mut computer = Computer::new();
    computer.load_program(&program);
    let computer_input = IOQueue::new();
    let computer_output = IOQueue::new();
    computer.set_input(&computer_input);
    computer.set_output(&computer_output);

    computer_input.push_front("1".to_string());
    computer.run();

    if !computer.is_halted()
    {
        panic!("Computer isn't finished");
    }

    for s in computer_output
    {
        println!("{}", s);
    }
}
