use macroquad::prelude::*;

pub struct GameConfig {
    pub gravity: f32,
    pub jump_force: f32,

    pub bird_size: f32,
    pub bird_color: Color,

    pub obstacle_width: f32,
    pub obstacle_color: Color,
    pub obstacle_min_gap: f32,
    pub obstacle_max_gap: f32,
    pub obstacle_min_y: f32,

    pub obstacles_speed: f32,
    pub obstacles_acceleration: f32,
    pub obstacles_gap: f32,
}

pub const CONFIG: GameConfig = GameConfig {
    gravity: -0.25,
    jump_force: 10.0,

    bird_size: 50.0,
    bird_color: WHITE,

    obstacle_width: 30.0,
    obstacle_color: RED,
    obstacle_min_gap: 200.0,
    obstacle_max_gap: 500.0,
    obstacle_min_y: 250.0,

    obstacles_speed: 5.0,
    obstacles_acceleration: 0.001,
    obstacles_gap: 400.0,
};
