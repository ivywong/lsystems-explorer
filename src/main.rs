use std::collections::HashMap;

use lsystem::LSystem;
use nannou::prelude::*;

mod turtle;
mod lsystem;

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::loop_once())
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame){
    frame.clear(WHITE);

    let draw = app.draw();

    let mut koch = LSystem::new(
        "F",
        HashMap::from([
            ('F', "F+F-F-F+F".to_string())
        ]), 
        10.0,
        90.0
    );

    let mut sierpinski = LSystem::new(
        "F-G-G",
        HashMap::from([
            ('F', "F-G+F+G-F".to_string()),
            ('G', "GG".to_string()),
        ]), 
        3.0,
        120.0
    );

    let mut dragon = LSystem::new(
        "F",
        HashMap::from([
            ('F', "F+G".to_string()),
            ('G', "F-G".to_string()),
        ]),
        10.0,
        90.0
    );

    draw.polyline()
        .weight(1.0)
        .color(BLUE)
        .points(koch.draw(7))
        .x_y(-500.0, -300.0);

    draw.polyline()
        .weight(1.0)
        .color(GREEN)
        .points(sierpinski.draw(7))
        .x_y(-500.0, 300.0);

    draw.polyline()
        .weight(1.5)
        .color(PURPLE)
        .points(dragon.draw(13))
        .x_y(-300.0, -100.0)
        .rotate(180.0.to_radians());
    
    draw.to_frame(app, &frame).unwrap();
}