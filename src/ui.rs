use bevy::prelude::*;

use crate::*;


pub fn ui_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(
                Val::Percent(100.0),
                Val::Percent(100.0),
            ),
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    })
    .with_children(|parent| {
        // FPS text
        parent.spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        font: asset_server.load("font.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("font.ttf"),
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
        .insert(FPSText);

        // Score text
        parent.spawn_bundle(
            TextBundle::from_sections([
                TextSection::from_style(TextStyle {
                    font: asset_server.load("font.ttf"),
                    font_size: 80.0,
                    color: Color::WHITE,
                }),
            ])
            .with_style(Style {
                align_self: AlignSelf::Center,
                margin: UiRect {
                    top: Val::Percent(-35.0),
                    left: Val::Percent(10.0),
                    ..Default::default()
                },
                ..default()
            }),
        )
        .insert(ScoreText); 
    });

}


#[derive(Component)]
pub struct FPSText;

pub fn fps_system(
    diagnostics: Res<Diagnostics>, 
    mut query: Query<&mut Text, With<FPSText>>
) {
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
pub struct ScoreText;

pub fn score_text_system(
    mut query: Query<(&mut Text, With<ScoreText>)>,
    game_controller_query: Query<&GameController>,
) {
    let (mut score_text, _) = query.single_mut();
    let game_controller = game_controller_query.single();

    score_text.sections[0].value = game_controller.score.to_string();
}


pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ui_setup);
        app.add_system(fps_system);
        app.add_system(score_text_system);
    }
}
