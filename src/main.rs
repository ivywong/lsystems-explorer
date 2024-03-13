use std::collections::HashMap;

use lsystem::LSystem;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

mod turtle;
mod lsystem;

struct Settings {
    scale: f32,
    offset: Vec2,
}

struct Drag {
    is_dragging: bool,
    start_pos: Vec2,
}

struct Model {
    drag_event: Drag,
    settings: Settings,
    egui: Egui,
}

fn main() {
    nannou::app(model)
        // .loop_mode(LoopMode::loop_once())
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    let window_id = app.new_window()
        .event(event)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = &app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);

    Model {
        egui,
        drag_event: Drag {
            is_dragging: false,
            start_pos: pt2(0.0, 0.0),
        },
        settings: Settings {
            scale: 1.0,
            offset: pt2(0.0, 0.0),
        },
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(_update.since_start);

    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        if ui.button(format!("Reset Scale ({:.1})", settings.scale)).clicked() {
            settings.scale = 1.0;
        }
        if ui.button("Recenter").clicked() {
            settings.offset = pt2(0.0, 0.0);
        }
    });
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
                model.settings.offset += _pos - model.drag_event.start_pos;
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
        .xy(model.settings.offset);

    draw.polyline()
        .weight(1.0)
        .color(GREEN)
        .points(sierpinski.draw(7))
        .xy(model.settings.offset);

    draw.polyline()
        .weight(1.5)
        .color(PURPLE)
        .points(dragon.draw(13))
        .rotate(180.0.to_radians())
        .xy(model.settings.offset);
    
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}