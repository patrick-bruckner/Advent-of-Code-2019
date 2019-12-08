use std::fs::read_to_string;
use crate::computer;
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

use permutohedron::Heap;

pub fn part1()
{
    let mut intcode_str = read_to_string("input/part1.txt").unwrap();
    intcode_str = intcode_str.trim_end().to_string();
    let program = computer::Program::new_from_str(intcode_str);

    let mut amp_a = computer::Computer::new();
    let mut amp_b = computer::Computer::new();
    let mut amp_c = computer::Computer::new();
    let mut amp_d = computer::Computer::new();
    let mut amp_e = computer::Computer::new();

    let amp_a_input: computer::IOQueue = Rc::new(RefCell::new(VecDeque::new()));
    let amp_b_input: computer::IOQueue = Rc::new(RefCell::new(VecDeque::new()));
    let amp_c_input: computer::IOQueue = Rc::new(RefCell::new(VecDeque::new()));
    let amp_d_input: computer::IOQueue = Rc::new(RefCell::new(VecDeque::new()));
    let amp_e_input: computer::IOQueue = Rc::new(RefCell::new(VecDeque::new()));
    let amp_e_output: computer::IOQueue = Rc::new(RefCell::new(VecDeque::new()));

    amp_a.set_input(&amp_a_input);
    amp_a.set_output(&amp_b_input);
    amp_b.set_input(&amp_b_input);
    amp_b.set_output(&amp_c_input);
    amp_c.set_input(&amp_c_input);
    amp_c.set_output(&amp_d_input);
    amp_d.set_input(&amp_d_input);
    amp_d.set_output(&amp_e_input);
    amp_e.set_input(&amp_e_input);
    amp_e.set_output(&amp_e_output);

    let mut options = vec!["0", "1", "2", "3", "4"];

    let mut max = 0;
    let mut best_inputs = String::new();

    let permutator = Heap::new(&mut options);

    for inputs in permutator
    {
        (*amp_a_input).borrow_mut().clear();
        (*amp_b_input).borrow_mut().clear();
        (*amp_c_input).borrow_mut().clear();
        (*amp_d_input).borrow_mut().clear();
        (*amp_e_input).borrow_mut().clear();
        (*amp_e_output).borrow_mut().clear();

        amp_a.load_program(&program);
        amp_b.load_program(&program);
        amp_c.load_program(&program);
        amp_d.load_program(&program);
        amp_e.load_program(&program);

        (*amp_a_input).borrow_mut().push_front(inputs[0].to_string());
        (*amp_a_input).borrow_mut().push_front("0".to_string());
        (*amp_b_input).borrow_mut().push_front(inputs[1].to_string());
        (*amp_c_input).borrow_mut().push_front(inputs[2].to_string());
        (*amp_d_input).borrow_mut().push_front(inputs[3].to_string());
        (*amp_e_input).borrow_mut().push_front(inputs[4].to_string());

        amp_a.run();
        amp_b.run();
        amp_c.run();
        amp_d.run();
        amp_e.run();

        if let Some(output) = (*amp_e_output).borrow_mut().pop_back()
        {
            let tmp = output.parse::<i32>().unwrap();

            if max < tmp
            {
                max = tmp;
                best_inputs = inputs.join("");
            }
        }
    }

    println!("Largest output signal: {}, inputs: {}", max, best_inputs);
}
