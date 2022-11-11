use bevy::prelude::*;
use crate::common::*;
use crate::resources::border_set::BorderSet;
use crate::resources::game_state::GameState;


/// Spawn the border picker buttons.
pub fn spawn_border_set_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    for border_set_variant in BorderSet::iterator() {
        commands
            .spawn()
            .insert_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    margin: UiRect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: NORMAL_BUTTON.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                    border_set_variant.to_string(),
                    TextStyle {
                        font: asset_server.load(FONT_ASSET_NAME),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ));
            })
            .insert(border_set_variant);
    }
}


/// The interactions with the button system.
pub fn border_set_choose_system(
    mut button_query: Query<(&Interaction, &mut UiColor, &BorderSet)>,
    mut game_state: ResMut<State<GameState>>,
    mut border_set: ResMut<Option<BorderSet>>,
) {
    for (interaction, mut color, button) in button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                // Each button corresponds to a border set, so we will
                // set the clicked border set as a resource
                *border_set = Some(*button);
                game_state.set(GameState::Ready).unwrap();
                break;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

/// Deletes the buttons, once they aren't useful anymore, and once a border set is picked.
pub fn delete_buttons(mut commands: Commands, mut button_query: Query<Entity, With<BorderSet>>) {
    for button_entity in button_query.iter_mut() {
        commands.entity(button_entity).despawn_recursive();
    }
}