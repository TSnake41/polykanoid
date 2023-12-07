use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    event::{Event, EventReader},
    query::{With, Without},
    system::{Commands, Query},
};
use nalgebra::Vector2;
use raylib::{
    core::drawing::{RaylibDraw, RaylibDrawHandle},
    ffi::Color,
};

use super::player::Player;
use crate::game::components::{Colored, Position, Velocity};

#[derive(Component)]
pub struct Ball {
    pub radius: i32,
    pub active: bool,
    pub owner: Entity,
}

#[derive(Bundle)]
pub struct BallBundle {
    pub position: Position,
    pub velocity: Velocity,
    pub ball: Ball,
    pub color: Colored,
}

#[derive(Event)]
pub struct BallResetEvent {
    pub target: Option<Entity>,
}

pub fn update_ball_velocity(mut query: Query<(&mut Position, &Velocity), With<Ball>>) {
    for (mut position, velocity) in &mut query {
        position.0 += velocity.0;
    }
}

fn reset_ball(
    position: &mut Position,
    velocity: &mut Velocity,
    ball: &mut Ball,
    player_position: &Position,
) {
    position.0 = Vector2::new(player_position.0.x, player_position.0.y * 8.0 / 7.0 - 30.0);
    velocity.0 = Vector2::zeros();
    ball.radius = 7;
    ball.active = false;
}

pub fn ball_reset_event(
    player_query: Query<&Position, With<Player>>,
    mut ball_query: Query<(&mut Position, &mut Velocity, &mut Ball), Without<Player>>,
    mut event_reader: EventReader<BallResetEvent>,
) {
    for event in event_reader.read() {
        if let Some(entity) = event.target {
            // Only apply reset on this entity.
            if let Ok((mut position, mut velocity, mut ball)) = ball_query.get_mut(entity) {
                // Get the associated player.
                if let Ok(player_position) = player_query.get(ball.owner) {
                    reset_ball(&mut position, &mut velocity, &mut ball, player_position);
                } else {
                    position.0 = Vector2::zeros();
                    velocity.0 = Vector2::new(0.0, 10.0);
                }
            }
        } else {
            // Apply to all entities.
            for (mut position, mut velocity, mut ball) in &mut ball_query {
                // Get the associated player.
                if let Ok(player_position) = player_query.get(ball.owner) {
                    reset_ball(&mut position, &mut velocity, &mut ball, player_position);
                } else {
                    position.0 = Vector2::zeros();
                    velocity.0 = Vector2::new(0.0, 10.0);
                }
            }
        }
    }
}

pub fn draw_ball(d: &RaylibDrawHandle, (position, ball, color): (&Position, &Ball, &Colored)) {
    // Draw ball
    d.draw_circle_v(position.0, ball.radius as f32, color.0);
}

pub fn ball_respawning(
    ball_query: Query<&Ball, Without<Player>>,
    mut player_query: Query<(Entity, &Position, &mut Player), With<Player>>,
    mut commands: Commands,
) {
    for (player_entity, position, mut player) in &mut player_query {
        if ball_query.iter().all(|ball| ball.owner != player_entity) {
            // Create a new ball for this player.
            commands.spawn(BallBundle::new(player_entity, player.ball_color, position));

            // Decrease life count.
            player.life -= 1;
        }
    }
}

impl BallBundle {
    pub fn new(player: Entity, color: Color, player_position: &Position) -> Self {
        Self {
            position: Position(Vector2::new(
                player_position.0.x,
                player_position.0.y * 8.0 / 7.0 - 30.0,
            )),
            velocity: Velocity(Vector2::zeros()),
            ball: Ball {
                radius: 7,
                active: false,
                owner: player,
            },
            color: Colored(color),
        }
    }
}
