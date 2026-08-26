#![allow(non_snake_case)]

use core::ffi::{c_char, c_int};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

pub const RAYWHITE: Color = Color {
    r: 245,
    g: 245,
    b: 245,
    a: 255,
};

pub const RED: Color = Color {
    r: 230,
    g: 41,
    b: 55,
    a: 255,
};

pub const SKYBLUE: Color = Color {
    r: 102,
    g: 191,
    b: 255,
    a: 255,
};

pub const YELLOW: Color = Color {
    r: 253,
    g: 249,
    b: 0,
    a: 255,
};

// raylib GamepadButton enum:
//
// UNKNOWN                 = 0
// ...
// RIGHT_FACE_RIGHT        = 6  -> Switch A
// ...
// MIDDLE_RIGHT            = 15 -> Switch Plus
pub const GAMEPAD_BUTTON_A: c_int = 6;
pub const GAMEPAD_BUTTON_PLUS: c_int = 15;

// raylib GamepadAxis
pub const GAMEPAD_AXIS_LEFT_X: c_int = 0;
pub const GAMEPAD_AXIS_LEFT_Y: c_int = 1;

unsafe extern "C" {
    pub fn InitWindow(
        width: c_int,
        height: c_int,
        title: *const c_char,
    );

    pub fn CloseWindow();

    pub fn WindowShouldClose() -> bool;

    pub fn BeginDrawing();
    pub fn EndDrawing();

    pub fn ClearBackground(color: Color);

    pub fn DrawLine(
        start_x: c_int,
        start_y: c_int,
        end_x: c_int,
        end_y: c_int,
        color: Color,
    );

    pub fn DrawRectangle(
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        color: Color,
    );

    pub fn DrawCircle(
        center_x: c_int,
        center_y: c_int,
        radius: f32,
        color: Color,
    );

    pub fn DrawText(
        text: *const c_char,
        x: c_int,
        y: c_int,
        font_size: c_int,
        color: Color,
    );

    pub fn IsGamepadButtonPressed(
        gamepad: c_int,
        button: c_int,
    ) -> bool;

    pub fn GetGamepadAxisMovement(
        gamepad: c_int,
        axis: c_int,
    ) -> f32;

    pub fn GetFrameTime() -> f32;
}