use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::*;

fn ui_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_controller: Res<GameController>,
) {
    const FONT_PATH: &str = "fonts/font.ttf";

    // style
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

    // texts
    const VOLUME_TITLE: &str = "Music volume";

    // text ui
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // FPS text
            parent
                .spawn_bundle(
                    TextBundle::from_sections([
                        TextSection::new(
                            "FPS: ",
                            TextStyle {
                                font: asset_server.load(FONT_PATH),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        TextSection::from_style(TextStyle {
                            font: asset_server.load(FONT_PATH),
                            font_size: 30.0,
                            color: Color::GOLD,
                        }),
                    ])
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            top: Val::Percent(0.0),
                            left: Val::Percent(0.0),
                            ..Default::default()
                        },
                        ..default()
                    }),
                )
                .insert(FPSText)
                .insert(UiZ(20.0));

            // Start text
            parent
                .spawn_bundle(
                    TextBundle::from_sections([TextSection::from_style(TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 50.0,
                        color: Color::BLACK,
                    })])
                    .with_style(Style { ..default() }),
                )
                .insert(StartText)
                .insert(UiZ(20.0));

            // Score text
            parent
                .spawn_bundle(
                    TextBundle::from_sections([TextSection::from_style(TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 80.0,
                        color: Color::BLACK,
                    })])
                    .with_style(Style {
                        margin: UiRect {
                            top: Val::Percent(10.0),
                            bottom: Val::Percent(15.0),
                            ..Default::default()
                        },
                        ..default()
                    }),
                )
                .insert(ScoreText)
                .insert(UiZ(20.0));
        });

    // settings ui
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(80.0), Val::Percent(90.0)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::ColumnReverse,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Percent(5.0),
                    left: Val::Percent(10.0),
                    ..Default::default()
                },
                padding: UiRect {
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            visibility: Visibility { is_visible: false },
            color: Color::rgba(0.0, 0.0, 0.0, 0.9).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // settings title
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect {
                            top: Val::Percent(2.0),
                            bottom: Val::Percent(2.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|title_parent| {
                    title_parent
                        .spawn_bundle(TextBundle {
                            text: Text::from_section(
                                "Settings",
                                TextStyle {
                                    font: asset_server.load(FONT_PATH),
                                    font_size: 50.0,
                                    color: Color::WHITE,
                                },
                            ),
                            ..Default::default()
                        })
                        .insert(UiZ(31.0));
                });

            // volume setting
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
                .with_children(|vol_parent| {
                    // volume title
                    vol_parent
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
                        .with_children(|vol_title| {
                            vol_title
                                .spawn_bundle(TextBundle {
                                    text: Text::from_section(
                                        VOLUME_TITLE,
                                        TextStyle {
                                            font: asset_server.load(FONT_PATH),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..Default::default()
                                })
                                .insert(UiZ(33.0));
                        });

                    // volume level text
                    vol_parent
                        .spawn_bundle(TextBundle {
                            text: Text::from_section(
                                "50%",
                                TextStyle {
                                    font: asset_server.load(FONT_PATH),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            ),
                            ..Default::default()
                        })
                        .insert(UiZ(32.0))
                        .insert(VolumeValueText);

                    // volume minus button
                    vol_parent
                        .spawn_bundle(ButtonBundle {
                            style: button_style.clone(),
                            color: Color::NONE.into(),
                            // material: asset_server.load("textures/minus.png").into(),
                            ..Default::default()
                        })
                        .with_children(|min_button| {
                            min_button
                                .spawn_bundle(TextBundle {
                                    text: Text::from_section(
                                        "-",
                                        TextStyle {
                                            font: asset_server.load(FONT_PATH),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..Default::default()
                                })
                                .insert(UiZ(33.0));
                        })
                        .insert(UiZ(33.0))
                        .insert(SettingsButton {
                            just_clicked: true,
                            button_type: SettingsButtonType::VolumeMinus,
                        });

                    // volume plus button
                    vol_parent
                        .spawn_bundle(ButtonBundle {
                            style: button_style.clone(),
                            color: Color::NONE.into(),
                            // material: asset_server.load("textures/plus.png").into(),
                            ..Default::default()
                        })
                        .with_children(|plus_button| {
                            plus_button
                                .spawn_bundle(TextBundle {
                                    text: Text::from_section(
                                        "+",
                                        TextStyle {
                                            font: asset_server.load(FONT_PATH),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..Default::default()
                                })
                                .insert(UiZ(33.0));
                        })
                        .insert(UiZ(33.0))
                        .insert(SettingsButton {
                            just_clicked: true,
                            button_type: SettingsButtonType::VolumePlus,
                        });
                })
                .insert(UiZ(31.0));

            // show fps setting
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
                .with_children(|vol_parent| {
                    // fps show title
                    vol_parent
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
                        .with_children(|vol_title| {
                            vol_title
                                .spawn_bundle(TextBundle {
                                    text: Text::from_section(
                                        "Show FPS",
                                        TextStyle {
                                            font: asset_server.load(FONT_PATH),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..Default::default()
                                })
                                .insert(UiZ(33.0));
                        });

                    // fps show toggle button
                    vol_parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(100.0), Val::Percent(100.0)),
                                ..button_style.clone()
                            },
                            color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                            ..Default::default()
                        })
                        .with_children(|plus_button| {
                            plus_button
                                .spawn_bundle(TextBundle {
                                    text: Text::from_section(
                                        match game_controller.settings.show_fps {
                                            true => "On".to_string(),
                                            false => "Off".to_string(),
                                        },
                                        TextStyle {
                                            font: asset_server.load(FONT_PATH),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..Default::default()
                                })
                                .insert(UiZ(33.0));
                        })
                        .insert(UiZ(33.0))
                        .insert(SettingsButton {
                            just_clicked: true,
                            button_type: SettingsButtonType::FPSShow,
                        });
                })
                .insert(UiZ(31.0));

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
                    color: Color::NONE.into(),
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
        })
        .insert(SettingsUI)
        .insert(UiZ(30.0));
}

#[derive(Component)]
struct FPSText;

fn fps_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<(&mut Text, &mut Visibility), With<FPSText>>,
    game_controller: Res<GameController>,
) {
    let (mut fps_text, mut fps_visibility) = query.single_mut();

    if game_controller.settings.show_fps {
        fps_visibility.is_visible = true;

        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                fps_text.sections[1].value = format!("{average:.2}");
            }
        }
    } else {
        fps_visibility.is_visible = false;
    }
}

#[derive(Component)]
struct ScoreText;

fn score_text_system(
    mut query: Query<(&mut Text, With<ScoreText>)>,
    game_controller: Res<GameController>,
) {
    let (mut score_text, _) = query.single_mut();

    if game_controller.started {
        score_text.sections[0].value = game_controller.score.to_string();
    } else {
        score_text.sections[0].value =
            format!("High score: {}", game_controller.player_stats.high_score);
    }
}

#[derive(Component)]
struct StartText;

fn start_text_system(
    mut query: Query<(&mut Text, With<StartText>)>,
    game_controller: Res<GameController>,
) {
    let (mut start_text, _) = query.single_mut();

    if game_controller.started {
        start_text.sections[0].value = "".to_string();
    } else {
        start_text.sections[0].value = "Press space to start".to_string();
    }
}

#[derive(Component)]
struct SettingsUI; // TODO: pause game when settings are open

fn settings_ui_system(
    mut settings_visibility_query: Query<&mut Visibility, With<SettingsUI>>,
    mut settings_buttons_query: Query<(&Interaction, &Children, &mut SettingsButton)>,
    mut text_query: Query<&mut Text, Without<VolumeValueText>>, // todo: change this
    mut vol_value_query: Query<&mut Text, With<VolumeValueText>>,

    (mut game_controller, keyboard_input, mut pkv, audio): (
        ResMut<GameController>,
        Res<Input<KeyCode>>,
        ResMut<PkvStore>,
        Res<Audio>,
    ),
) {
    let mut settings_visibility = settings_visibility_query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Escape) {
        settings_visibility.is_visible = !settings_visibility.is_visible;
    }

    // volume setting system
    let mut vol_value_text = vol_value_query.single_mut();

    let mut changed = false;

    for (interaction, children, mut button) in settings_buttons_query.iter_mut() {
        if interaction == &Interaction::Clicked && button.just_clicked {
            button.just_clicked = false;
            changed = true;

            match button.button_type {
                SettingsButtonType::VolumeMinus => {
                    game_controller.settings.vol_level -= 0.05;

                    if game_controller.settings.vol_level < 0.0 {
                        game_controller.settings.vol_level = 0.0;
                    }
                }
                SettingsButtonType::VolumePlus => {
                    game_controller.settings.vol_level += 0.05;

                    if game_controller.settings.vol_level > 1.0 {
                        game_controller.settings.vol_level = 1.0;
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
                    settings_visibility.is_visible = !settings_visibility.is_visible;
                }
            }
        } else if interaction != &Interaction::Clicked {
            button.just_clicked = true;
        }
    }

    vol_value_text.sections[0].value =
        format!("{:.0}%", game_controller.settings.vol_level * 100.0);

    if changed {
        audio.set_volume(game_controller.settings.vol_level);

        // update settings in pkv
        pkv.set(GAME_SETTINGS_KEY, &game_controller.settings)
            .expect("Failed to save game settings");
    }
}

#[derive(Component)]
struct SettingsButton {
    just_clicked: bool,
    button_type: SettingsButtonType,
}

enum SettingsButtonType {
    VolumeMinus,
    VolumePlus,
    FPSShow,
    Close,
}

#[derive(Component)]
struct VolumeValueText;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UiZPlugin)
            .add_startup_system(ui_setup)
            .add_system(fps_system)
            .add_system(score_text_system)
            .add_system(start_text_system)
            .add_system(settings_ui_system);
    }
}

pub struct UiZPlugin; // TODO: fix bevy ui
#[derive(Component)]
pub struct UiZ(pub f32);

impl Plugin for UiZPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            update_uiz.after(bevy::transform::TransformSystem::TransformPropagate),
        );
    }
}

#[allow(clippy::type_complexity)]
fn update_uiz(mut query: Query<(&UiZ, &mut GlobalTransform), (With<Node>, Changed<Transform>)>) {
    for (uiz, mut transform) in query.iter_mut() {
        let translation = transform.translation_mut();
        translation.z = uiz.0;
    }
}
