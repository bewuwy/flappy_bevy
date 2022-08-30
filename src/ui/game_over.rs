use bevy::prelude::*;

use crate::ui::*;

fn game_over_ui_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // game_controller: Res<GameController>,
) {
    let text_style = TextStyle {
        font: asset_server.load(FONT_PATH),
        font_size: 40.0,
        color: Color::WHITE,
    };
    let button_style: Style = Style {
        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect {
            left: Val::Px(10.0),
            right: Val::Px(10.0),
            ..Default::default()
        },
        margin: UiRect {
            top: Val::Auto,
            ..Default::default()
        },
        ..Default::default()
    };

    let window = UiWindow::new();
    window.with_width_percent(0.4f32).spawn_with_children(
        &mut commands,
        |parent| {
            SectionHeader::from_title(
                parent,
                "Game Over",
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            );

            parent
                .spawn_bundle(TextBundle {
                    text: Text::from_section("", text_style.clone()),
                    style: Style {
                        margin: UiRect {
                            top: Val::Percent(7.0),
                            bottom: Val::Percent(2.0),
                            ..Default::default()
                        },
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(UiZ(32.0))
                .insert(WindowValueText {
                    text_type: WindowValueType::Score,
                });

            parent
                .spawn_bundle(TextBundle {
                    text: Text::from_section("", text_style.clone()),
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(UiZ(32.0))
                .insert(WindowValueText {
                    text_type: WindowValueType::HighScore,
                });

            let retry_button = UiButton::new("retry_button");
            retry_button.spawn_from_text(
                parent,
                "Retry",
                TextStyle {
                    font_size: 50.0,
                    ..text_style
                },
                button_style,
                Color::NONE,
            );
        },
        GameOverUi,
    );
}

fn game_over_ui_system(
    mut visibility_query: Query<&mut Visibility, With<GameOverUi>>,
    mut text_query: Query<(&mut Text, &WindowValueText)>,
    mut ui_button_query: Query<(&mut UiButton, &Interaction)>,
    player_transform_query: Query<(&Transform, &Player)>,

    mut game_controller: ResMut<GameController>,
) {
    let mut visibility = visibility_query.single_mut();

    let (player_transform, _) = player_transform_query.single();

    if game_controller.is_game_finished(player_transform) {
        visibility.is_visible = true;

        for (mut text, window_value) in text_query.iter_mut() {
            match window_value.text_type {
                WindowValueType::Score => {
                    text.sections[0].value = format!("Score: {}", game_controller.score);
                }
                WindowValueType::HighScore => {
                    text.sections[0].value =
                        format!("High Score: {}", game_controller.player_stats.high_score);
                }
            }
        }

        // ui buttons
        for (mut button, interaction) in ui_button_query.iter_mut() {
            if interaction == &Interaction::Clicked && button.just_clicked {
                button.just_clicked = false;

                match button.button_id.as_str() {
                    "retry_button" => {
                        game_controller.game_state = GameState::Restart;
                        println!("Restarting game");
                    }

                    _ => {
                        println!("Unknown button: {}", button.button_id);
                    }
                }
            } else if interaction != &Interaction::Clicked {
                button.just_clicked = true;
            }
        }
    } else {
        visibility.is_visible = false;
    }
}

#[derive(Component)]
pub struct GameOverUi;

#[derive(Component)]
struct WindowValueText {
    text_type: WindowValueType,
}

enum WindowValueType {
    Score,
    HighScore,
}

pub struct GameOverUiPlugin;

impl Plugin for GameOverUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(game_over_ui_setup)
            .add_system(game_over_ui_system);
    }
}
