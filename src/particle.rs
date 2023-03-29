use std::ops::Add;
use nannou::color::{Srgb, Srgba, STEELBLUE};
use nannou::Draw;
use nannou::geom::Vec2;
use nannou::prelude::{random_range, Rect};

pub struct Particle {
    location: Vec2,
    velocity: Vec2,
    color: Srgba<u8>,
    draw_fn: fn(&Draw, Vec2, Srgba<u8>),
}

impl Particle {
    pub fn new(location: Vec2, color: Srgba<u8>, velocity: Vec2, draw_fn: fn(&Draw, Vec2, Srgba<u8>)) -> Self {
        Self {
            location,
            velocity,
            color,
            draw_fn,
        }
    }

    pub fn update(&mut self) {
        self.location = self.location.add(self.velocity);
    }

    pub fn display(&self, draw: &Draw) {
        (self.draw_fn)(draw, self.location, self.color);
    }


    pub fn location(&self) -> Vec2 {
        self.location
    }
    pub fn velocity(&self) -> Vec2 {
        self.velocity
    }
    pub fn color(&self) -> Srgba<u8> {
        self.color
    }


    pub fn set_location(&mut self, location: Vec2) {
        self.location = location;
    }
    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }
    pub fn set_color(&mut self, color: Srgba<u8>) {
        self.color = color;
    }
}
