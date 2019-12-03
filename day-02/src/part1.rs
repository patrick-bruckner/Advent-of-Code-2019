use std::fs::read_to_string;
use crate::program::Program;

fn substitute(input_str: String) -> String
{
    let mut intcode: Vec<&str> = input_str.split(',').collect();
    intcode[1] = "12";
    intcode[2] = "2";

    return intcode.join(",");
}

pub fn part1()
{
    let mut intcode_str = read_to_string("input/part1.txt").unwrap();
    intcode_str = intcode_str.trim_end().to_string();

    intcode_str = substitute(intcode_str);

    let mut program = Program::new_from_str(intcode_str);
    program.run();

    println!("Value at address 0: {}", program.get_value(0));
}
