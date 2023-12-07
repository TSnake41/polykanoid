use raylib::{
    core::{drawing::RaylibDraw, RaylibHandle},
    ffi::Color,
};

pub fn intro(rl: &RaylibHandle) {
    let (w, h) = (rl.get_screen_width(), rl.get_screen_height());

    let logo_raylib = rl.load_texture("assets/logo_raylib.png").unwrap();
    let logo_raylib_rs = rl.load_texture("assets/logo_raylib_rust.png").unwrap();

    let mut time = 0.0;

    while !rl.window_should_close() && time < 4.0 {
        let opacity = if time < 1.0 {
            // Linear fade in
            time
        } else if time > 3.0 {
            // Linear fade out
            4.0 - time
        } else {
            1.0
        };

        rl.begin_drawing(|d| {
            d.clear_background(Color::WHITE);

            d.draw_text(
                "Polykanoid",
                (w - logo_raylib.as_raw().width * 2) / 2 + 40,
                (h - logo_raylib.as_raw().height) / 2 - 80,
                36,
                Color::BLACK.fade(opacity),
            );

            d.draw_texture(
                &logo_raylib,
                (w - logo_raylib.as_raw().width * 2) / 2 - 16,
                (h - logo_raylib.as_raw().height) / 2,
                Color::WHITE.fade(opacity),
            );

            d.draw_texture(
                &logo_raylib_rs,
                w / 2 + 16,
                (h - logo_raylib_rs.as_raw().height) / 2,
                Color::WHITE.fade(opacity),
            );

            //d.draw_text(
            //    "David \"Dacode45\" Ayeke (original)\nTeddy Astie (améliorations, sons, raylib 5.0, adaptation P++)\n\nraylib-rs par DeltaPHC, Mia Ayeke, Teddy Astie\nraylib par Raymon Santamaria et al.",
            //    (w - logo_raylib.as_raw().width * 2) / 2 - 16,
            //    (h + logo_raylib.as_raw().height) / 2 + 16,
            //    10,
            //    Color::BLACK.fade(opacity),
            //);
        });

        time += rl.get_frame_time();
    }
}
