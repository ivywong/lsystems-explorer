use std::collections::HashMap;

use nannou::glam::Vec2;

use crate::turtle::Turtle;

pub struct LSystem {
    pub start: String,
    pub rules: HashMap<String, String>,
    pub length: u32,
    pub angle: f32,
}

impl LSystem {
    pub fn new(start: &str, rules: HashMap<String, String>, length: u32, angle: f32) -> LSystem {
        LSystem {
            start: start.to_string(),
            rules,
            length,
            angle
        }
    }

    pub fn draw(&self, n: u32, scale: f32) -> Vec<Vec<Vec2>> {
        let mut command = self.start.to_owned();

        // println!("\n0: {}", command);

        for _ in 1..n + 1 {
            command = self.rewrite(&command);
            // println!("{}: {}", i, command);
        }

        let points: Vec<Vec<Vec2>> = self.calc_points(&command, scale);
        // println!("{:?}", points);

        points
    }

    pub fn rewrite(&self, input: &String) -> String {
        let mut res = String::from("");
        for c in input.chars() {
            match c {
                c if self.rules.contains_key(&c.to_string()) => res.push_str(self.rules.get(&c.to_string()).unwrap()),
                _ => res.push(c)
            }
        }
        res
    }

    pub fn calc_points(&self, input: &String, scale: f32) -> Vec<Vec<Vec2>> {
        let mut turtle = Turtle::new();
        let mut points = vec![vec![turtle.curr()]];

        for c in input.chars() {
            match c {
                'X' => continue,
                c if self.rules.contains_key(&c.to_string()) => {
                    points.last_mut().unwrap().push(turtle.fd(self.length as f32 * scale))
                },
                '+' => turtle.left(self.angle),
                '-' => turtle.right(self.angle),
                '[' => turtle.push(),
                ']' => match turtle.pop() {
                    Ok(pos) => points.push(vec![pos]),
                    Err(err) => println!("{}", err),
                },
                _ => println!("Malformed input: {}", c),
            }
        }
        points
    }
}