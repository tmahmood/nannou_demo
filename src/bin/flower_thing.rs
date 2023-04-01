use nannou::ease::sine::{ease_in, ease_in_out};
use nannou::lyon::lyon_tessellation::LineCap;
use nannou::noise::*;
use nannou::prelude::*;
use nannou_utils::{C8, CPt, poly_shapes, poly_shapes_colored, Pt, srgba8_t};
use nannou_utils::ca::Ca;


const POINTS: u32 = 2000;
type NoiseAlgo = Perlin;

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}


struct Model {
    noise: NoiseAlgo,
    loc: u32
}


fn model(app: &App) -> Model {
    app.new_window()
        .resizable(false)
        .size(600, 600)
        .resized(on_resize)
        .build()
        .unwrap();
    Model {
        noise: Perlin::new(),
        loc: 1,
    }
}

fn on_resize(_app: &App, model: &mut Model, new_size: Vec2) {
    if model.loc < POINTS {
        model.loc += 1;
    }
}


fn update(app: &App, model: &mut Model, _update: Update) {
    if model.loc < POINTS {
        model.loc += 1;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {

    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(BLACK);
    }

    let r1 = 6.0 * (model.loc as f32).sqrt();
    let theta1 = 2.4 * model.loc as f32;
    let x1 = r1 * theta1.cos();
    let y1 = r1 * theta1.sin();
    draw.ellipse().x_y(x1, y1).color(INDIGO).w_h(9.0, 9.0);

    let r0 = 6.0 * (model.loc as f32).sqrt() - 2.0;
    let theta0 = 2.4 * model.loc as f32;
    let x0 = r0 * theta0.cos();
    let y0 = r0 * theta0.sin();
    draw.ellipse()
        .x_y(x0, y0)
        .color(WHITE)
        .w_h(5.0, 5.0)
        .stroke(GRAY)
        .stroke_weight(2.0);

    draw.rect()
        .wh(app.window_rect().wh())
        .color(srgba(0.0, 0.0, 0.0, 0.001));

    if model.loc >= POINTS {
        app.set_loop_mode(LoopMode::loop_once());
    }

    draw.to_frame(app, &frame).unwrap();
}
