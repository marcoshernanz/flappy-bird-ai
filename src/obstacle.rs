use crate::config::CONFIG;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Obstacle {
    pub x: f32,
    pub y: f32,
    pub gap: f32,
    pub width: f32,
    pub color: Color,
}

impl Obstacle {
    pub fn new() -> Self {
        let gap = rand::gen_range(CONFIG.obstacle_min_gap, CONFIG.obstacle_max_gap);
        let y = rand::gen_range(
            CONFIG.obstacle_min_y,
            screen_height() - CONFIG.obstacle_min_y - gap,
        );

        Self {
            x: screen_width(),
            y,
            gap,
            width: CONFIG.obstacle_width,
            color: CONFIG.obstacle_color,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, 0.0, self.width, self.y, self.color);
        draw_rectangle(
            self.x,
            self.y + self.gap,
            self.width,
            screen_height() - self.y - self.gap,
            self.color,
        );
    }
}

pub struct ObstacleManager {
    pub obstacles: Vec<Obstacle>,
    pub speed: f32,
    pub acceleration: f32,
    pub gap: f32,
}

impl ObstacleManager {
    pub fn new() -> Self {
        Self {
            obstacles: vec![Obstacle::new()],
            speed: CONFIG.obstacles_speed,
            acceleration: CONFIG.obstacles_acceleration,
            gap: CONFIG.obstacles_gap,
        }
    }

    pub fn update(&mut self) {
        self.speed += self.acceleration;

        for obstacle in &mut self.obstacles {
            obstacle.x -= self.speed;
        }

        self.obstacles.retain(|o| o.x + o.width > 0.0);

        let needs_spawn = match self.obstacles.last() {
            Some(last_obstacle) => screen_width() - last_obstacle.x > self.gap,
            None => true,
        };

        if needs_spawn {
            self.obstacles.push(Obstacle::new());
        }
    }

    pub fn draw(&self) {
        for obstacle in &self.obstacles {
            obstacle.draw();
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Default for ObstacleManager {
    fn default() -> Self {
        Self::new()
    }
}
