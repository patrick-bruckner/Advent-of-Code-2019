use std::collections::HashSet;
use fraction::{Fraction,ToPrimitive};

use super::Point;

#[derive(Debug)]
pub struct Line {
    // fractions are used to maximize accuracy
    m: Fraction,
    b: Fraction
}

impl Line {
    pub fn calc_line(p1: &Point, p2: &Point) -> Self {
        let p1_x = Fraction::from(p1.x);
        let p1_y = Fraction::from(p1.y);
        let p2_x = Fraction::from(p2.x);
        let p2_y = Fraction::from(p2.y);

        let m = (p2_y - p1_y) / (p2_x - p1_x);
        let b = p1_y - (m * p1_x);

        if m == 0.0.into() {
            // store y cord as b if slove is 0
            Self {m, b:p1_y}
        } else if m.is_infinite() {
            // store x cord as b if slope is inf
            Self {m, b:p1_x}
        } else {
            Self {m, b}
        }
    }

    pub fn find_int_points(&self, start: isize, end: isize) -> HashSet<Point> {
        let mut points: HashSet<Point> = HashSet::new();

        for v in (start+1)..end {
            if self.m.is_infinite() {
                // slope is inifinite so add points along x axis
                points.insert(Point::new(self.b.to_f64().unwrap() as isize, v));
            } else if self.m == 0.0.into() {
                // slope is 0 so add points along y axis
                points.insert(Point::new(v, self.b.to_f64().unwrap() as isize));
            } else {
                let y = (self.m*(v.into())) + self.b;
                if y.fract() == 0.0.into() {
                    points.insert(Point::new(v, y.to_f64().unwrap() as isize));
                }
            }
        }

        return points;
    }
}
