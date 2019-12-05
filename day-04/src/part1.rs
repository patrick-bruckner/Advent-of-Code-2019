use std::fs::read_to_string;

fn does_not_decrease(digits: &Vec<u32>) -> bool
{
    let mut last_digit = 0;

    for digit in digits
    {
        if *digit < last_digit
        {
            return false;
        }
        else
        {
            last_digit = *digit;
        }
    }

    return true;
}

fn has_repeat(digits: &Vec<u32>) -> bool
{
    let mut last_digit = 0;

    for digit in digits
    {
        if *digit == last_digit
        {
            return true;
        }
        else
        {
            last_digit = *digit;
        }
    }

    return false;
}

pub fn part1()
{
    let mut input_string = read_to_string("input/part1.txt").unwrap();
    input_string = input_string.trim_end().to_string();

    let mut split_input = input_string.split('-');

    let begin_range = split_input.next().unwrap().parse::<u32>().unwrap();
    let end_range = split_input.next().unwrap().parse::<u32>().unwrap();

    let mut number_of_passwords = 0;

    for x in begin_range..(end_range+1)
    {
        let digits: Vec<u32> = x.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();

        if does_not_decrease(&digits) && has_repeat(&digits)
        {
            number_of_passwords += 1;
        }
    }

    println!("Number of possible passwords: {}", number_of_passwords);
}
