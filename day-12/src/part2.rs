use crate::moon::Moon;

use num_format::{Locale, ToFormattedString};
use paste::paste;

use std::fs::read_to_string;

macro_rules! find_steps_for_axis {
    ($axis:ident, $moons:expr) => {{
        let mut steps: usize = 0;

        let initial_state = $moons.clone();

        loop {
            steps += 1;

            // update velocity
            for a in 0..$moons.len() {
                for b in (a+1)..$moons.len() {
                    let (head, tail) = $moons.split_at_mut(a+1);
                    paste!{Moon::[<update_velocity_$axis>](&mut head[a], &mut tail[b - a - 1]);}
                }
            }

            // apply velocity
            for m in &mut $moons {
                m.update_position();
            }

            // if start_hash == calculate_hash(&moons) {
            if initial_state == $moons {
                break;
            }
        }

        steps
    }};
}

fn gcd(x: usize, y: usize) -> usize {
    if x == y {
        return x;
    } else {
        if x > y {
            return gcd(x-y, y);
        } else {
            return gcd(x, y-x);
        }
    }
}

fn lcm(x: usize, y: usize) -> usize {
    return (x * y) / gcd(x, y);
}

pub fn part2() {
    let input = read_to_string("input/input.txt").unwrap();

    let mut moons: Vec<Moon> = Vec::new();
    for line in input.lines() {
        moons.push(Moon::new_from_str(line));
    }

    let steps_x = find_steps_for_axis!(x, moons);
    let steps_y = find_steps_for_axis!(y, moons);
    let steps_z = find_steps_for_axis!(z, moons);

    println!("Repeat in X after {} steps",
        steps_x.to_formatted_string(&Locale::en));
    println!("Repeat in Y after {} steps",
        steps_y.to_formatted_string(&Locale::en));
    println!("Repeat in Z after {} steps",
        steps_z.to_formatted_string(&Locale::en));

    let steps = lcm(lcm(steps_x, steps_y), steps_z);
    println!("Repeat in system after {} steps",
        steps.to_formatted_string(&Locale::en));
}
