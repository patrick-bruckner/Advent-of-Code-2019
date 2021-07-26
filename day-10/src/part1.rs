use crate::util::{Grid, Point};

pub fn part1(grid: &Grid) -> Point {
    let best = grid.find_best_asteroid();
    println!("Best asteroid at ({},{}) can see {}",
             best.0.x, best.0.y, best.1);

    return best.0;
}
