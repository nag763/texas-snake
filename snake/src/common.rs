use bevy::{math::Vec2, render::color::Color};

pub const APP_TITLE: &str = "TI Snake";

/// The screen height.
pub const SCREEN_HEIGHT: f32 = 480.;
pub const MAX_SCREEN_HEIGHT: f32 = SCREEN_HEIGHT / 2.;
pub const MIN_SCREEN_HEIGHT: f32 = -MAX_SCREEN_HEIGHT;

/// The screen width.
pub const SCREEN_WIDTH: f32 = 640.;
pub const MAX_SCREEN_WIDTH: f32 = SCREEN_WIDTH / 2.;
pub const MIN_SCREEN_WIDTH: f32 = -MAX_SCREEN_WIDTH;

/// The snake head size, same size for each queue member
pub const SNAKE_SIZE: f32 = 10f32;
/// The snake dimensions
pub const SNAKE_DIMENSIONS: Vec2 = Vec2::splat(SNAKE_SIZE);
/// The snake speed
pub const SNAKE_SPEED_FACTOR: f32 = SNAKE_SIZE / 1.8;

/// The bonus diameter
pub const BONUS_DIAMETER: f32 = 10f32;

/// The size of each border
pub const BORDER_SIZE: f32 = 15f32;

pub const NORMAL_BUTTON: Color = Color::DARK_GRAY;
pub const HOVERED_BUTTON: Color = Color::GRAY;

pub const CHANCE_OF_EXTRA_BONUS: f64 = 0.10f64;
pub const EXTRA_BONUS_RGB: (f32, f32, f32) = (202f32 / 256f32, 138f32 / 256f32, 4f32 / 265f32);
pub const TIME_FOR_BONUS: f32 = 10f32;

/// The font name
pub const FONT_ASSET_NAME: &str = "score_font.otf";
