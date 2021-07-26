use crate::util::{Grid,Point};

pub fn part2(grid: &Grid, center_pt: Point) {
    // center grid at center_pt
    let adj_grid = grid.move_center(center_pt);
    let destroy_order = adj_grid.find_destroy_order();

    // for (idx, p) in destroy_order.iter().enumerate() {
    //     let adj_p = *p + center_pt;
    //     println!("Destryy asteroid {} at ({},{})", idx+1, adj_p.x, adj_p.y);
    // }

    // get 200th destroyed asteroid and restore orig position
    let at_200 = destroy_order[199] + center_pt;
    let result = (at_200.x * 100) + at_200.y;

    println!("200th asteroid destroyed: ({},{}); result: {}",at_200.x, at_200.y, result);
}
