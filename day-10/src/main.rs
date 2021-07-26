use std::fs::read_to_string;

mod part1;
mod part2;
mod util;
use util::Grid;

fn main() {
    let mut input_str = read_to_string("input/input.txt").unwrap();
    // let mut input_str = read_to_string("input/test_input5.txt").unwrap();
    input_str = input_str.trim_end().to_string();

    let grid = Grid::new(input_str);

    let center = part1::part1(&grid);
    part2::part2(&grid, center);
}
