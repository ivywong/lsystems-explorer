use std::collections::HashMap;

use lsystem::LSystem;
use nannou::prelude::*;
use nannou_egui::{self, egui::{self, Align2, ComboBox}, Egui};

mod turtle;
mod lsystem;

struct Settings {
    scale: f32,
    offset: Vec2,
    level: u32,
    speed: f32,
    angle: f32,
    length: u32,
    animate_angle: bool,
    default_preset: String,
}

#[derive(Clone)]
struct LSystemInput {
    variables: Vec<char>,
    rules: Vec<(char, String)>,
    start: String,
}

struct Preset {
    lsystem: LSystemInput,
    level: u32,
    angle: f32,
    length: u32,
}

struct Drag {
    is_dragging: bool,
    start_pos: Vec2,
}

struct Model {
    drag_event: Drag,
    settings: Settings,
    egui: Egui,
    lsys_input: LSystemInput,
    presets: HashMap<String, Preset>,
}

fn main() {
    nannou::app(model)
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

    let presets: HashMap<String, Preset> = HashMap::from([
        ("sierpinsky".to_string(), Preset {
            level: 6,
            length: 3,
            angle: 120.0,
            lsystem: LSystemInput {
            start: "F-G-G".to_string(),
            rules: vec![
                ('F', "F-G+F+G-F".to_string()),
                ('G', "GG".to_string()),
            ],
            variables: vec!['F', 'G'],
        }}),
        ("dragon".to_string(), Preset {
            level: 12,
            length: 10,
            angle: 90.0,
            lsystem: LSystemInput {
            start: "F".to_string(),
            rules: vec![
                ('F', "F+G".to_string()),
                ('G', "F-G".to_string()),
            ],
            variables: vec!['F', 'G'],
        }}),
    ]);

    let default_preset = "dragon".to_string();
    let preset = presets.get(&default_preset).unwrap();

    Model {
        egui,
        drag_event: Drag {
            is_dragging: false,
            start_pos: pt2(0.0, 0.0),
        },
        settings: Settings {
            scale: 1.0,
            offset: pt2(0.0, 0.0),
            level: preset.level,
            speed: 5.0,
            angle: preset.angle,
            length: preset.length,
            animate_angle: false,
            default_preset,
        },
        lsys_input: preset.lsystem.clone(),
        presets,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(_update.since_start);

    let ctx = egui.begin_frame();
    let window = egui::Window::new("Settings")
        .anchor(Align2::LEFT_TOP, [5.0, 5.0]);

    if settings.animate_angle {
        let sine = (app.time / map_range(settings.speed, 1.0, 10.0, 5.0, 1.0)).sin();
        settings.angle = map_range(sine, -1.0, 1.0, 60.0, 100.0);
    }

    window.show(&ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Preset: ");
            ComboBox::from_label(format!("{}", &settings.default_preset))
            .show_ui(ui, |ui| {
                for (key, _) in model.presets.iter() {
                    ui.selectable_value(&mut settings.default_preset, key.clone(), key);
                }
            });
            if ui.button("load preset").clicked() {
                let preset = model.presets.get(&settings.default_preset).unwrap();
                model.lsys_input = preset.lsystem.clone();
                settings.angle = preset.angle;
                settings.length = preset.length;
                settings.level = preset.level;
            }
        });

        ui.horizontal(|ui| {
            ui.label("n = ");
            ui.add(egui::Slider::new(&mut settings.level, 0..=20));
        });
        ui.horizontal(|ui| {
            ui.label("length: ");
            ui.add(egui::Slider::new(&mut settings.length, 0..=100));
        });
        ui.horizontal(|ui| {
            ui.label("angle: ");
            ui.add(egui::Slider::new(&mut settings.angle, 0.0..=180.0)
                .suffix("°")
                .custom_formatter(|n, _| {
                    format!("{:>3.0}", n)
                })
            );
            ui.checkbox(&mut settings.animate_angle, "animate?");
        });
        
        ui.add_enabled_ui(settings.animate_angle, |ui| {
            ui.horizontal(|ui| {
                ui.label("animation speed: ");
                ui.add(egui::Slider::new(&mut settings.speed, 0.0..=10.0));
            });
        });

        ui.horizontal(|ui| {
            if ui.button(format!("Reset Scale ({:.1})", settings.scale)).clicked() {
                settings.scale = 1.0;
            }
            if ui.button("Recenter").clicked() {
                settings.offset = pt2(0.0, 0.0);
            }
        });

        ui.horizontal(|ui| {
            ui.label("Start: ");
            ui.text_edit_singleline(&mut model.lsys_input.start);
        });

        ui.vertical(|ui| {
            for (key, val) in model.lsys_input.rules.iter_mut() {
                ui.horizontal(|ui| {
                    ui.label(format!("{key}"));
                    ui.text_edit_singleline(val);
                });
            }
        });
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
                    if !model.egui.ctx().is_pointer_over_area() {
                        model.drag_event.is_dragging = true;
                        model.drag_event.start_pos = pt2(app.mouse.x, app.mouse.y);
                    }
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
    let mut rules = HashMap::new();

    model.lsys_input.rules.iter().for_each(|(k, v)| {
        rules.insert(*k, v.to_string());
    });

    let lsystem = LSystem::new(
        &model.lsys_input.start,
        rules,
        model.settings.length,
        model.settings.angle,
    );

    draw.polyline()
        .weight(1.0)
        .color(BLUE)
        .points(lsystem.draw(model.settings.level, model.settings.scale))
        .xy(model.settings.offset);
    
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}