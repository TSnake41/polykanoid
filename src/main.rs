use assets::Assets;
use game::Game;
use raylib::{
    core::drawing::RaylibDraw,
    ffi::{Color, KeyboardKey},
    prelude::{RaylibAudio, RaylibHandle},
};

mod assets;
mod game;
mod intro;

fn main() {
    let rl = raylib::init()
        .title("Polykanoid")
        .width(1366)
        .height(768)
        .fullscreen()
        .vsync()
        .build();

    intro::intro(&rl);

    let raudio = RaylibAudio::init_audio_device();

    rl.set_target_fps(60);
    raudio.set_master_volume(0.4);

    let mut game = Game::default();
    let assets = Assets::load(&raudio);

    let two_players = main_menu(&rl);

    if rl.window_should_close() {
        return;
    }

    game.init(&rl, two_players);

    while !rl.window_should_close() {
        game.update(&rl, &raudio, &assets);
        rl.begin_drawing(|d| game.draw(&rl, &d));
    }
}

fn main_menu(rl: &RaylibHandle) -> bool {
    let logo1p = rl.load_texture("assets/logo1j.png").unwrap();
    let logo2p = rl.load_texture("assets/logo2j.png").unwrap();

    while !rl.window_should_close() {
        rl.begin_drawing(|d| {
            d.clear_background(Color::WHITE);

            d.draw_text("Mode 1 joueur", 400, 200, 32, Color::BLACK);
            d.draw_texture(&logo1p, 200, 200 - 50, Color::WHITE);
            d.draw_text("Mode 2 joueur", 400, 450, 32, Color::BLACK);
            d.draw_texture(&logo2p, 200, 450 - 50, Color::WHITE);
        });

        if rl.is_key_down(KeyboardKey::KEY_Z) {
            return false;
        }

        if rl.is_key_down(KeyboardKey::KEY_X) {
            return true;
        }
    }

    false
}
