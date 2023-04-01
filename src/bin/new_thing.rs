#![feature(fn_traits)]

use nannou::noise::*;
use nannou::prelude::*;
use nannou_utils::{C8, CPt, get_random_blue, get_random_color, get_random_green, get_random_night, get_random_retro, poly_shapes, poly_shapes_colored, Pt, srgba8_t};
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


struct Model {
    element: Hex,
    p: Vec2,
    s: Vec2,
    theta: f32,
}

pub struct Hex {
    color: C8,
    stroke_color: C8,
    points: Pt,
    rotation: f32,
}

impl Hex {
    pub fn new(points: Pt, color: C8, stroke_color: C8) -> Self {
        Self {
            color,
            stroke_color,
            points,
            rotation: 0.,
        }
    }


    pub fn draw(&self, position: Vec2, draw: &Draw, color: Option<C8>) {
        let d = draw.polygon().xy(position);
        if let Some(c) = color {
            let mut sc = self.stroke_color;
            sc.alpha = c.alpha;
            d.stroke_color(sc)
                .rotate(self.rotation)
                .color(c)
                .points(self.points.clone());
        } else {
            d.stroke_color(self.stroke_color).color(self.color)
                .rotate(self.rotation)
                .points(self.points.clone());
        }
    }

    pub fn set_color(&mut self, color: C8) {
        self.color = color;
    }

    pub fn set_stroke_color(&mut self, stroke_color: C8) {
        self.stroke_color = stroke_color;
    }
}


fn model(app: &App) -> Model {
    app.new_window()
        .resizable(false)
        .size(600, 600)
        .resized(on_resize)
        .build()
        .unwrap();
    let r = R;
    let c = srgba8_t(P1[0], 250);
    let s = srgba8_t(P1[1], 250);


    Model {
        element: Hex::new(poly_shapes(r, c, 60), c, s),
        p: Default::default(),
        s: Default::default(),
        theta: 0.0,
    }
}

fn on_resize(_app: &App, model: &mut Model, new_size: Vec2) {}


fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();

    let t = (app.elapsed_frames() as f32) * 0.03;
    let w = (t * 0.832).sin() * 90.0 + 100.0;
    let h = (t * 0.734).cos() * 90.0 + 100.0;
    let x = (t * 0.132).cos() * 200.0;
    let y = (t * 0.176).sin() * 200.0;

    model.element.rotation += 10.;
    model.p = vec2(x, y);
    model.s = vec2(w, h);
    model.theta = map_range(app.mouse.x, win.left(), win.right(), 0.0, PI / 2.0);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win   = app.window_rect();
    let draw = app.draw();


    let s1 = srgba8_t(P1[3], 100);
    if frame.nth() == 0 {
        frame.clear(BLACK);
    }


    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(600.0, 600.0)
        .color(srgba8(0, 0, 0, 10));

    let num_of_hex_w = win.w() / GAPS;
    let num_of_hex_h = win.h() / GAPS;
    let element = &model.element;


    for jj in 0..(num_of_hex_w.ceil() / 2.) as usize {
        for ii in 0..(num_of_hex_h.ceil() / 2.) as usize {
            let p = Vec2::new(ii as f32 * GAPS, jj as f32 * GAPS);
            draw_shaded(p, model, &draw, s1, element);

            let p = Vec2::new(ii as f32 * -GAPS, jj as f32 * GAPS);
            draw_shaded(p, model, &draw, s1, element);

            let p = Vec2::new(ii as f32 * -GAPS, jj as f32 * -GAPS);
            draw_shaded(p, model, &draw, s1, element);

            let p = Vec2::new(ii as f32 * GAPS, jj as f32 * -GAPS);
            draw_shaded(p, model, &draw, s1, element);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}


fn draw_shaded(p: Vec2, model: &Model, draw: &Draw, s1: C8, element: &Hex) {
    let d = p.distance(model.p);
    let k = random_range(100., 200.);
    if d <  k {
        let mut c = get_random_green(Some((k - d) as u8));
        element.draw(p, &draw, Some(c));
    } else {
        let mut c = get_random_night(Some(10));
        element.draw(p, &draw, Some(c));
    }
}

