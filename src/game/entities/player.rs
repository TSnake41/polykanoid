use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    query::{With, Without},
    system::{Commands, Query, Res},
};
use nalgebra::Vector2;
use raylib::{
    core::{
        drawing::{RaylibDraw, RaylibDrawHandle},
        RaylibHandle,
    },
    ffi::{Color, KeyboardKey},
};

use crate::game::{
    components::{Colored, Position, Size, Velocity},
    resources::{InputManager, ScreenInfo},
};

use super::ball::Ball;

const PLAYER_MAX_LIFE: i32 = 5;
const PLAYER_SPEED: f32 = 10.0;

#[derive(Component)]
pub struct PlayerControls {
    pub left: KeyboardKey,
    pub right: KeyboardKey,
    pub launch: KeyboardKey,
}

#[derive(Component)]
pub struct Player {
    /// Remaining lives
    pub life: i32,

    /// Offset to apply to display UI (scores, lives)
    pub ui_display_offset: i32,

    /// Score
    pub score: u32,

    /// Color of the balls associated to the player.
    pub ball_color: Color,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub position: Position,
    pub size: Size,
    pub player: Player,
    pub controls: PlayerControls,
    pub color: Colored,
}

pub fn player_movement_logic(
    mut query: Query<(&mut Position, &Size, &PlayerControls)>,
    screeninfo: Res<ScreenInfo>,
    input: Res<InputManager>,
) {
    for (mut position, size, controls) in &mut query {
        // player movement logic
        if input.is_key_down(controls.left) {
            position.0.x -= PLAYER_SPEED;
        }

        if position.0.x - size.0.x / 2.0 <= 0.0 {
            position.0.x = size.0.x / 2.0;
        }

        if input.is_key_down(controls.right) {
            position.0.x += PLAYER_SPEED;
        }

        if position.0.x + size.0.x / 2.0 >= screeninfo.width {
            position.0.x = screeninfo.width - size.0.x / 2.0;
        }
    }
}

pub fn player_death(
    player_query: Query<(Entity, &Player)>,
    ball_query: Query<(Entity, &Ball)>,
    mut commands: Commands,
) {
    for (entity, player) in &player_query {
        if player.life <= 0 {
            commands.entity(entity).despawn();

            // Kill all player balls
            for (ball_entity, _) in ball_query.iter().filter(|(_, ball)| ball.owner == entity) {
                commands.entity(ball_entity).despawn();
            }
        }
    }
}

pub fn ball_retaining_logic(
    mut balls_query: Query<(&mut Position, &mut Velocity, &mut Ball), With<Ball>>,
    players_query: Query<(&Position, &PlayerControls), Without<Ball>>,
    input: Res<InputManager>,
    screeninfo: Res<ScreenInfo>,
) {
    for (mut position, mut velocity, mut ball) in &mut balls_query {
        if !ball.active {
            if let Ok((player_position, controls)) = players_query.get(ball.owner) {
                // Move the ball with the player.
                position.0 =
                    Vector2::new(player_position.0.x, screeninfo.height * 7.0 / 8.0 - 30.0);

                // Ball launching logic
                if input.is_key_pressed(controls.launch) {
                    ball.active = true;
                    velocity.0 = Vector2::new(0.0, -10.0);
                }
            } else {
                ball.active = true;
            }
        }
    }
}

pub fn draw_player(
    d: &RaylibDrawHandle,
    (position, size, player, color): (&Position, &Size, &Player, &Colored),
    screeninfo: &ScreenInfo,
) {
    // Draw player bar
    d.draw_rectangle(
        (position.0.x - size.0.x / 2.0) as i32,
        (position.0.y - size.0.y / 2.0) as i32,
        size.0.x as i32,
        size.0.y as i32,
        color.0,
    );

    // Draw player lives
    for i in 0..player.life {
        d.draw_rectangle(
            20 + 40 * i,
            screeninfo.height as i32 - 30 + player.ui_display_offset,
            35,
            10,
            Color::LIGHTGRAY,
        );
    }

    // Display player score
    d.draw_text(
        &format!("Score: {}", player.score),
        0,
        player.ui_display_offset,
        20,
        color.0,
    );
}

impl PlayerBundle {
    /// Initialize the player (prepare it for the game).
    pub fn new(
        rl: &RaylibHandle,
        ui_display_offset: i32,
        color: Color,
        ball_color: Color,
        controls: PlayerControls,
    ) -> Self {
        Self {
            position: Position(Vector2::new(
                rl.get_screen_width() as f32 / 2.0,
                rl.get_screen_height() as f32 * 7.0 / 8.0,
            )),
            size: Size(Vector2::new(rl.get_screen_width() as f32 / 10.0, 20.0)),
            player: Player {
                life: PLAYER_MAX_LIFE,
                ui_display_offset,
                score: 0,
                ball_color,
            },
            controls,
            color: Colored(color),
        }
    }
}
