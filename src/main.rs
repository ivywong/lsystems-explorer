use std::collections::HashMap;

use lsystem::LSystem;
use nannou::prelude::*;

mod turtle;
mod lsystem;

struct Settings {
    scale: f32,
}

struct Drag {
    is_dragging: bool,
    start_pos: Vec2,
}

struct Model {
    x: f32,
    y: f32,
    drag_event: Drag,
    settings: Settings
}

fn main() {
    nannou::app(model)
        // .loop_mode(LoopMode::loop_once())
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .event(event)
        .view(view)
        .build()
        .unwrap();

    Model {
        x: 0.0,
        y: 0.0,
        drag_event: Drag {
            is_dragging: false,
            start_pos: pt2(0.0, 0.0),
        },
        settings: Settings {
            scale: 1.0,
        }
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MousePressed(_button) => {
            match _button {
                MouseButton::Left => {
                    model.drag_event.is_dragging = true;
                    model.drag_event.start_pos = pt2(app.mouse.x, app.mouse.y);
                }
                _ => {}
            }
        }
        MouseReleased(_button) => {
            model.drag_event.is_dragging = false;
            model.drag_event.start_pos = pt2(0.0, 0.0);
        }
        MouseMoved(_pos) => {
            if model.drag_event.is_dragging {
                println!("{:?}", event);
                println!("start: {:?}, curr: {:?}", model.drag_event.start_pos, _pos);
                println!("diff: {}", model.drag_event.start_pos - _pos);
                model.x += _pos.x - model.drag_event.start_pos.x;
                model.y += _pos.y -  model.drag_event.start_pos.y;
            }
        }
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame){
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
        .x_y(-500.0 + model.x, -300.0 + model.y);

    draw.polyline()
        .weight(1.0)
        .color(GREEN)
        .points(sierpinski.draw(7))
        .x_y(-500.0 + model.x, 300.0 + model.y);

    draw.polyline()
        .weight(1.5)
        .color(PURPLE)
        .points(dragon.draw(13))
        .x_y(-300.0 + model.x, -100.0 + model.y)
        .rotate(180.0.to_radians());
    
    draw.to_frame(app, &frame).unwrap();
}