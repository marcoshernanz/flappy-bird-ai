pub fn circle_intersects_rect(
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
