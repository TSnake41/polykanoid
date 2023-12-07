use bevy_ecs::{bundle::Bundle, component::Component};
use raylib::core::drawing::{RaylibDraw, RaylibDrawHandle};

use crate::game::components::{Colored, Position, Size};

#[derive(Component)]
pub struct Brick;

#[derive(Bundle)]
pub struct BrickBundle {
    pub position: Position,
    pub size: Size,
    pub color: Colored,
    pub brick: Brick,
}

pub fn draw_brick(d: &RaylibDrawHandle, (position, size, color): (&Position, &Size, &Colored)) {
    d.draw_rectangle(
        (position.0.x - size.0.x / 2.0) as i32,
        (position.0.y - size.0.y / 2.0) as i32,
        size.0.x as i32,
        size.0.y as i32,
        color.0,
    );
}
