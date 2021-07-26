use fraction::{Fraction};
use super::Point;

#[derive(Debug)]
pub struct Vector {
    pub point: Point,
    pub mag: isize,
    // fraction is used to maximize accuracy
    pub angle: Fraction
}

#[derive(PartialEq, PartialOrd)]
pub enum Quadrant {
    I, II, III, IV
}

impl Vector {
    pub fn new(point: Point) -> Self {
        let mag = Point::distance(&Point::new(0,0), &point);

        // omit atan from calculation to avoid float
        let angle = Fraction::from(point.y) / Fraction::from(point.x);

        let v = Self {
            point,
            mag,
            angle
        };

        return v;
    }

    pub fn get_quadrant(&self) -> Quadrant {
        // start from straight down and rotate counter-clockwise
        // straight down is at beginning of quad I
        if self.point.x >= 0 && self.point.y < 0 {
            return Quadrant::I;
        // straight right is at beginning of quad II
        } else if self.point.x > 0 && self.point.y >= 0 {
            return Quadrant::II;
        // straight up is at beginning of quad III
        } else if self.point.x <= 0 && self.point.y > 0 {
            return Quadrant::III;
        // straight left is at beginning of quad IV
        } else {
            return Quadrant::IV;
        }
    }
}
