use nannou::prelude::*;
use nannou_utils::{draw_soft_bg, get_random_color, get_random_green, Pt, srgba8_t};

const A: f32 = 1.5;
const B: f32 = -1.8;
const C: f32 = 1.6;
const D: f32 = 0.9;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    points: Pt,
    h: f32,
}

fn get_random_point() -> Vec2 {
    pt2(
        random_range(-2.0, 2.0),
        random_range(-2.0, 2.0),
    )
}
fn model(_app: &App) -> Model {
    let points = vec![
        get_random_point()
    ];
    Model { points, h: random() }
}

fn de_jong(p: &Vec2) -> Vec2 {
    vec2 (
        (A * p.y).sin() - (B * p.x).cos(),
        (C * p.x).sin() - (D * p.y).cos()
    )
}

fn clifford_attractors(p: &Vec2) -> Vec2 {
    vec2 (
        (A * p.y).sin() + C * (A * p.x).cos(),
        (B * p.x).sin() + D * (B * p.y).cos()
    )
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // xn+1 = sin(a * yn) + c * cos(a xn)
    // yn+1 = sin(b * xn) + d * cos(b yn)

    if model.points.len() < 1000 {
        model.points.push(get_random_point());
    }

    model.points.iter_mut().for_each(|p| {
        *p = clifford_attractors(&p)
    });
    model.h = random();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let r = app.window_rect();

    for p in model.points.clone() {
        let xn = map_range(p.x, -3., 3., r.left(), r.right());
        let yn = map_range(p.y, -3., 3., r.top(), r.bottom());
        draw.ellipse().radius(0.2).color(hsla(
            model.h,
            0.72,
            0.80,
            0.8,
        )).x_y(xn, yn);
    }
    draw.to_frame(app, &frame).unwrap();
}
