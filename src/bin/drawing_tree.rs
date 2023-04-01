
use nannou::noise::*;
use nannou::prelude::*;
use nannou_utils::{C8, CPt, poly_shapes, poly_shapes_colored, Pt, srgba8_t};
use nannou_utils::ca::Ca;

const DISC_COUNT: usize = 5000;
const SPEED: f64 = 0.5;
const R: f32 = 8.;
const GAPS: f32 = (R * 2.) + 2.;
const RULE: i32 = 5;

type NoiseAlgo = Perlin;

const P1: [(u8, u8, u8); 4] = [
    (77, 69, 93),
    (233, 100, 121),
    (245, 233, 207),
    (125, 185, 182),
];

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

fn branch(draw: &Draw, len: f32, theta: f32) {
    let mut length = len;
    // Each branch will be 2/3rds the size of the previous one
    let sw = map_range(length, 2.0, 120.0, 1.0, 10.0);

    draw.line()
        .start(pt2(0.0, 0.0))
        .end(pt2(0.0, length))
        .weight(sw)
        .color(BLACK);
    // Move to the end of that line
    let draw = draw.x_y(0.0, length);

    length *= 0.66;

    // All recursive functions must have an exit condition!!!!
    // Here, ours is when the length of the branch is 2 pixels or less
    if len > 2.0 {
        let draw2 = draw.rotate(theta); // Save the current state of transformation (i.e. where are we now) and Rotate by theta
        branch(&draw2, length, theta); // Ok, now call myself to draw two new branches!!

        // Repeat the same thing, only branch off to the "left" this time!
        let draw3 = draw.rotate(-theta);
        branch(&draw3, length, theta);
    }
}



struct Model {
    theta: f32,
}


fn model(app: &App) -> Model {
    app.new_window()
        .resizable(false)
        .size(600, 600)
        .resized(on_resize)
        .build()
        .unwrap();
    Model {
        theta: 0.0,
    }
}

fn on_resize(_app: &App, model: &mut Model, new_size: Vec2) {}


fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();
    model.theta = map_range(app.mouse.x, win.left(), win.right(), 0.0, PI / 2.0);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win   = app.window_rect();
    let draw = app.draw().x_y(0.0, win.bottom());
    frame.clear(WHITE);
    branch(&draw, 120.0, model.theta);
    draw.to_frame(app, &frame).unwrap();
}
