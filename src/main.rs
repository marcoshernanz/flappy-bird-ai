use macroquad::prelude::*;

struct GameConfig {
    gravity: f32,
    jump_force: f32,

    bird_size: f32,
    bird_color: Color,

    obstacle_width: f32,
    obstacle_color: Color,
    obstacle_min_gap: f32,
    obstacle_max_gap: f32,
    obstacle_min_y: f32,

    obstacles_speed: f32,
    obstacles_acceleration: f32,
    obstacles_gap: f32,
}

const CONFIG: GameConfig = GameConfig {
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

struct Bird {
    x: f32,
    y: f32,
    r: f32,
    color: Color,

    speed: f32,
    acceleration: f32,
}

impl Bird {
    fn new() -> Bird {
        Bird {
            x: (screen_width()) / 2.0,
            y: (screen_height()) / 2.0,
            r: CONFIG.bird_size / 2.0,
            color: CONFIG.bird_color,
            speed: 0.0,
            acceleration: CONFIG.gravity,
        }
    }
}

struct Obstacle {
    x: f32,
    y: f32,
    gap: f32,
    width: f32,
    color: Color,
}

impl Obstacle {
    fn new() -> Obstacle {
        Obstacle {
            x: screen_width(),
            y: rand::gen_range(
                CONFIG.obstacle_min_y,
                // screen_height() - CONFIG.obstacle_min_y,
                CONFIG.obstacle_min_y + 10.0,
            ),
            gap: rand::gen_range(CONFIG.obstacle_min_gap, CONFIG.obstacle_max_gap),
            width: CONFIG.obstacle_width,
            color: CONFIG.obstacle_color,
        }
    }
}

struct Obstacles {
    obstacles: Vec<Obstacle>,
    speed: f32,
    acceleration: f32,
    gap: f32,
}

impl Obstacles {
    fn new() -> Obstacles {
        Obstacles {
            obstacles: vec![Obstacle::new()],
            speed: CONFIG.obstacles_speed,
            acceleration: CONFIG.obstacles_acceleration,
            gap: CONFIG.obstacles_gap,
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("AI Flappy Bird"),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    show_mouse(false);

    let mut bird = Bird::new();
    let mut obstacles = Obstacles::new();

    loop {
        handle_bird_jump(&mut bird);
        apply_physics(&mut bird, &mut obstacles);
        check_collisions(&mut bird, &mut obstacles);
        handle_draw(&bird, &obstacles);

        next_frame().await
    }
}

fn handle_bird_jump(bird: &mut Bird) {
    if is_key_pressed(KeyCode::Space) {
        bird.speed = CONFIG.jump_force;
    }
}

fn apply_physics(bird: &mut Bird, obstacles: &mut Obstacles) {
    bird.speed += bird.acceleration;
    bird.y -= bird.speed;

    obstacles.speed += obstacles.acceleration;

    let mut to_remove = Vec::new();
    let mut to_add = 0;
    for (index, obstacle) in obstacles.obstacles.iter_mut().enumerate() {
        obstacle.x -= obstacles.speed;
        if obstacle.x + obstacle.width < 0.0 {
            to_remove.push(index);
        } else if obstacle.x + obstacle.width <= screen_width() - obstacles.gap
            && obstacle.x + obstacle.width + obstacles.speed > screen_width() - obstacles.gap
        {
            to_add += 1;
        }
    }

    for index in to_remove {
        obstacles.obstacles.remove(index);
    }
    for _ in 0..to_add {
        obstacles.obstacles.push(Obstacle::new());
    }
}

fn check_collisions(bird: &mut Bird, obstacles: &mut Obstacles) {
    for obstacle in obstacles.obstacles.iter() {
        if bird.y + bird.r > screen_height()
            || bird.y - bird.r < 0.0
            || circle_intersects_rect(
                bird.x,
                bird.y,
                bird.r,
                obstacle.x,
                0.0,
                obstacle.width,
                obstacle.y,
            )
            || circle_intersects_rect(
                bird.x,
                bird.y,
                bird.r,
                obstacle.x,
                obstacle.y + obstacle.gap,
                obstacle.width,
                screen_height() - obstacle.y - obstacle.gap,
            )
        {
            *bird = Bird::new();
            *obstacles = Obstacles::new();
            return;
        }
    }
}

fn handle_draw(bird: &Bird, obstacles: &Obstacles) {
    clear_background(BLACK);

    draw_circle(bird.x, bird.y, bird.r, bird.color);

    for obstacle in obstacles.obstacles.iter() {
        draw_rectangle(obstacle.x, 0.0, obstacle.width, obstacle.y, obstacle.color);
        draw_rectangle(
            obstacle.x,
            obstacle.y + obstacle.gap,
            obstacle.width,
            screen_height() - obstacle.y - obstacle.gap,
            obstacle.color,
        );
    }
}

fn circle_intersects_rect(
    circle_x: f32,
    circle_y: f32,
    circle_r: f32,
    rect_x: f32,
    rect_y: f32,
    rect_w: f32,
    rect_h: f32,
) -> bool {
    let closest_x = circle_x.clamp(rect_x, rect_x + rect_w);
    let closest_y = circle_y.clamp(rect_y, rect_y + rect_h);

    let distance_x = circle_x - closest_x;
    let distance_y = circle_y - closest_y;

    let distance_squared = (distance_x * distance_x) + (distance_y * distance_y);

    distance_squared < (circle_r * circle_r)
}
