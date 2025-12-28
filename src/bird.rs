use crate::config::CONFIG;
use crate::neural_network::Network;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Bird {
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub color: Color,

    pub speed: f32,
    pub acceleration: f32,
    pub is_alive: bool,

    pub brain: Network,
    pub fitness: f32,
}

impl Bird {
    pub fn new(brain: Option<Network>) -> Self {
        let brain = brain.unwrap_or_else(|| Network::new(CONFIG.network_layers.to_vec()));

        Self {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            r: CONFIG.bird_size / 2.0,
            color: CONFIG.bird_color,
            speed: 0.0,
            acceleration: CONFIG.gravity,
            is_alive: true,
            brain,
            fitness: 0.0,
        }
    }

    pub fn jump(&mut self) {
        self.speed = CONFIG.jump_force;
    }

    pub fn update(&mut self) {
        self.speed += self.acceleration;
        self.y -= self.speed;
        self.fitness += 1.0;
    }

    pub fn draw(&self) {
        draw_circle(self.x, self.y, self.r, self.color);
    }

    pub fn reset(&mut self) {
        let brain = self.brain.clone();
        *self = Self::new(Some(brain));
    }
}

impl Default for Bird {
    fn default() -> Self {
        Self::new(None)
    }
}
