use crate::config::CONFIG;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Bird {
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub color: Color,

    pub speed: f32,
    pub acceleration: f32,
}

impl Bird {
    pub fn new() -> Self {
        Self {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            r: CONFIG.bird_size / 2.0,
            color: CONFIG.bird_color,
            speed: 0.0,
            acceleration: CONFIG.gravity,
        }
    }

    pub fn jump(&mut self) {
        self.speed = CONFIG.jump_force;
    }

    pub fn update(&mut self) {
        self.speed += self.acceleration;
        self.y -= self.speed;
    }

    pub fn draw(&self) {
        draw_circle(self.x, self.y, self.r, self.color);
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Default for Bird {
    fn default() -> Self {
        Self::new()
    }
}
