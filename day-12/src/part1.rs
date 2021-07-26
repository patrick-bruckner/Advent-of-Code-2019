use crate::moon::Moon;

use std::fs::read_to_string;


pub fn part1() {
    let input = read_to_string("input/input.txt").unwrap();

    let mut moons: Vec<Moon> = Vec::new();
    for line in input.lines() {
        moons.push(Moon::new_from_str(line));
    }

    for _ in 0..1000 {
        // update velocity
        for a in 0..moons.len() {
            for b in (a+1)..moons.len() {
                let (head, tail) = moons.split_at_mut(a+1);
                Moon::update_velocity(&mut head[a], &mut tail[b - a - 1])
            }
        }

        // apply velocity
        for m in &mut moons {
            m.update_position();
        }
    }

    // calculate energy
    let mut system_energy = 0;

    for m in &moons {
        system_energy += m.calculate_energy();
    }

    println!("Total System Energy after 1000 steps: {}", system_energy);
}
