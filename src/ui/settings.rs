use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::ui::*;

fn settings_ui_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_controller: Res<GameController>,
) {
    let button_color: UiColor = Color::NONE.into();
    let button_style: Style = Style {
        size: Size::new(Val::Auto, Val::Percent(100.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect {
            left: Val::Px(10.0),
            right: Val::Px(10.0),
            ..Default::default()
        },
        ..Default::default()
    };

    // settings ui
    let window = UiWindow::new(); //.with_bg_alpha(1.0);
    window.spawn_with_children(
        &mut commands,
        |parent| {
            // settings title
            SectionHeader::from_title(
                parent,
                "Settings",
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            );

            // audio settings section
            SectionHeader::from_title(
                parent,
                "Audio",
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            );

            // music volume setting
            SettingsElement::create(
                parent,
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
                "Music volume",
                &[
                    (SettingsButtonType::MusicVolumeMinus, "-"),
                    (SettingsButtonType::MusicVolumePlus, "+"),
                ],
                Some(SettingValueType::MusicVolume),
            );

            // sound effect volume setting
            SettingsElement::create(
                parent,
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
                "Sound effects volume",
                &[
                    (SettingsButtonType::EffectsVolumeMinus, "-"),
                    (SettingsButtonType::EffectsVolumePlus, "+"),
                ],
                Some(SettingValueType::EffectsVolume),
            );

            // debug settings section
            SectionHeader::from_title(
                parent,
                "Debug",
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            );

            // show fps setting
            SettingsElement::create(
                parent,
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
                "Show FPS",
                &[(
                    SettingsButtonType::FPSShow,
                    match game_controller.settings.show_fps {
                        true => "On",
                        false => "Off",
                    },
                )],
                None,
            );

            // reset highscore setting
            SettingsElement::create(
                parent,
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
                "Reset Highscore",
                &[(SettingsButtonType::Reset, "Reset")],
                None,
            );

            // close settings button
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Auto),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            top: Val::Px(0.0),
                            right: Val::Px(0.0),
                            ..Default::default()
                        },
                        ..button_style.clone()
                    },
                    color: button_color,
                    ..Default::default()
                })
                .with_children(|close_button| {
                    close_button
                        .spawn_bundle(TextBundle {
                            text: Text::from_section(
                                "x",
                                TextStyle {
                                    font: asset_server.load(FONT_PATH),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            ),
                            ..Default::default()
                        })
                        .insert(UiZ(41.0));
                })
                .insert(UiZ(40.0))
                .insert(SettingsButton {
                    just_clicked: true,
                    button_type: SettingsButtonType::Close,
                });
        },
        SettingsUI,
    );
}

#[derive(Component)]
struct SettingsUI;

fn settings_ui_system(
    mut settings_visibility_query: Query<&mut Visibility, With<SettingsUI>>,

    mut settings_buttons_query: Query<(&Interaction, &Children, &mut SettingsButton)>,
    mut text_query: Query<&mut Text, Without<SettingValueText>>, // todo: change this
    mut value_query: Query<(&mut Text, &SettingValueText)>,

    (mut game_controller, keyboard_input, mut pkv, audio): (
        ResMut<GameController>,
        Res<Input<KeyCode>>,
        ResMut<PkvStore>,
        Res<Audio>,
    ),
) {
    let mut settings_visibility = settings_visibility_query.single_mut();

    fn close_settings(
        mut settings_visibility: &mut Visibility,
        game_controller: &mut GameController,
    ) {
        settings_visibility.is_visible = !settings_visibility.is_visible;

        if settings_visibility.is_visible {
            game_controller.pause_game();
        } else {
            game_controller.resume_game();
        }
    }

    if keyboard_input.just_pressed(KeyCode::Escape)
        && game_controller.game_state != GameState::Finished
    {
        close_settings(&mut settings_visibility, &mut game_controller);
    }

    let mut changed = false;

    for (interaction, children, mut button) in settings_buttons_query.iter_mut() {
        if interaction == &Interaction::Clicked && button.just_clicked {
            button.just_clicked = false;
            changed = true;

            match button.button_type {
                SettingsButtonType::MusicVolumeMinus => {
                    game_controller.settings.music_vol_level -= 0.05;

                    if game_controller.settings.music_vol_level < 0.0 {
                        game_controller.settings.music_vol_level = 0.0;
                    }
                }
                SettingsButtonType::MusicVolumePlus => {
                    game_controller.settings.music_vol_level += 0.05;

                    if game_controller.settings.music_vol_level > 1.0 {
                        game_controller.settings.music_vol_level = 1.0;
                    }
                }
                SettingsButtonType::EffectsVolumeMinus => {
                    game_controller.settings.effects_vol_level -= 0.05;

                    if game_controller.settings.effects_vol_level < 0.0 {
                        game_controller.settings.effects_vol_level = 0.0;
                    }
                }
                SettingsButtonType::EffectsVolumePlus => {
                    game_controller.settings.effects_vol_level += 0.05;

                    if game_controller.settings.effects_vol_level > 1.0 {
                        game_controller.settings.effects_vol_level = 1.0;
                    }
                }
                SettingsButtonType::FPSShow => {
                    game_controller.settings.show_fps = !game_controller.settings.show_fps;

                    // change button text
                    text_query.get_mut(children[0]).unwrap().sections[0].value =
                        match game_controller.settings.show_fps {
                            true => "On".to_string(),
                            false => "Off".to_string(),
                        }
                }
                SettingsButtonType::Close => {
                    close_settings(&mut settings_visibility, &mut game_controller);
                }
                SettingsButtonType::Reset => {
                    game_controller.player_stats.high_score = 0;
                    game_controller.save_player_stats(&mut pkv)
                }
            }
        } else if interaction != &Interaction::Clicked {
            button.just_clicked = true;
        }
    }

    for (mut text, setting_value_text) in value_query.iter_mut() {
        match setting_value_text.value_type {
            SettingValueType::MusicVolume => {
                text.sections[0].value =
                    format!("{:.0}%", game_controller.settings.music_vol_level * 100.0);
            }
            SettingValueType::EffectsVolume => {
                text.sections[0].value =
                    format!("{:.0}%", game_controller.settings.effects_vol_level * 100.0);
            }
        }
    }

    if changed {
        audio.set_volume(game_controller.settings.music_vol_level);

        // update settings in pkv
        pkv.set(GAME_SETTINGS_KEY, &game_controller.settings)
            .expect("Failed to save game settings");
    }
}

struct SettingsElement;

impl SettingsElement {
    fn create(
        parent: &mut ChildBuilder,
        text_style: TextStyle,
        title: &str,

        buttons: &[(SettingsButtonType, &str)],
        value_type_: Option<SettingValueType>,
    ) {
        // style
        let button_color: UiColor = Color::NONE.into();
        let button_style: Style = Style {
            size: Size::new(Val::Auto, Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect {
                left: Val::Px(10.0),
                right: Val::Px(10.0),
                ..Default::default()
            },
            ..Default::default()
        };

        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Auto),
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Row,
                    margin: UiRect {
                        bottom: Val::Percent(2.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            })
            .with_children(|setting_parent| {
                setting_parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            justify_content: JustifyContent::FlexStart,
                            margin: UiRect {
                                right: Val::Auto,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        color: Color::NONE.into(),
                        ..Default::default()
                    })
                    .with_children(|setting_title| {
                        setting_title
                            .spawn_bundle(TextBundle {
                                text: Text::from_section(title.to_string(), text_style.clone()),
                                ..Default::default()
                            })
                            .insert(UiZ(33.0));
                    });

                // setting value
                if let Some(value_type) = value_type_ {
                    setting_parent
                        .spawn_bundle(TextBundle {
                            text: Text::from_section("", text_style.clone()),
                            ..Default::default()
                        })
                        .insert(UiZ(32.0))
                        .insert(SettingValueText { value_type });
                }

                // setting buttons
                for (button_type, button_text) in buttons {
                    setting_parent
                        .spawn_bundle(ButtonBundle {
                            style: button_style.clone(),
                            color: button_color,
                            ..Default::default()
                        })
                        .with_children(|setting_button| {
                            setting_button
                                .spawn_bundle(TextBundle {
                                    text: Text::from_section(
                                        button_text.to_string(),
                                        text_style.clone(),
                                    ),
                                    ..Default::default()
                                })
                                .insert(UiZ(34.0));
                        })
                        .insert(UiZ(33.0))
                        .insert(SettingsButton {
                            just_clicked: true,
                            button_type: *button_type,
                        });
                }
            })
            .insert(UiZ(31.0));
    }
}

#[derive(Component)]
struct SettingsButton {
    just_clicked: bool,
    button_type: SettingsButtonType,
}

#[derive(Clone, Copy)]
enum SettingsButtonType {
    MusicVolumeMinus,
    MusicVolumePlus,
    EffectsVolumeMinus,
    EffectsVolumePlus,
    FPSShow,
    Close,
    Reset,
}

#[derive(Component)]
struct SettingValueText {
    value_type: SettingValueType,
}

enum SettingValueType {
    MusicVolume,
    EffectsVolume,
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(settings_ui_setup)
            .add_system(settings_ui_system);
    }
}
