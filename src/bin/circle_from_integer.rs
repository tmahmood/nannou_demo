use std::ops::Add;
use nannou::color::IntoLinSrgba;
use nannou::draw::properties::ColorScalar;
use nannou::event::Key::P;
use nannou::prelude::*;
use nannou_utils::{draw_soft_bg, get_random_color, get_random_green, Pt, srgba8_t};


const D: f32 = 1.;
const E: f32 = 1.7323;
const COLORS: [(u8, u8, u8); 7] = [
    (125, 22, 22),
    (107, 19, 16),
    (69, 15, 16),
    (72, 0, 50),
    (223, 0, 84),
    (255, 139, 106),
    (255, 214, 194),
];


fn p() -> f32 {
    // p = pi/arcsin(sqrt(d e)/2)
    let k = (D * E).sqrt() / 2.;
    PI / (4. * k.asin())
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    points: Pt,
}

fn model(_app: &App) -> Model {
    Model {
        points: (0..1000).map(|_| Vec2::ONE * 6.0).collect(),
    }
}

fn map_sin(v: f32, out_min: f32, out_max: f32) -> f32 {
    map_range(v.sin(), -1., 1., out_min, out_max)
}

fn calc_next_position(mut point: Vec2, t: f32) -> Vec2 {
    let d = map_sin((t * 1.1), 0.01, 0.12);
    let e = (t.sqrt() * 0.01 + 0.3).min(1.1);

    point.x -= (d * point.y).floor();
    point.y += (e * point.x).floor();

    point
}

fn update(app: &App, model: &mut Model, _update: Update) {

    let t = app.elapsed_frames() as f32 / 60.;
    let mut last = model.points.last().unwrap().clone();
    for point in model.points.iter_mut() {
        last = calc_next_position(last, t);
        *point = last;
    }

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let t = frame.nth() as f32 / 60.0;
    draw_soft_bg(&draw, app, BLACK, 0.01);
    for point in model.points.iter() {
        let r = map_sin(point.x.powi(3) + point.y.powi(7) + t * 0.1, 1.0, 3.0);
        draw.ellipse().xy(point.clone() * 7.).radius(r).color(get_random_green(Some(240)));
    }
    draw.to_frame(app, &frame).unwrap();
}