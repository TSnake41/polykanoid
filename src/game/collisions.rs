use bevy_ecs::{
    entity::Entity,
    event::EventWriter,
    query::{With, Without},
    system::{Commands, Query, Res},
};
use raylib::ffi::Rectangle;

use super::{
    audio::AudioEvent,
    components::{Position, Size, Velocity},
    entities::{ball::Ball, brick::Brick, player::Player},
    resources::ScreenInfo,
};

const BALL_SPEED: f32 = 10.0f32;

pub fn collisions_ball_borders(
    mut ball_query: Query<(Entity, &Position, &Ball, &mut Velocity)>,
    mut audio_writer: EventWriter<AudioEvent>,
    screeninfo: Res<ScreenInfo>,
    mut commands: Commands,
) {
    for (entity, position, ball, mut velocity) in &mut ball_query {
        if position.0.x + ball.radius as f32 >= screeninfo.width
            || position.0.x - ball.radius as f32 <= 0.0
        {
            velocity.0.x *= -1.0;
            audio_writer.send(AudioEvent::Bounce);
        }

        if position.0.y - ball.radius as f32 <= 0.0 {
            velocity.0.y *= -1.0;
            audio_writer.send(AudioEvent::Bounce);
        }

        if position.0.y + ball.radius as f32 >= screeninfo.height {
            // Out of map
            commands.entity(entity).despawn();
        }
    }
}

pub fn collisions_ball_player(
    mut ball_query: Query<(&Position, &Ball, &mut Velocity), Without<Player>>,
    player_query: Query<(&Position, &Size), With<Player>>,
    mut audio_writer: EventWriter<AudioEvent>,
) {
    for (position, ball, mut velocity) in &mut ball_query {
        for (player_position, player_size) in &player_query {
            let r = Rectangle::new(
                player_position.0.x - player_size.0.x / 2.0,
                player_position.0.y - player_size.0.y / 2.0,
                player_size.0.x,
                player_size.0.y,
            );

            if r.check_collision_circle_rec(position.0, ball.radius as f32) && velocity.0.y > 0.0 {
                velocity.0.y *= -1.0;
                velocity.0.x = (position.0.x - player_position.0.x) / (player_size.0.x / 2.0) * BALL_SPEED;
                audio_writer.send(AudioEvent::Bounce);
                break;
            }
        }
    }
}

pub fn collisions_ball_bricks(
    mut player_query: Query<&mut Player, (Without<Brick>, With<Player>)>,
    brick_query: Query<(Entity, &Position, &Size), (With<Brick>, Without<Player>)>,
    mut ball_query: Query<(&Position, &Ball, &mut Velocity), (Without<Brick>, Without<Player>)>,
    mut audio_writer: EventWriter<AudioEvent>,
    mut commands: Commands,
) {
    for (position, ball, mut velocity) in &mut ball_query {
        for (brick_entity, brick_position, brick_size) in &brick_query {
            // Hit below
            if (position.0.y - ball.radius as f32 <= brick_position.0.y + brick_size.0.y / 2.0)
                && (position.0.y - ball.radius as f32
                    > brick_position.0.y + brick_size.0.y / 2.0 + velocity.0.y)
                && ((position.0.x - brick_position.0.x).abs()
                    < brick_size.0.x / 2.0 + ball.radius as f32 * 2.0 / 3.0)
                && velocity.0.y < 0.0
            {
                velocity.0.y *= -1.0;
                audio_writer.send(AudioEvent::Destroyed);

                player_query
                    .get_mut(ball.owner)
                    .expect("Ball without player ?")
                    .score += 1;

                commands.entity(brick_entity).despawn();
            }
            // Hit above
            else if position.0.y + ball.radius as f32 >= brick_position.0.y - brick_size.0.y / 2.0
                && (position.0.y + ball.radius as f32)
                    .partial_cmp(&(brick_position.0.y - brick_size.0.y / 2.0 + velocity.0.y))
                    .unwrap()
                    == std::cmp::Ordering::Less
                && (position.0.x - brick_position.0.x).abs()
                    < brick_size.0.x / 2.0 + ball.radius as f32 * 2.0 / 3.0
                && velocity.0.y > 0.0
            {
                velocity.0.y *= -1.0;

                audio_writer.send(AudioEvent::Destroyed);

                player_query
                    .get_mut(ball.owner)
                    .expect("Ball without player ?")
                    .score += 1;

                commands.entity(brick_entity).despawn();
            }
            // Hit Left
            else if ((position.0.x + ball.radius as f32)
                >= (brick_position.0.x - brick_size.0.x / 2.0))
                && ((position.0.x + ball.radius as f32)
                    < (brick_position.0.x - brick_size.0.x / 2.0 + velocity.0.x))
                && (((position.0.y - brick_position.0.y).abs())
                    < (brick_size.0.y / 2.0 + ball.radius as f32 * 2.0 / 3.0))
                && (velocity.0.x > 0.0)
            {
                velocity.0.x *= -1.0;

                audio_writer.send(AudioEvent::Destroyed);

                player_query
                    .get_mut(ball.owner)
                    .expect("Ball without player ?")
                    .score += 1;

                commands.entity(brick_entity).despawn();
            }
            // Hit right
            else if ((position.0.x - ball.radius as f32)
                <= (brick_position.0.x + brick_size.0.x / 2.0))
                && ((position.0.x - ball.radius as f32)
                    > (brick_position.0.x + brick_size.0.x / 2.0 + velocity.0.x))
                && (((position.0.y - brick_position.0.y).abs())
                    < (brick_size.0.y / 2.0 + ball.radius as f32 * 2.0 / 3.0))
                && (velocity.0.x < 0.0)
            {
                velocity.0.x *= -1.0;

                audio_writer.send(AudioEvent::Destroyed);

                player_query
                    .get_mut(ball.owner)
                    .expect("Ball without player ?")
                    .score += 1;

                commands.entity(brick_entity).despawn();
            }
        }
    }
}
