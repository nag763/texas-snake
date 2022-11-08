use super::score::Score;

use crate::common::*;
use bevy::prelude::*;

#[derive(Default, Debug, Eq, PartialEq, Copy, Clone)]
pub enum GameState {
    #[default]
    Initialized,
    Ready,
    Running,
    Paused,
    Over,
}

impl GameState {
    pub fn are_borders_visible(&self) -> bool {
        matches!(self, Self::Running | Self::Ready)
    }

    fn get_score_text_value(&self, score: &str) -> String {
        match self {
            GameState::Running => format!("Score : {}", score),
            GameState::Over => format!("Game over\nYour score : {}\nPress 'R' to restart.", score),
            GameState::Paused => "The game has been paused\nPress 'P' to resume.".into(),
            GameState::Ready => String::default(),
            GameState::Initialized => "Choose a border set".into(),
        }
    }

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

    pub fn get_score_text(&self, score: Score, font: Handle<Font>) -> Text {
        let text_style: TextStyle = self.get_score_text_style(font);
        Text::from_section(self.get_score_text_value(&score.to_string()), text_style)
    }
}
