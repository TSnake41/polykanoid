mod audio;
mod collisions;
mod components;
mod entities;
mod resources;

use bevy_ecs::{event::Events, query::With, schedule::Schedule, world::World};
use nalgebra::Vector2;
use raylib::{ffi::KeyboardKey, prelude::*, core::text::measure_text};

use crate::assets::Assets;

use self::{
    audio::AudioEvent,
    collisions::{collisions_ball_borders, collisions_ball_bricks, collisions_ball_player},
    components::{Colored, Position, Size},
    entities::ball::{
        ball_reset_event, ball_respawning, draw_ball, update_ball_velocity, Ball, BallResetEvent,
    },
    entities::brick::{draw_brick, Brick, BrickBundle},
    entities::player::{
        ball_retaining_logic, draw_player, player_death, player_movement_logic, Player,
        PlayerBundle, PlayerControls,
    },
    resources::{InputManager, ScreenInfo},
};

const LINES_OF_BRICKS: usize = 5;
const BRICKS_PER_LINE: usize = 20;

pub struct Game {
    brick_size: Vector2<f32>,
    world: World,
    schedule: Schedule,
}

impl Default for Game {
    fn default() -> Game {
        let brick_size = Vector2::default();

        let world = World::new();
        let mut schedule = Schedule::default();

        schedule.add_systems((
            update_ball_velocity,
            ball_retaining_logic,
            collisions_ball_borders,
            collisions_ball_bricks,
            player_death,
            collisions_ball_player,
            player_movement_logic,
            ball_respawning,
            ball_reset_event,
        ));

        Game {
            brick_size,
            schedule,
            world,
        }
    }
}

impl Game {
    pub fn init(&mut self, rl: &RaylibHandle, two_players: bool) {
        let (width, height) = (rl.get_screen_width() as f32, rl.get_screen_height() as f32);
        self.brick_size = Vector2::new(rl.get_screen_width() as f32 / BRICKS_PER_LINE as f32, 40.0);

        // Spawn players
        let mut input_manager = InputManager::default();

        self.world.spawn(PlayerBundle::new(
            rl,
            0,
            Color::BLACK.fade(0.5),
            Color::MAROON,
            PlayerControls {
                left: KeyboardKey::KEY_LEFT,
                right: KeyboardKey::KEY_RIGHT,
                launch: KeyboardKey::KEY_R,
            },
        ));
        input_manager.track(KeyboardKey::KEY_LEFT);
        input_manager.track(KeyboardKey::KEY_RIGHT);
        input_manager.track(KeyboardKey::KEY_R);

        if two_players {
            self.world.spawn(PlayerBundle::new(
                rl,
                20,
                Color::BLUE.fade(0.5),
                Color::BLUEVIOLET,
                PlayerControls {
                    left: KeyboardKey::KEY_A,
                    right: KeyboardKey::KEY_D,
                    launch: KeyboardKey::KEY_O,
                },
            ));

            input_manager.track(KeyboardKey::KEY_A);
            input_manager.track(KeyboardKey::KEY_D);
            input_manager.track(KeyboardKey::KEY_O);
        }

        self.world.insert_resource(input_manager);
        self.world.insert_resource(ScreenInfo { width, height });
        self.world.insert_resource(Events::<AudioEvent>::default());
        self.world
            .insert_resource(Events::<BallResetEvent>::default());

        self.reset_bricks();
    }

    fn reset_bricks(&mut self) {
        // Initialize bricks
        const INITIAL_DOWN_POSITION: f32 = 50.0;

        let mut bricks = vec![];

        for i in 0..LINES_OF_BRICKS {
            for j in 0..BRICKS_PER_LINE {
                bricks.push(BrickBundle {
                    position: components::Position(Vector2::new(
                        j as f32 * self.brick_size.x + self.brick_size.x / 2.0,
                        i as f32 * self.brick_size.y + INITIAL_DOWN_POSITION,
                    )),
                    size: components::Size(self.brick_size),
                    color: components::Colored(if (i + j) % 2 == 0 {
                        Color::GRAY
                    } else {
                        Color::LIGHTGRAY
                    }),
                    brick: Brick,
                });
            }
        }

        self.world.spawn_batch(bricks);
    }

    pub fn update(&mut self, rl: &RaylibHandle, raudio: &RaylibAudio, assets: &Assets) {
        self.world.resource_mut::<InputManager>().update(rl);

        self.schedule.run(&mut self.world);

        for event in self.world.resource_mut::<Events<AudioEvent>>().drain() {
            match event {
                AudioEvent::Destroyed => assets.play_destroyed(raudio),
                AudioEvent::Bounce => assets.play_bounce(raudio),
            }
        }
    }

    pub fn draw(&mut self, rl: &RaylibHandle, d: &RaylibDrawHandle) {
        d.draw_fps(10, 10);

        d.clear_background(Color::RAYWHITE);

        let screeninfo = self.world.get_resource().cloned().unwrap();

        for brick in self
            .world
            .query_filtered::<(&Position, &Size, &Colored), With<Brick>>()
            .iter(&self.world)
        {
            draw_brick(d, brick);
        }

        for player in self
            .world
            .query_filtered::<(&Position, &Size, &Player, &Colored), With<Player>>()
            .iter(&self.world)
        {
            draw_player(d, player, &screeninfo);
        }

        for ball in self
            .world
            .query_filtered::<(&Position, &Ball, &Colored), With<Ball>>()
            .iter(&self.world)
        {
            draw_ball(d, ball);
        }

        // Game over display
        if self
            .world
            .query_filtered::<(), With<Player>>()
            .iter(&self.world)
            .next()
            .is_none()
        {
            // No more player
            d.draw_rectangle_gradient_ex(
                Rectangle::new(0.0, 0.0, screeninfo.width, screeninfo.height),
                Color::BLACK.fade(0.35),
                Color::GRAY.fade(0.35),
                Color::GRAY.fade(0.35),
                Color::BLACK.fade(0.35),
            );

            d.draw_text(
                "GAME OVER",
                (screeninfo.width as i32 - measure_text("GAME OVER", 48)) / 2,
                (screeninfo.height as i32) / 2 - 24,
                48,
                Color::WHITE,
            );
        }
    }
}
