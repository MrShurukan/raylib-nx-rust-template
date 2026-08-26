#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

mod runtime;
mod raylib;

use core::ffi::c_int;

use raylib::{
    App,
    Button,
    Color,
    Rect,
    Text,
    Vec2,
};

#[no_mangle]
pub extern "C" fn rust_main() -> c_int {
    let Some(mut app) =
        App::new(
            1280,
            720,
            "Rust Switch",
        )
    else {
        return 1;
    };

    app.set_target_fps(60);

    let gamepad = app.gamepad(0);

    let mut player_pos =
        Vec2::new(640.0, 360.0);

    let instructions = Text::new(
        "Left stick: move   A: color",
    );

    let mut alternate = false;

    while app.running() {
        let dt = app.delta_time();

        let stick =
            gamepad
                .left_stick()
                .deadzone(0.15);

        player_pos += stick * 400.0 * dt;

        player_pos = player_pos.clamp(
            Vec2::new(32.0, 32.0),
            Vec2::new(1248.0, 688.0),
        );

        if gamepad.pressed(Button::A) {
            alternate = !alternate;
        }

        let color =
            if alternate {
                Color::GREEN
            } else {
                Color::SKY_BLUE
            };

        let mut frame =
            app.begin_frame(Color::BLACK);

        frame.prepared_text(
            &instructions,
            Vec2::new(40.0, 40.0),
            28,
            Color::RAY_WHITE,
        );

        frame.line(
            Vec2::new(40.0, 90.0),
            Vec2::new(1240.0, 90.0),
            3.0,
            Color::RED,
        );

        frame.rect(
            Rect::new(
                40.0,
                130.0,
                250.0,
                100.0,
            ),
            Color::RED,
        );

        frame.circle(
            player_pos,
            32.0,
            color,
        );

        // EndDrawing() happens automatically here.
    }

    // CloseWindow() automatically happens
    // when App is dropped.

    0
}