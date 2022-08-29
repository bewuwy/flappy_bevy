use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::*;

static PRESS_START_TEXT: &str = "Press space to start";

fn ui_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_controller: Res<GameController>,
) {
    const FONT_PATH: &str = "fonts/font.ttf";

    let button_color: UiColor = Color::NONE.into();

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
    const AUDIO_SECTION: &str = "Audio";
    const VOLUME_TITLE: &str = "Music volume";
    const DEBUG_SECTION: &str = "Debug";

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
                .insert(UiText {
                    text_type: UiTextType::FPSText,
                })
                .insert(UiZ(20.0));

            // Start text
            parent
                .spawn_bundle(TextBundle::from_section(
                    PRESS_START_TEXT,
                    TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 50.0,
                        color: Color::BLACK,
                    },
                ))
                .insert(UiText {
                    text_type: UiTextType::StartMessage,
                })
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
                .insert(UiText {
                    text_type: UiTextType::Score,
                })
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
            color: Color::rgba(0.0, 0.0, 0.0, 0.97).into(),
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

            // audio settings section
            SectionHeader::from_title(
                parent,
                AUDIO_SECTION,
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            );

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
                            color: button_color,
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
                                .insert(UiZ(34.0));
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
                            color: button_color,
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
                                .insert(UiZ(34.0));
                        })
                        .insert(UiZ(33.0))
                        .insert(SettingsButton {
                            just_clicked: true,
                            button_type: SettingsButtonType::VolumePlus,
                        });
                })
                .insert(UiZ(31.0));

            // debug settings section
            SectionHeader::from_title(
                parent,
                DEBUG_SECTION,
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            );

            // show fps setting
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Auto),
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Row,
                        margin: UiRect {
                            // bottom: Val::Percent(2.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|fps_show_parent| {
                    // fps show title
                    fps_show_parent
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
                        .with_children(|fps_show_title| {
                            fps_show_title
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
                    fps_show_parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Percent(100.0)),
                                ..button_style.clone()
                            },
                            color: button_color,
                            ..Default::default()
                        })
                        .with_children(|fps_button| {
                            fps_button
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
                                .insert(UiZ(34.0));
                        })
                        .insert(UiZ(33.0))
                        .insert(SettingsButton {
                            just_clicked: true,
                            button_type: SettingsButtonType::FPSShow,
                        });
                })
                .insert(UiZ(31.0));

            // reset highscore setting
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Auto),
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Row,
                        margin: UiRect {
                            // bottom: Val::Percent(2.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|reset_parent| {
                    // reset title
                    reset_parent
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
                        .with_children(|reset_title| {
                            reset_title
                                .spawn_bundle(TextBundle {
                                    text: Text::from_section(
                                        "Reset highscore",
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

                    // reset button
                    reset_parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Percent(100.0)),
                                ..button_style.clone()
                            },
                            color: button_color,
                            ..Default::default()
                        })
                        .with_children(|reset_button| {
                            reset_button
                                .spawn_bundle(TextBundle {
                                    text: Text::from_section(
                                        "Reset",
                                        TextStyle {
                                            font: asset_server.load(FONT_PATH),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..Default::default()
                                })
                                .insert(UiZ(34.0));
                        })
                        .insert(UiZ(33.0))
                        .insert(SettingsButton {
                            just_clicked: true,
                            button_type: SettingsButtonType::Reset,
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
        })
        .insert(SettingsUI)
        .insert(UiZ(30.0));
}

fn text_ui_system(
    mut query: Query<(&mut Text, &mut Visibility, &UiText)>,
    game_controller: Res<GameController>,
    diagnostics: Res<Diagnostics>,
) {
    const HIGH_SCORE_TEXT: &str = "High Score";

    for (mut text, mut visibility, ui_text) in query.iter_mut() {
        match ui_text.text_type {
            UiTextType::StartMessage => {
                if game_controller.started {
                    visibility.is_visible = false;
                } else {
                    visibility.is_visible = true;
                }
            }
            UiTextType::Score => {
                if game_controller.started {
                    text.sections[0].value = game_controller.score.to_string();
                } else {
                    text.sections[0].value = format!(
                        "{}: {}",
                        HIGH_SCORE_TEXT, game_controller.player_stats.high_score
                    );
                }
            }
            UiTextType::FPSText => {
                if game_controller.settings.show_fps {
                    visibility.is_visible = true;

                    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                        if let Some(average) = fps.average() {
                            // Update the value of the second section
                            text.sections[1].value = format!("{average:.2}");
                        }
                    }
                } else {
                    visibility.is_visible = false;
                }
            }
        }
    }
}

#[derive(Component)]
struct UiText {
    text_type: UiTextType,
}

enum UiTextType {
    StartMessage,
    Score,
    FPSText,
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

        if settings_visibility.is_visible {
            game_controller.paused = true;
        } else {
            game_controller.paused = false;
        }
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
                SettingsButtonType::Reset => {
                    game_controller.player_stats.high_score = 0;
                    game_controller.save_player_stats(&mut pkv)
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
    Reset,
}

#[derive(Component)]
struct VolumeValueText;

struct SectionHeader; // TODO: change to function

impl SectionHeader {
    pub fn from_title(parent: &mut ChildBuilder, title: &str, style: TextStyle) -> Self {
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Auto),
                    // margin: Rect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            })
            .with_children(|header_parent| {
                header_parent
                    .spawn_bundle(TextBundle {
                        text: Text::from_section(title, style),
                        ..Default::default()
                    })
                    .insert(UiZ(31.0));
            });

        Self
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UiZPlugin)
            .add_startup_system(ui_setup)
            .add_system(text_ui_system)
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
