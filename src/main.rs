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
    offset: Vec2,
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
        offset: pt2(0.0, 0.0),
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
        MouseWheel(amount, _phase) => {
            match amount {
                MouseScrollDelta::PixelDelta(pos) => {
                    model.settings.scale += pos.y as f32 / 50.0;
                    model.settings.scale = model.settings.scale.clamp(0.2, 10.0);
                }
                _ => {},
            }
        }
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
                model.offset += _pos - model.drag_event.start_pos;
                model.drag_event.start_pos = _pos;
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
        10.0 * model.settings.scale,
        90.0
    );

    let mut sierpinski = LSystem::new(
        "F-G-G",
        HashMap::from([
            ('F', "F-G+F+G-F".to_string()),
            ('G', "GG".to_string()),
        ]), 
        3.0 * model.settings.scale,
        120.0
    );

    let mut dragon = LSystem::new(
        "F",
        HashMap::from([
            ('F', "F+G".to_string()),
            ('G', "F-G".to_string()),
        ]),
        10.0 * model.settings.scale,
        90.0
    );

    draw.polyline()
        .weight(1.0)
        .color(BLUE)
        .points(koch.draw(7))
        .xy(model.offset);

    draw.polyline()
        .weight(1.0)
        .color(GREEN)
        .points(sierpinski.draw(7))
        .xy(model.offset);

    draw.polyline()
        .weight(1.5)
        .color(PURPLE)
        .points(dragon.draw(13))
        .rotate(180.0.to_radians())
        .xy(model.offset);
    
    draw.to_frame(app, &frame).unwrap();
}