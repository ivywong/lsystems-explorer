use std::collections::HashMap;

use nannou::glam::Vec2;

use crate::turtle::Turtle;

pub struct LSystem {
    start: String,
    rules: HashMap<char, String>,
    length: f32,
    angle: f32,
}

impl LSystem {
    pub fn new(start: &str, rules: HashMap<char, String>, length: f32, angle: f32) -> LSystem {
        LSystem {
            start: start.to_string(),
            rules,
            length,
            angle
        }
    }

    pub fn draw(&mut self, n: i32) -> Vec<Vec2> {
        let mut command = self.start.to_owned();

        // println!("\n0: {}", command);

        for _ in 1..n + 1 {
            command = self.rewrite(&command);
            // println!("{}: {}", i, command);
        }

        let points: Vec<Vec2> = self.parse(&command);
        // println!("{:?}", points);

        points
    }

    pub fn rewrite(&self, input: &String) -> String {
        let mut res = "".to_owned();
        for c in input.chars() {
            match c {
                c if self.rules.contains_key(&c) => res.push_str(self.rules.get(&c).unwrap()),
                _ => res.push(c)
            }
        }
        res
    }

    pub fn parse(&mut self, input: &String) -> Vec<Vec2> {
        let mut turtle = Turtle::new();
        let mut points = vec![turtle.curr()];

        for c in input.chars() {
            match c {
                c if self.rules.contains_key(&c) => points.push(turtle.fd(self.length)),
                '+' => turtle.left(self.angle),
                '-' => turtle.right(self.angle),
                '[' => turtle.push(),
                ']' => turtle.pop(),
                _ => println!("Malformed input: {}, {}", input, c),
            }
        }
        points
    }
}