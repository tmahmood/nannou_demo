#![feature(drain_filter)]

use std::ops::{Add, Mul, Sub};
use std::time::Duration;
use nannou::noise::*;
use nannou::prelude::*;
use nannou::wgpu::Texture;
use rayon::prelude::*;
use nannou_utils::{draw_background_grid, get_random_blue, get_random_color, get_random_green, get_random_night, get_random_position, get_random_retro, GREEN_PALATE};
use nannou_utils::particle::Particle;
//use ;

const PARTICLE_COUNT: usize = 1000;
const PLATFORM_COUNT: usize = 10;
const MAX_LIGHTENING_ALPHA: u8 = 100;

const SPEED: f64 = 0.5;

type NoiseAlgo = Perlin;

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

struct Platform {
    pub rect: Rect,
    pub color: Srgba<u8>,
    pub stroke: Srgba<u8>,
}

impl Platform {

    pub fn display(&self, draw: &Draw) {
        draw.rect()
            .xy(self.rect.xy())
            .wh(self.rect.wh())
            .stroke(self.stroke)
            .stroke_weight(3.)
            .color(self.color);

        let building_r = self.rect.pad(3.);
        let w_row = (building_r.w() / 13.) + 1.;
        let w_col = (building_r.h() / 13.) + 1.;
        let wh = Vec2::new(1., 1.);

    }

    pub fn new(rect: Rect, color: Srgba<u8>, stroke: Srgba<u8>) -> Self {
        let d = Draw::new();
        Self { rect, color, stroke }
    }
}

struct Circle {
    pub position: Vec2,
    pub r: f32,
    pub color: Srgba<u8>,
}

struct Model {
    noise: NoiseAlgo,
    particles: Vec<Particle>,
    size: Vec2,
    p: Rect,
    explosions: Vec<Circle>,
    platforms: Vec<Platform>,
    lightening: Option<Vec<Point2>>,
    lightening_steps: i32,
    lightening_color: Srgba<u8>,
    lightening_last_time: Duration,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1200, 1000)
        .resized(on_resize)
        .build()
        .unwrap();
    let size = Vec2::new(1200., 1000.);
    let noise = NoiseAlgo::new();
    let mut particles = vec![];
    let mut platforms = vec![];

    for _ii in 0..PARTICLE_COUNT {
        let p = new_random_particle(size);
        particles.push(p);
    }

    for ii in 0..PLATFORM_COUNT + 1 {
        let p = make_skyline(size, app.window_rect(), ii);
        platforms.push(p);
    }

    Model {
        noise,
        size,
        particles,
        p: Rect::from_w_h(0., 0.),
        explosions: vec![],
        platforms,
        lightening: None,
        lightening_steps: 0,
        lightening_color: srgba(203, 241, 245, 0),
        lightening_last_time: Default::default(),
    }
}

fn on_resize(app: &App, model: &mut Model, new_size: Vec2) {
    model.size = new_size;
    model.platforms.clear();
    for ii in 0..PLATFORM_COUNT + 1 {
        let p = make_skyline(new_size, app.window_rect(), ii);
        model.platforms.push(p);
    }
}

fn draw_fn(draw: &Draw, location: Vec2, color: Srgba<u8>) {
    let mut stroke = get_random_retro(Some(100));
    draw.line()
        .start(location)
        .end(location.add(Vec2::new(0., 10.)))
        .color(stroke);
}

fn new_random_particle(size: Vec2) -> Particle {
    let mut p = get_random_position(size);
    p.y = (size.y / 2.) + random_range(0., 500.);
    let v = Vec2::new(
        0.,
        random_range(-9., -4.),
    );
    let c = get_random_retro(Some(200));
    Particle::new(p, c, v, draw_fn)
}

fn make_skyline(size: Vec2, rect: Rect, ii: usize) -> Platform {
    let color = srgba(27, 38, 44, 255);
    let stroke = srgba(28, 39, 45, 225);
    let w = rect.w() / PLATFORM_COUNT as f32;
    let x = rect.left() + w * ii as f32;
    let y = rect.bottom() - random_range(rect.bottom() + 20., rect.bottom() + 100.);
    let h = random_range(100., 300.);
    let mut rt = Rect::from_x_y_w_h(x, y, w, h);
    Platform::new(rt.align_bottom_of(rect), color, stroke)
}


fn update(app: &App, model: &mut Model, _update: Update) {
    let r = app.window_rect();
    for particle in model.particles.iter_mut() {
        particle.update();

        // going out of screen
        if particle.location().y < r.bottom() {
            update_particle(particle, &mut model.explosions, model.size);
        }

        // collision with any platform?
        for platform in model.platforms.iter() {
            let c = Circle { position: particle.location(), r: 5., color: Default::default() };
            if circle_rect_collision(c, platform.rect) {
                update_particle(particle, &mut model.explosions, model.size);
            }
        }
    }
    model.explosions.drain_filter(|v| {
        v.color.alpha < 50
    });
    model.explosions.iter_mut().for_each(|v| {
        v.r += 0.15;
        v.color.alpha -= 5;
    });

    if model.lightening.is_some() {
        let t = model.lightening_color.alpha as i32 + model.lightening_steps;
        model.lightening_color.alpha = t as u8;
        if model.lightening_color.alpha >= MAX_LIGHTENING_ALPHA { model.lightening_steps = -10 }
        if model.lightening_color.alpha == 0 {
            model.lightening = None;
            model.lightening_steps = 0;
        }
    }

    let long_enough = app.duration.since_start - model.lightening_last_time > Duration::from_secs(5);
    // time for a lightening
    if random::<bool>() && model.lightening.is_none() && long_enough {
        let time = app.elapsed_frames() as f32 / 120.0;
        let sn = 0.01 + time.cos() as f64 * 0.005;
        let mut start_position = get_random_position(model.size);
        start_position.y = r.top();

        let mut end_position = get_random_position(model.size);
        end_position.y = r.bottom();

        let mut current_position = start_position;

        let mut points = vec![];

        while current_position.distance(end_position) > 30. {
            let n = end_position.sub(current_position).normalize();
            current_position = current_position + n + vec2(
                model.noise.get([sn * current_position.x as f64, sn * current_position.y as f64, 0.0]) as f32,
                model.noise.get([sn * current_position.x as f64, sn * current_position.y as f64, 1.0]) as f32,
            );
            points.push(current_position.clone());
        }
        model.lightening = Some(points);
        model.lightening_steps = 10;
        model.lightening_color.alpha = 0;
        model.lightening_last_time = app.duration.since_start;
    }
}

fn update_particle(particle: &mut Particle, explosions: &mut Vec<Circle>, size: Vec2) {
    let mut color = particle.color();
    color.alpha = 150;
    explosions.push(Circle {
        position: particle.location(),
        r: 2.,
        color,
    });
    let p = new_random_particle(size);
    particle.set_color(p.color());
    particle.set_location(p.location());
    particle.set_velocity(p.velocity())
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(BLACK);

    if let Some(lightening) = model.lightening.as_ref() {
        draw.polyline().points(lightening.clone()).color(model.lightening_color).finish();
    }

    if let Some(points) = model.lightening.clone() {
        draw.polyline().weight(3.).points(points.clone()).color(model.lightening_color).finish();
    }

    for explosion in model.explosions.iter() {
        draw.ellipse().xy(explosion.position).radius(explosion.r).color(explosion.color);
    }

    for particle in model.particles.iter() {
        particle.display(&draw);
    }

    for platform in model.platforms.iter() {
        platform.display(&draw);
    }

    // draw_background_grid(&app, &draw);
    draw.to_frame(app, &frame).unwrap();
}


fn circle_rect_collision(circle: Circle, rect: Rect) -> bool {
    // temporary variables to set edges for testing
    let mut test_x = circle.position.x;
    let mut test_y = circle.position.y;

    // which edge is closest?
    if circle.position.x < rect.left() {
        test_x = rect.left();      // test left edge
    } else if circle.position.x > rect.right() {
        test_x = rect.right();
    }  // right edge

    if circle.position.y < rect.top() {
        test_y = rect.top();      // top edge
    } else if circle.position.y > rect.bottom() {
        test_y = rect.bottom();
    }

    // get distance from closest edges
    let dist_x = circle.position.x - test_x;
    let dist_y = circle.position.y - test_y;
    let distance = ((dist_x * dist_x) + (dist_y * dist_y));

    // if the distance is less than the radius, collision!
    if distance <= circle.r {
        return true;
    }
    return false;
}