use crate::bird::Bird;
use crate::collisions::circle_intersects_rect;
use crate::config::CONFIG;
use crate::obstacle::ObstacleManager;
use macroquad::prelude::*;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

pub struct Game {
    pub birds: Vec<Bird>,
    pub obstacle_manager: ObstacleManager,
    pub game_over: bool,
    pub generation: u32,
    pub high_score: f32,
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
            high_score: 0.0,
        }
    }

    pub fn update(&mut self) {
        if self.game_over {
            self.next_generation();
            return;
        }

        let obstacle_speed = self.obstacle_manager.speed;
        let screen_height = screen_height();
        let screen_width = screen_width();

        let next_obstacle = self
            .obstacle_manager
            .obstacles
            .iter()
            .find(|o| o.x + o.width > self.birds[0].x);

        if let Some(obstacle) = next_obstacle {
            self.birds.par_iter_mut().for_each(|bird| {
                if !bird.is_alive {
                    return;
                }

                let inputs = vec![
                    bird.y / screen_height,
                    bird.speed / 10.0,
                    (obstacle.x - bird.x) / screen_width,
                    obstacle.y / screen_height,
                    obstacle_speed / 10.0,
                ];

                let output = bird.brain.feed_forward(inputs);
                if output[0] > 0.5 {
                    bird.jump();
                }

                bird.update();
            });
        } else {
            self.birds.par_iter_mut().for_each(|bird| {
                if bird.is_alive {
                    bird.update();
                }
            });
        }

        self.obstacle_manager.update();

        self.check_collisions();

        if self.birds.iter().all(|b| !b.is_alive) {
            self.game_over = true;
        }

        let current_max_fitness = self.birds.iter().map(|b| b.fitness).fold(0.0, f32::max);
        if current_max_fitness > self.high_score {
            self.high_score = current_max_fitness;
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
            30.0,
            30.0,
            WHITE,
        );
        let alive_count = self.birds.iter().filter(|b| b.is_alive).count();
        draw_text(&format!("Alive: {}", alive_count), 10.0, 60.0, 30.0, WHITE);

        let current_score = self.birds.iter().map(|b| b.fitness).fold(0.0, f32::max);
        let score_text = format!("Score: {:.0}", current_score);
        let best_text = format!("Best: {:.0}", self.high_score);

        let score_dims = measure_text(&score_text, None, 30, 1.0);
        let best_dims = measure_text(&best_text, None, 30, 1.0);

        draw_text(
            &score_text,
            screen_width() / 2.0 - score_dims.width / 2.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text(
            &best_text,
            screen_width() / 2.0 - best_dims.width / 2.0,
            60.0,
            30.0,
            WHITE,
        );

        let speed_text = format!("Speed: {:.1}", self.obstacle_manager.speed);
        let speed_dims = measure_text(&speed_text, None, 30, 1.0);
        draw_text(
            &speed_text,
            screen_width() - speed_dims.width - 10.0,
            30.0,
            30.0,
            WHITE,
        );
    }

    fn check_collisions(&mut self) {
        let obstacles = &self.obstacle_manager.obstacles;
        let screen_height = screen_height();

        self.birds.par_iter_mut().for_each(|bird| {
            if !bird.is_alive {
                return;
            }

            if bird.y + bird.r > screen_height || bird.y - bird.r < 0.0 {
                bird.is_alive = false;
                return;
            }

            for obstacle in obstacles {
                if circle_intersects_rect(
                    bird.x,
                    bird.y,
                    bird.r,
                    obstacle.x,
                    0.0,
                    obstacle.width,
                    obstacle.y,
                ) || circle_intersects_rect(
                    bird.x,
                    bird.y,
                    bird.r,
                    obstacle.x,
                    obstacle.y + obstacle.gap,
                    obstacle.width,
                    screen_height - obstacle.y - obstacle.gap,
                ) {
                    bird.is_alive = false;
                    break;
                }
            }
        });
    }

    fn next_generation(&mut self) {
        self.birds
            .sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        let top_count = ((CONFIG.population_size as f32 * 0.1) as usize).max(1);
        let top_birds: Vec<Bird> = self.birds.iter().take(top_count).cloned().collect();

        self.birds.clear();

        for bird in &top_birds {
            self.birds.push(Bird::new(Some(bird.brain.clone())));
        }

        while self.birds.len() < CONFIG.population_size {
            let parent = &top_birds[rand::gen_range(0, top_count)];

            let mut children_brain = parent.brain.clone();
            children_brain.mutate(CONFIG.mutation_rate);

            self.birds.push(Bird::new(Some(children_brain)));
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
