use crate::bird::Bird;
use crate::collisions::circle_intersects_rect;
use crate::obstacle::ObstacleManager;
use macroquad::prelude::*;

pub struct Game {
    pub bird: Bird,
    pub obstacle_manager: ObstacleManager,
    pub game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            bird: Bird::new(),
            obstacle_manager: ObstacleManager::new(),
            game_over: false,
        }
    }

    pub fn update(&mut self) {
        if self.game_over {
            if is_key_pressed(KeyCode::Space) {
                self.reset();
            }
            return;
        }

        if is_key_pressed(KeyCode::Space) {
            self.bird.jump();
        }

        self.bird.update();
        self.obstacle_manager.update();

        self.check_collisions();
    }

    pub fn draw(&self) {
        clear_background(BLACK);
        self.bird.draw();
        self.obstacle_manager.draw();
    }

    fn check_collisions(&mut self) {
        if self.bird.y + self.bird.r > screen_height() || self.bird.y - self.bird.r < 0.0 {
            self.game_over = true;
            return;
        }

        for obstacle in &self.obstacle_manager.obstacles {
            if circle_intersects_rect(
                self.bird.x,
                self.bird.y,
                self.bird.r,
                obstacle.x,
                0.0,
                obstacle.width,
                obstacle.y,
            ) {
                self.game_over = true;
                return;
            }

            if circle_intersects_rect(
                self.bird.x,
                self.bird.y,
                self.bird.r,
                obstacle.x,
                obstacle.y + obstacle.gap,
                obstacle.width,
                screen_height() - obstacle.y - obstacle.gap,
            ) {
                self.game_over = true;
                return;
            }
        }
    }

    pub fn reset(&mut self) {
        self.bird.reset();
        self.obstacle_manager.reset();
        self.game_over = false;
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
