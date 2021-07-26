use regex::Regex;

#[derive(Debug, Hash, PartialEq, Clone)]
struct Position {
    x: isize,
    y: isize,
    z: isize
}

#[derive(Debug, Hash, PartialEq, Clone)]
pub struct Moon {
    pos: Position,
    velocity: Position
}

impl Position {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self {x, y, z}
    }

    fn new_from_str(s: &str) -> Self {
        let re = Regex::new(r"^<x=([-0-9]+), y=([-0-9]+), z=([-0-9]+)>$").unwrap();
        let cap = re.captures(s).unwrap();

        Self {
            x: cap[1].parse().unwrap(),
            y: cap[2].parse().unwrap(),
            z: cap[3].parse().unwrap()
        }
    }
}

impl Moon {
    pub fn new_from_str(s: &str) -> Self {
        Self {
            pos: Position::new_from_str(s),
            velocity: Position::new(0, 0, 0)
        }
    }

    pub fn update_position(&mut self) {
        self.pos.x += self.velocity.x;
        self.pos.y += self.velocity.y;
        self.pos.z += self.velocity.z;
    }

    pub fn calculate_energy(&self) -> isize {
        return self.calculate_potential_energy() *
            self.calculate_kinetic_energy();
    }

    pub fn calculate_potential_energy(&self) -> isize {
        return self.pos.x.abs() + self.pos.y.abs() +
            self.pos.z.abs();
    }

    pub fn calculate_kinetic_energy(&self) -> isize {
        return self.velocity.x.abs() + self.velocity.y.abs() +
            self.velocity.z.abs();
    }

    pub fn update_velocity(a: &mut Self, b: &mut Self) {
        Self::update_velocity_x(a, b);
        Self::update_velocity_y(a, b);
        Self::update_velocity_z(a, b);
    }

    pub fn update_velocity_x(a: &mut Self, b: &mut Self) {
        if a.pos.x < b.pos.x {
            a.velocity.x += 1;
            b.velocity.x -= 1;
        } else if a.pos.x > b.pos.x {
            a.velocity.x -= 1;
            b.velocity.x += 1;
        }
    }
    pub fn update_velocity_y(a: &mut Self, b: &mut Self) {
        if a.pos.y < b.pos.y {
            a.velocity.y += 1;
            b.velocity.y -= 1;
        } else if a.pos.y > b.pos.y {
            a.velocity.y -= 1;
            b.velocity.y += 1;
        }
    }
    pub fn update_velocity_z(a: &mut Self, b: &mut Self) {
        if a.pos.z < b.pos.z {
            a.velocity.z += 1;
            b.velocity.z -= 1;
        } else if a.pos.z > b.pos.z {
            a.velocity.z -= 1;
            b.velocity.z += 1;
        }
    }
}
