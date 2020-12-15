use macroquad::prelude::Vec2;

pub fn clampx(pos: Vec2, min: f32, max: f32) -> Vec2 {
    let mut new_pos = pos.clone();
    if pos.x() < min {
        new_pos.set_x(min);
    }
    if pos.x() > max {
        new_pos.set_x(max);
    }

    return new_pos;
}

pub fn clampy(pos: Vec2, min: f32, max: f32) -> Vec2 {
    let mut new_pos = pos.clone();
    if pos.y() < min {
        new_pos.set_y(min);
    }
    if pos.y() > max {
        new_pos.set_y(max);
    }

    return new_pos;
}
