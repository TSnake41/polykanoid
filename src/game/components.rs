use bevy_ecs::component::Component;
use nalgebra::Vector2;
use raylib::ffi::Color;

#[derive(Component)]
pub struct Position(pub Vector2<f32>);

#[derive(Component)]
pub struct Velocity(pub Vector2<f32>);

#[derive(Component)]
pub struct Size(pub Vector2<f32>);

#[derive(Component)]
pub struct Colored(pub Color);
