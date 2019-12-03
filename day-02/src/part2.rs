use std::fs::read_to_string;
use crate::program::Program;

struct NounVerb
{
    noun: i32,
    verb: i32
}

fn substitute(input_str: &String, n: i32, v: i32) -> String
{
    let mut intcode: Vec<String> = input_str.split(',').map(|s| s.to_string()).collect();
    intcode[1] = n.to_string();
    intcode[2] = v.to_string();

    return intcode.join(",");
}

fn find_noun_verb(initial_memory_str: String) -> NounVerb
{
    for n in 0..100
    {
        for v in 0..100
        {
            let modified_memory_str = substitute(&initial_memory_str, n, v);
            let mut program = Program::new_from_str(modified_memory_str);
            program.run();

            if program.get_value(0) == 19690720
            {
                return NounVerb
                {
                    noun: n,
                    verb: v
                }
            }
        }
    }

    panic!("Noun and verb couldn't be found");
}


pub fn part2()
{
    let mut intcode_str = read_to_string("input/part1.txt").unwrap();
    intcode_str = intcode_str.trim_end().to_string();

    let noun_verb = find_noun_verb(intcode_str);

    println!("Noun: {}, Verb: {}, Answer: {}",
             noun_verb.noun,
             noun_verb.verb,
             100 * noun_verb.noun + noun_verb.verb);
}
