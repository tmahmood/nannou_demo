use nannou::noise::*;
use nannou::prelude::*;
use rayon::prelude::*;
use nannou_utils::{draw_background_grid, get_random_blue, get_random_color, get_random_green, get_random_position, get_random_retro, GREEN_PALATE};
use nannou_utils::particle::Particle;
//use ;

const PARTICLE_COUNT: usize = 3000;

const SPEED: f64 = 0.5;

type NoiceAlgo = Perlin;

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

struct Model {
    noise: NoiceAlgo,
    particles: Vec<Particle>,
    size: Vec2,
    p: Rect,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1200, 1000)
        .resized(on_resize)
        .build()
        .unwrap();
    let size = Vec2::new(1200., 1000.);
    let noise = NoiceAlgo::new();
    let mut particles = vec![];
    for ii in 0..PARTICLE_COUNT {
        let p = new_random_particle(size);
        particles.push(p);
    }
    Model {
        noise,
        size,
        particles,
        p: Rect::from_w_h(0., 0.),
    }
}

fn on_resize(_app: &App, model: &mut Model, new_size: Vec2) {
    model.size = new_size;
}

fn draw_fn(draw: &Draw, location: Vec2, color: Srgba<u8>) {
    let mut stroke = get_random_retro(Some(100));
    draw.ellipse()
        .xy(location)
        .radius(random_range(2., 5.))
        .stroke(color)
        .stroke_weight(4.)
        .color(stroke);
}

fn new_random_particle(size: Vec2) -> Particle {
    let mut p = get_random_position(size);
    p.y = (size.y / 2.) + random_range(0., 500.);
    let v = Vec2::new(
        0.,
        random_range(-1.5, -1.),
    );
    let c = get_random_retro(Some(200));
    Particle::new(p, c, v, draw_fn)
}


fn update(app: &App, model: &mut Model, _update: Update) {
    let r = app.window_rect();
    for particle in model.particles.iter_mut() {
        particle.update();
        if particle.location().y < r.bottom() {
            let p = new_random_particle(model.size);
            particle.set_color(p.color());
            particle.set_location(p.location());
            particle.set_velocity(p.velocity())
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(BLACK);
    for particle in model.particles.iter() {
        particle.display(&draw);
    }
    //draw_circle(&draw, 0., 0., 30.);
    draw.to_frame(app, &frame).unwrap();
}

fn draw_circle(draw: &Draw, x: f32, y: f32, r: f32) {
    draw.ellipse().no_fill().stroke(WHITE).x_y(x, y).radius(r).color(WHITE).finish();
    if r > 2. {
        draw_circle(draw, x + r / 2., y, r / 2.);
        draw_circle(draw, x - r / 2., y, r / 2.);
    }
}


