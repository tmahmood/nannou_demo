#![feature(drain_filter)]

use std::time::Duration;
use nannou::noise::*;
use nannou::prelude::*;
use rayon::prelude::*;
use nannou_utils::{draw_background_grid, get_random_blue, get_random_color, get_random_green, get_random_position, get_random_retro, GREEN_PALATE};
use nannou_utils::particle::Particle;
//use ;

const PARTICLE_COUNT: usize = 1000;
const PLATFORM_COUNT: usize = 10;

const SPEED: f64 = 0.5;

type NoiceAlgo = Perlin;

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

struct Platform {
    pub rect: Rect,
    pub color: Srgba<u8>,
}

struct Circle {
    pub position: Vec2,
    pub r: f32,
    pub color: Srgba<u8>,
}

struct Model {
    noise: NoiceAlgo,
    particles: Vec<Particle>,
    size: Vec2,
    p: Rect,
    explosions: Vec<Circle>,
    platforms: Vec<Platform>,
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
    let mut platforms = vec![];
    for ii in 0..PARTICLE_COUNT {
        let p = new_random_particle(size);
        particles.push(p);
    }
    for ii in 0..PLATFORM_COUNT {
        let p = new_random_platform(size);
        platforms.push(p);
    }
    Model {
        noise,
        size,
        particles,
        p: Rect::from_w_h(0., 0.),
        explosions: vec![],
        platforms,
    }
}

fn on_resize(_app: &App, model: &mut Model, new_size: Vec2) {
    model.size = new_size;
    model.platforms.clear();
    for ii in 0..PLATFORM_COUNT {
        let p = new_random_platform(new_size);
        model.platforms.push(p);
    }
}

fn draw_fn(draw: &Draw, location: Vec2, color: Srgba<u8>) {
    let mut stroke = get_random_retro(Some(100));
    draw.ellipse()
        .xy(location)
        .radius(random_range(2., 5.))
        .stroke(color)
        .stroke_weight(2.)
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

fn new_random_platform(size: Vec2) -> Platform {
    let mut p = get_random_position(size);
    let color = get_random_retro(Some(200));
    let w = random_range(40., 100.);
    Platform {
        rect: Rect::from_xy_wh(p, Vec2::new(w, 10.)),
        color,
    }
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
        v.color.alpha < 20
    });
    model.explosions.iter_mut().for_each(|v| {
        v.r += 3. / app.time;
        v.color.alpha -= 5;
    })
}

fn update_particle(particle: &mut Particle, explosions: &mut Vec<Circle>, size: Vec2) {
    let mut color = particle.color();
    color.alpha = 100;
    explosions.push(Circle {
        position: particle.location(),
        r: 5.,
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

    for explosion in model.explosions.iter() {
        draw.ellipse().xy(explosion.position).radius(explosion.r).color(explosion.color);
    }
    for particle in model.particles.iter() {
        particle.display(&draw);
    }
    for platform in model.platforms.iter() {
        draw.rect()
            .xy(platform.rect.xy())
            .wh(platform.rect.wh())
            .color(platform.color);
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