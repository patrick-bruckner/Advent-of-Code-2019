use std::fs::File;
use std::io::{BufRead, BufReader};

struct Fuel
{
    amount: i32
}

impl Fuel
{
    fn new() -> Self
    {
        Self
        {
            amount: 0
        }
    }

    fn add_module(&mut self, mass: i32)
    {
        let additional_fuel = (mass as f32 / 3.0).floor() as i32 - 2;
        if additional_fuel > 0
        {
            self.amount += additional_fuel;
            self.add_module(additional_fuel);
        }
    }
}

pub fn part2()
{
    let file = File::open("input/part1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut required_fuel = Fuel::new();

    for line in reader.lines()
    {
        match line
        {
            Ok(s) => required_fuel.add_module(s.parse::<i32>().unwrap()),
            _ => panic!("Failed to read file")
        }
    }

    println!("Required Fuel: {}", required_fuel.amount);
}
