use std::ops::Div;
use std::str::FromStr;
use nannou::draw::properties::ColorScalar;
use nannou::prelude::*;
use nannou_utils::{C8, get_from_palate, get_random_blue, get_random_color, get_random_from_palette, get_random_green, GREEN_PALATE, map_sin, Pt, srgba8_t};

const COLORS: [(u8, u8, u8); 5] = [
    (255, 135, 135),
    (248, 196, 180),
    (229, 235, 178),
    (188, 226, 158),
    (39, 225, 193) ,
];

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct SinWave {
    r: f32,
    n: f32,
    h: f32,
}

impl SinWave {
    pub fn new(r: f32, n: f32, h: f32) -> Self {
        Self { r, n, h }
    }

    pub fn radius(&self, t: f32) -> f32 {
        self.r + (t * self.n).sin() * self.h
    }
}

struct Guilloche {
    inner: SinWave,
    outer: SinWave,
    nodes: f32,
    div: f32,
    color: C8,
}

impl Guilloche {
    pub fn new(inner: SinWave, outer: SinWave, nodes: f32, div: f32, color: C8) -> Self {
        Self {
            inner,
            outer,
            nodes,
            div,
            color,
        }
    }

    pub fn display(&self, draw: &Draw) {
        let bound = 2. * PI * self.div;
        let mut t = 0.;
        let mut p = vec![];
        while t < bound {
            let r0 = self.inner.radius(t);
            let r1 = self.outer.radius(t);
            let range = (r1 - r0) * 0.5;
            let mid = r0 + range;
            let r = mid + (t * self.nodes / self.div).sin() * range;
            let x = t.cos() * r;
            let y = t.sin() * r;
            p.push(vec2(x, y));
            t += 0.01;
        }
        draw.polyline().points(p).color(self.color);
    }
}

struct Model {
    elms: Vec<Guilloche>,
}

fn model(app: &App) -> Model {
    let r = app.window_rect();

    let elms = vec![
        Guilloche::new(
            SinWave::new(50., 6.0, 10.0),
            SinWave::new(120., 12., 10.),
            137., 37., srgba8_t(COLORS[2], 240)),
        Guilloche::new(
            SinWave::new(120., 12.0, 10.0),
            SinWave::new(220., 18., 30.),
            141., 41., srgba8_t(COLORS[1], 240)),
        Guilloche::new(
            SinWave::new(220., 18.0, 30.0),
            SinWave::new(350., 24., 20.),
            164., 53., srgba8_t(COLORS[0], 240)),
        Guilloche::new(
            SinWave::new(350., 24.0, 20.0),
            SinWave::new(650., 28., 40.),
            164., 53., srgba8_t(COLORS[4], 240)),
    ];
    Model {
        elms
    }
}


fn update(app: &App, model: &mut Model, _update: Update) {
    let t = app.elapsed_frames() as f32 / 20.;

    let mut prev_h = -1.;
    for (ii, elm) in model.elms.iter_mut().enumerate() {
        let m = map_sin(t * 1.1, 0., 30.);
        elm.outer.h = m;
        if prev_h > 0. {
            elm.inner.h = prev_h;
        }
        prev_h = m;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(BLACK);
    model.elms.iter().for_each(|e| {
        e.display(&draw);
    });
    draw.to_frame(app, &frame).unwrap();
}
