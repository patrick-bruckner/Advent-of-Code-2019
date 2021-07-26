use std::collections::{HashSet,HashMap};
use std::cmp::Ordering;

use super::{Line, Point, Vector};

pub struct Grid {
    asteroids: HashSet<Point>
}

impl Grid {
    pub fn new(input_str: String) -> Self {
        let lines: Vec<String> = input_str.lines().map(|s| s.to_string()).collect();

        // using hashset so set intersection can be done later
        let mut asteroids: HashSet<Point> = HashSet::new();

        for (r_idx, r) in lines.iter().enumerate() {
            for (c_idx, c) in r.chars().enumerate() {
                match c {
                    '#' => {
                        let _ = asteroids.insert(Point::new(c_idx as isize, r_idx as isize));
                    },
                    _ => ()
                }
            }
        }

        Self {
            asteroids
        }
    }

    pub fn find_best_asteroid(&self) -> (Point,usize) {
        let mut visible: HashMap<Point, usize> = HashMap::new();

        for p1 in &self.asteroids {
            visible.insert(*p1, 0);
            for p2 in &self.asteroids {
                if p1 == p2 {
                    continue;
                }

                let line = Line::calc_line(p1, p2);

                let points_on_line = if (p1.x == p2.x) && (p1.y < p2.y) {
                    line.find_int_points(p1.y, p2.y)
                } else if p1.x == p2.x {
                    line.find_int_points(p2.y, p1.y)
                } else if p1.x < p2.x {
                    line.find_int_points(p1.x, p2.x)
                } else {
                    line.find_int_points(p2.x, p1.x)
                };

                let in_between = self.asteroids.intersection(&points_on_line).collect::<Vec<&Point>>();

                // if in_between.len() == 0, there are no asteroids between p1 and p2
                if in_between.len() == 0 {
                    visible.insert(*p1, visible[p1] + 1);
                }
            }
        }

        let mut max = (Point::new(0, 0), 0);
        for (p, c) in visible.iter() {
            if *c > max.1 {
                max = (*p,*c);
            }
        }

        return max;
    }

    pub fn find_destroy_order(&self) -> Vec<Point> {
        let mut vecrors: Vec<Vector> = Vec::new();
        let mut destroy_order: Vec<Point> = Vec::new();

        for a in &self.asteroids {
            vecrors.push(Vector::new(*a));
        }

        let compare = |a: &Vector, b: &Vector| -> Ordering {
            let a_quad = a.get_quadrant();
            let b_quad = b.get_quadrant();

            // check if quadrands differ
            if a_quad != b_quad {
                // different quadrants so order is determined by
                //  which quad comes first
                if a_quad < b_quad {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            } else {
                // same quad so closer look is needed
                // check if angles are the same
                if a.angle == b.angle {
                    // angles are the same so closer asteroid comes first
                    if a.mag < b.mag {
                        return Ordering::Less
                    } else {
                        return Ordering::Greater;
                    }
                } else {
                    // andgles are different so check which angle comes first
                    if a.angle < b.angle {
                        return Ordering::Less;
                    } else {
                        return Ordering::Greater;
                    }
                }
            }
        };

        vecrors.sort_by(compare);

        // determine actuall destory order
        // this takes into consideration that only one asteroid
        //  can be destroyed per angle per rotation
        while vecrors.len() > 0 {
            let mut last_angle = None;
            vecrors.retain(|v| {
                let should_retain = match last_angle {
                    Some(last_a) => v.angle == last_a,
                    None => false
                };
                if !should_retain {
                    destroy_order.push(v.point);
                    last_angle = Some(v.angle);
                }

                return should_retain;
            });
        }

        for v in vecrors {
            destroy_order.push(v.point);
        }

        return destroy_order;
    }

    pub fn move_center(&self, new_center: Point) -> Self {
        let mut asteroids: HashSet<Point> = HashSet::new();

        for a in &self.asteroids {
            asteroids.insert(*a-new_center);
        }

        // remove origin since it won't be destroyed
        asteroids.remove(&Point::new(0, 0));

        Self {asteroids}
    }
}
