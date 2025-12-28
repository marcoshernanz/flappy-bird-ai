use crate::bird::Bird;
use crate::collisions::circle_intersects_rect;
use crate::config::CONFIG;
use crate::obstacle::ObstacleManager;
use macroquad::prelude::*;

pub struct Game {
    pub birds: Vec<Bird>,
    pub obstacle_manager: ObstacleManager,
    pub game_over: bool,
    pub generation: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            birds: (0..CONFIG.population_size)
                .map(|_| Bird::new(None))
                .collect(),
            obstacle_manager: ObstacleManager::new(),
            game_over: false,
            generation: 1,
        }
    }

    pub fn update(&mut self) {
        if self.game_over {
            self.next_generation();
            return;
        }

        let next_obstacle = self
            .obstacle_manager
            .obstacles
            .iter()
            .find(|o| o.x + o.width > self.birds[0].x);

        if let Some(obstacle) = next_obstacle {
            for bird in &mut self.birds {
                if bird.is_alive {
                    let inputs = vec![
                        bird.y / screen_height(),
                        bird.speed / 10.0,
                        (obstacle.x - bird.x) / screen_width(),
                        obstacle.y / screen_height(),
                    ];

                    let output = bird.brain.feed_forward(inputs);
                    if output[0] > 0.5 {
                        bird.jump();
                    }
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

        draw_text(
            &format!("Generation: {}", self.generation),
            10.0,
            20.0,
            30.0,
            WHITE,
        );
        let alive_count = self.birds.iter().filter(|b| b.is_alive).count();
        draw_text(&format!("Alive: {}", alive_count), 10.0, 50.0, 30.0, WHITE);
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

    fn next_generation(&mut self) {
        let best_bird = self
            .birds
            .iter()
            .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
            .unwrap()
            .clone();

        self.birds.clear();

        self.birds.push(Bird::new(Some(best_bird.brain.clone())));

        for _ in 1..CONFIG.population_size {
            let mut new_brain = best_bird.brain.clone();
            new_brain.mutate(CONFIG.mutation_rate);
            self.birds.push(Bird::new(Some(new_brain)));
        }

        self.obstacle_manager.reset();
        self.game_over = false;
        self.generation += 1;
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
