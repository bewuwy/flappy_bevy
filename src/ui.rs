use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::*;

fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    const FONT_PATH: &str = "fonts/font.ttf";

    // style
    let button_style: Style = Style {
        size: Size::new(Val::Auto, Val::Percent(100.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect {
            left: Val::Px(20.0),
            right: Val::Px(20.0),
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
            // title
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
                            color: Color::RED.into(),
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
                        .insert(VolumeMinusButton);

                    // volume plus button
                    vol_parent
                        .spawn_bundle(ButtonBundle {
                            style: button_style.clone(),
                            color: Color::GREEN.into(),
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
                        .insert(VolumePlusButton);
                })
                .insert(UiZ(31.0));
        })
        .insert(SettingsUI)
        .insert(UiZ(30.0));
}

#[derive(Component)]
struct FPSText;

fn fps_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FPSText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{average:.2}");
            }
        }
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
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Visibility, With<SettingsUI>>,
    vol_minus_query: Query<&Interaction, With<VolumeMinusButton>>,
    vol_plus_query: Query<&Interaction, With<VolumePlusButton>>,
    mut vol_value_query: Query<&mut Text, With<VolumeValueText>>,
    audio: ResMut<Audio>,
    mut game_controller: ResMut<GameController>,
    mut pkv: ResMut<PkvStore>,
) {
    let mut settings_visibility = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Escape) {
        settings_visibility.is_visible = !settings_visibility.is_visible;
    }

    // volume setting system
    let vol_minus_interaction = vol_minus_query.single();
    let vol_plus_interaction = vol_plus_query.single();
    let mut vol_value_text = vol_value_query.single_mut();
    let mut vol_changed = false;

    if vol_minus_interaction == &Interaction::Clicked {
        game_controller.settings.vol_level -= 0.01;

        if game_controller.settings.vol_level < 0.0 {
            game_controller.settings.vol_level = 0.0;
        }
        vol_changed = true;
    }

    if vol_plus_interaction == &Interaction::Clicked {
        game_controller.settings.vol_level += 0.01;

        if game_controller.settings.vol_level > 1.0 {
            game_controller.settings.vol_level = 1.0;
        }
        vol_changed = true;
    }

    vol_value_text.sections[0].value =
        format!("{:.0}%", game_controller.settings.vol_level * 100.0);

    if vol_changed {
        audio.set_volume(game_controller.settings.vol_level as f64);

        // update settings in pkv
        pkv.set(GAME_SETTINGS_KEY, &game_controller.settings)
            .expect("Failed to save game settings");
    }
}

#[derive(Component)]
struct VolumeMinusButton;

#[derive(Component)]
struct VolumePlusButton;

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

fn update_uiz(mut query: Query<(&UiZ, &mut GlobalTransform), (With<Node>, Changed<Transform>)>) {
    for (uiz, mut transform) in query.iter_mut() {
        let translation = transform.translation_mut();
        translation.z = uiz.0;
    }
}
