use nannou::{geom::pt2, glam::Vec2};
pub struct State {
    pos: Vec2,
    head: f32,
}

impl State {
    pub fn new(pos: Vec2, head: f32) -> State {
        State {
            pos,
            head
        }
    }
}

pub struct Turtle {
    /*
        turtle keeps current state of the pen:
        - LIFO stack() - position, angle
            - push()
            - pop()
        - current position
        - current heading

        turtle.fd(100) -> (x, y) -> set and return position
        turtle.left(120) (deg) -> set heading
        turtle.right(120) (deg) -> set heading
     */
    stack: Vec<State>,
    position: Vec2,
    heading: f32, // radians
}

impl Turtle {
    pub fn new() -> Turtle {
        let pos = pt2(0.0, 0.0);

        Turtle {
            position: pos,
            stack: Vec::new(),
            heading: 0.0,
        }
    }

    pub fn curr(&self) -> Vec2 {
        self.position
    }

    pub fn fd(&mut self, distance: f32) -> Vec2 {
        let new_pos = pt2(
            self.position.x + (distance * self.heading.cos()),
            self.position.y + (distance * self.heading.sin())
        );
        self.position = new_pos;

        new_pos
    }

    pub fn left(&mut self, degrees: f32) {
        self.heading = self.heading + degrees.to_radians();
    }

    pub fn right(&mut self, degrees: f32) {
        self.heading = self.heading - degrees.to_radians();
    }

    pub fn push(&mut self) {
        self.stack.push(State::new(self.position, self.heading))
    }

    pub fn pop(&mut self) {
        let last = self.stack.pop();
        match last {
            Some(State { pos, head }) => {
                self.position = pos;
                self.heading = head;
            },
            None => println!("Malformed command: nothing to pop!"), 
        }
    }
}