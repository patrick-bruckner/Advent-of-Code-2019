use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point
{
    x: i32,
    y: i32
}

impl Point
{
    fn new(x: i32, y: i32) -> Self
    {
        Self { x, y }
    }
}

enum Direction
{
    Right,
    Down,
    Up,
    Left
}

impl Direction
{
    fn from_char(c: char) -> Self
    {
        match c
        {
            'R' => Self::Right,
            'D' => Self::Down,
            'U' => Self::Up,
            'L' => Self::Left,
            _ => panic!("Invalid direction char: {}", c)
        }
    }
}

struct Move
{
    direction: Direction,
    amount: i32
}

impl Move
{
    fn new(direction: Direction, amount: i32) -> Self
    {
        Self { direction, amount }
    }

    fn from_string(s: String) -> Self
    {
        Self::new(Direction::from_char(s.chars().next().unwrap()),
                  s[1..].parse::<i32>().unwrap())
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Wire
{
    Wire1,
    Wire2
}

struct Grid
{
    visited_points: HashSet<Point>,
    collisions: HashSet<Point>
}

impl Grid
{
    fn new() -> Self
    {
        Self
        {
            visited_points: HashSet::<Point>::new(),
            collisions: HashSet::<Point>::new()
        }
    }

    fn add_wire1(&mut self, moves: &String)
    {
        self.add_wire(moves, Wire::Wire1);
    }

    fn add_wire2(&mut self, moves: &String)
    {
        self.add_wire(moves, Wire::Wire2);
    }

    fn add_wire(&mut self, moves: &String, wire: Wire)
    {
        let mut moves_vec = Vec::<Move>::new();

        for movement in moves.split(',')
        {
            moves_vec.push(Move::from_string(movement.to_string()));
        }

        let mut point = Point::new(0, 0);

        for movement in moves_vec
        {
            match movement.direction
            {
                Direction::Right => self.move_point_right(&mut point, movement.amount, wire),
                Direction::Down => self.move_point_down(&mut point, movement.amount, wire),
                Direction::Up => self.move_point_up(&mut point, movement.amount, wire),
                Direction::Left => self.move_point_left(&mut point, movement.amount, wire)
            }
        }
    }

    fn move_point_right(&mut self, point: &mut Point, mut amount: i32, wire: Wire)
    {
        while amount > 0
        {
            point.x += 1;
            self.add_visited_point(*point, wire);
            amount -= 1;
        }
    }

    fn move_point_down(&mut self, point: &mut Point, mut amount: i32, wire: Wire)
    {
        while amount > 0
        {
            point.y -= 1;
            self.add_visited_point(*point, wire);
            amount -= 1;
        }
    }

    fn move_point_up(&mut self, point: &mut Point, mut amount: i32, wire: Wire)
    {
        while amount > 0
        {
            point.y += 1;
            self.add_visited_point(*point, wire);
            amount -= 1;
        }
    }

    fn move_point_left(&mut self, point: &mut Point, mut amount: i32, wire: Wire)
    {
        while amount > 0
        {
            point.x -= 1;
            self.add_visited_point(*point, wire);
            amount -= 1;
        }
    }

    fn add_visited_point(&mut self, point: Point, wire: Wire)
    {
        if self.visited_points.contains(&point) && wire == Wire::Wire2
        {
            self.collisions.insert(point);
        }
        else
        {
            self.visited_points.insert(point);
        }
    }

    fn find_min_distance(&self) -> i32
    {
        let mut min_distance = std::i32::MAX;

        for point in &self.collisions
        {
            let distance = point.x.abs() + point.y.abs();
            if distance < min_distance
            {
                min_distance = distance;
            }
        }

        return min_distance;
    }
}

pub fn part1()
{
    let file = File::open("input/part1.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut grid = Grid::new();

    grid.add_wire1(&lines[0]);
    grid.add_wire2(&lines[1]);

    println!("Manhattan Distance: {}", grid.find_min_distance());
}
