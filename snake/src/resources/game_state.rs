use super::score::Score;

use crate::common::*;
use bevy::prelude::*;

/// The game state defines the current status of the application.
///
/// This is handful in case we need to spawn or despawn some entities,
/// or load resources before others.
#[derive(Default, Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum GameState {
    /// The initial state : the app is started and a user choice is awaited.
    #[default]
    Initialized,
    /// The ready state : the border set is defined, user input waited.
    Ready,
    /// The running state: the user is playing.
    Running,
    /// The paused state:  the user has paused the system, and his input
    /// is waited in order to resume.
    Paused,
    /// Game over ! User lost, his input is waited in order to
    /// either restart with the same border set or another one.
    Over,
}

impl GameState {
    /// Returns whether borders are visible or not.
    ///
    /// Useful when we need to show text, as borders can overflow the text.
    pub fn are_borders_visible(&self) -> bool {
        matches!(self, Self::Running | Self::Ready)
    }

    /// Returns the text that has to be displayed to the user.
    fn get_score_text_value(&self, score: &str) -> String {
        match self {
            GameState::Running => format!("Score : {}", score),
            GameState::Over => format!("Game over\nYour score : {}\nPress 'R' to restart.\nPress 'ESC' to choose another border set.", score),
            GameState::Paused => "The game has been paused\nPress 'P' to resume.".into(),
            GameState::Ready => String::default(),
            GameState::Initialized => "Choose a border set".into(),
        }
    }

    /// Returns the style of the text displayed to the user.
    fn get_score_text_style(&self, font: Handle<Font>) -> TextStyle {
        match &self {
            GameState::Running => TextStyle {
                font_size: 16f32,
                color: Color::WHITE,
                font,
            },
            GameState::Over | GameState::Paused | GameState::Initialized => TextStyle {
                font_size: 30f32,
                color: Color::WHITE,
                font,
            },
            GameState::Ready => TextStyle::default(),
        }
    }

    /// Returns the style of the score.
    pub fn get_score_style(&self) -> Style {
        match &self {
            GameState::Running => Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(10f32),
                    right: Val::Px(10f32),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                overflow: Overflow::Hidden,
                ..default()
            },
            GameState::Paused | GameState::Over => Style {
                margin: UiRect::all(Val::Auto),
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            GameState::Initialized => Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(MAX_SCREEN_HEIGHT / 3f32),
                    left: Val::Px(MAX_SCREEN_WIDTH / 2f32),
                    ..default()
                },
                ..default()
            },
            GameState::Ready => Style {
                display: Display::None,
                ..default()
            },
        }
    }

    /// Returns the text of the score.
    pub fn get_score_text(&self, score: Score, font: Handle<Font>) -> Text {
        let text_style: TextStyle = self.get_score_text_style(font);
        Text::from_section(self.get_score_text_value(&score.to_string()), text_style)
    }
}
