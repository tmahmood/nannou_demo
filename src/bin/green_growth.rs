use nannou::noise::*;
use nannou::prelude::*;
use nannou_utils::{draw_soft_bg, get_random_green};

const DISC_COUNT: usize = 5000;

const SPEED: f64 = 0.5;

type NoiseAlgo = Perlin;

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}


//Find the curl of the noise field based on on the noise value at the location of a disc
fn compute_curl(noise: &NoiseAlgo, x: f64, y: f64) -> [f32; 2] {
    let eps = 0.01;
    //Find rate of change in X direction
    let n1 = noise.get([x + eps, y]);
    let n2 = noise.get([x - eps, y]);
    //Average to find approximate derivative
    let a = (n1 - n2) / (2. * eps);
    //Find rate of change in Y direction
    let n1 = noise.get([x, y + eps]);
    let n2 = noise.get([x, y - eps]);
    //Average to find approximate derivative
    let b = (n1 - n2) / (2. * eps);
    //Curl
    return [b as f32, -a as f32];
}

#[derive(Debug)]
struct Curve {
    start_x: f64,
    start_y: f64,
    color: Srgba<u8>,
}

impl Curve {
    pub fn get_a_curve(&self, noise: &NoiseAlgo) -> Vec<(Point2, Srgba<u8>)> {
        let mut positions = vec![(Point2::new(self.start_x as f32, self.start_y as f32), self.color)];
        let mut c = self.color;
        for ii in 0..15 {
            let last = positions.last().clone().unwrap();
            let [x, y] = compute_curl(noise, last.0.x as f64 / SPEED, last.0.y as f64 / SPEED);
            positions.push((Point2::new(last.0.x + x, last.0.y + y), c.clone()));
            c.alpha -= ii;
        }
        positions
    }
}

struct Model {
    discs: Vec<Curve>,
    poly_points: Vec<Vec<(Point2, Srgba<u8>)>>,
    noise: NoiseAlgo,
    size: Vec2,
}

fn get_random_position(size: Vec2) -> (f64, f64) {
    (
        random_range(-size.x as f64 / 2., size.x as f64 / 2.),
        random_range(-size.y as f64 / 2., size.y as f64 / 2.),
    )
}

fn model(app: &App) -> Model {
    app.new_window()
        .resized(on_resize)
        .build()
        .unwrap();

    let size = Vec2::new(512., 512.);
    let mut simplex = NoiseAlgo::new();
    simplex.set_seed(random());
    let (discs, poly_points) = prepare_drawing_things(&simplex,  size.clone());
    Model { discs, poly_points, noise: simplex, size}
}

fn on_resize(_app: &App, model: &mut Model, new_size: Vec2) {
    model.size = new_size;
    let (discs, poly_points) = prepare_drawing_things(&model.noise, new_size);
    model.discs = discs;
    model.poly_points = poly_points;

}

fn prepare_drawing_things(noise: &NoiseAlgo, new_size: Vec2) -> (Vec<Curve>, Vec<Vec<(Point2, Srgba<u8>)>>) {
    let mut discs = vec![];
    let mut poly_points = vec![];
    for ii in 0..DISC_COUNT {
        let c = get_random_green(Some(244));
        let (x, y) = get_random_position(Vec2::new(new_size.x, new_size.y));
        let disc = Curve { start_x: x, start_y: y, color: c };
        poly_points.push(disc.get_a_curve(&noise));
        discs.push(disc);
    }
    (discs, poly_points)
}


fn update(_app: &App, model: &mut Model, _update: Update) {
    model.poly_points.clear();
    for disc in model.discs.iter_mut() {
        let [x, y] = compute_curl(&model.noise, disc.start_x / 0.4, disc.start_y / 0.4);
        disc.start_x = disc.start_x + x as f64;
        disc.start_y = disc.start_y + y as f64;
        model.poly_points.push(disc.get_a_curve(&model.noise));
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw_soft_bg(&draw, app, BLACK, 0.01);
    //draw_background_grid(app, &draw);
    //draw.background().color(BLACK);
    for poly_point in model.poly_points.iter() {
        draw.polyline().weight(2.).points_colored(poly_point.clone()).finish();
    }
    draw.to_frame(app, &frame).unwrap();
}

