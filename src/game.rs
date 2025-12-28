use crate::bird::Bird;
use crate::collisions::circle_intersects_rect;
use crate::obstacle::ObstacleManager;
use macroquad::prelude::*;

pub struct Game {
    pub birds: Vec<Bird>,
    pub obstacle_manager: ObstacleManager,
    pub game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            birds: vec![Bird::new()],
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
            for bird in &mut self.birds {
                if bird.is_alive {
                    bird.jump();
                }
            }
        }

        for bird in &mut self.birds {
            if bird.is_alive {
                bird.update();
            }
        }

        self.obstacle_manager.update();

        self.check_collisions();

        if self.birds.iter().all(|b| !b.is_alive) {
            self.game_over = true;
        }
    }

    pub fn draw(&self) {
        clear_background(BLACK);

        self.obstacle_manager.draw();

        for bird in &self.birds {
            if bird.is_alive {
                bird.draw();
            }
        }
    }

    fn check_collisions(&mut self) {
        for bird in &mut self.birds {
            if !bird.is_alive {
                continue;
            }

            if bird.y + bird.r > screen_height() || bird.y - bird.r < 0.0 {
                bird.is_alive = false;
                continue;
            }

            for obstacle in &self.obstacle_manager.obstacles {
                if circle_intersects_rect(
                    bird.x,
                    bird.y,
                    bird.r,
                    obstacle.x,
                    0.0,
                    obstacle.width,
                    obstacle.y,
                ) {
                    bird.is_alive = false;
                    break;
                }

                if circle_intersects_rect(
                    bird.x,
                    bird.y,
                    bird.r,
                    obstacle.x,
                    obstacle.y + obstacle.gap,
                    obstacle.width,
                    screen_height() - obstacle.y - obstacle.gap,
                ) {
                    bird.is_alive = false;
                    break;
                }
            }
        }
    }

    pub fn reset(&mut self) {
        for bird in &mut self.birds {
            bird.reset();
        }
        self.obstacle_manager.reset();
        self.game_over = false;
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
