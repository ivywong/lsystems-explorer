use std::collections::HashMap;

use nannou::glam::Vec2;
use nannou::rand::prelude::SliceRandom;
use rand_chacha::{rand_core::SeedableRng, ChaCha12Rng};

use crate::turtle::Turtle;

pub struct LSystem {
    pub start: String,
    pub rules: HashMap<String, Vec<(String, u8)>>,
    pub length: u32,
    pub angle: f32,
    rng: ChaCha12Rng,
}

impl LSystem {
    pub fn new(start: &str, rules: HashMap<String, Vec<(String, u8)>>, length: u32, angle: f32, seed: u64) -> LSystem {
        LSystem {
            start: start.to_string(),
            rules,
            length,
            angle,
            rng: ChaCha12Rng::seed_from_u64(seed),
        }
    }

    pub fn draw(&mut self, n: u32, scale: f32) -> Vec<Vec<Vec2>> {
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

    pub fn rewrite(&mut self, input: &String) -> String {
        let mut res = String::from("");
        for c in input.chars() {
            match c {
                c if self.rules.contains_key(&c.to_string()) => {
                    res.push_str(&self.get_random_rewrite(&c.to_string()))
                },
                _ => res.push(c)
            }
        }
        res
    }

    fn get_random_rewrite(&mut self, key: &String) -> String {
        if self.rules.contains_key(&key.to_string()) {
            let rules = self.rules.get(&key.to_string()).unwrap();
            rules.choose_weighted(&mut self.rng, |item| item.1).unwrap().0.clone()
        } else {
            String::from("")
        }
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