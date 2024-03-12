use std::collections::HashMap;

use lsystem::LSystem;
use nannou::prelude::*;

mod turtle;
mod lsystem;

fn main() {
    nannou::app(model)
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
        "F".to_string(), 
        HashMap::from([
            ('F', "F+F-F-F+F".to_string())
        ]), 
        30.0, 
        90.0
    );

    let mut sierpinski = LSystem::new(
        "F-G-G".to_string(),
        HashMap::from([
            ('F', "F-G+F+G-F".to_string()),
            ('G', "GG".to_string()),
        ]), 
        50.0, 
        120.0
    );

    draw.polyline()
        .weight(2.0)
        .color(BLUE)
        .points(koch.draw(3))
        .x(-500.0);

    draw.polyline()
        .weight(2.0)
        .color(GREEN)
        .points(sierpinski.draw(3))
        .x(-500.0);
    
    draw.to_frame(app, &frame).unwrap();
}