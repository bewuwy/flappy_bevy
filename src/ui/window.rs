use bevy::prelude::*;

use crate::ui::*;

pub struct UiWindow {
    width_percent: Option<f32>,
    bg_alpha: Option<f32>
}

impl UiWindow {
    pub fn new() -> Self {
        Self {
            width_percent: None,
            bg_alpha: None
        }
    }

    pub fn with_width_percent(mut self, width_percent: f32) -> Self {
        self.width_percent = Some(width_percent);
        self
    }

    // pub fn with_bg_alpha(mut self, bg_alpha: f32) -> Self {
    //     self.bg_alpha = Some(bg_alpha);
    //     self
    // }

    pub fn spawn_with_children<T: bevy::prelude::Component>(
        &self,
        commands: &mut Commands,
        spawn_children: impl FnOnce(&mut ChildBuilder),
        window_struct: T,
    ) {
        let window_width = self.width_percent.unwrap_or(0.8) * 100.0;
        let bg_alpha = self.bg_alpha.unwrap_or(0.97);

        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(window_width), Val::Percent(90.0)),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    flex_direction: FlexDirection::ColumnReverse,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Percent(5.0),
                        left: Val::Percent((100.0 - window_width) / 2.0),
                        ..Default::default()
                    },
                    padding: UiRect {
                        left: Val::Px(10.0),
                        right: Val::Px(10.0),
                        bottom: Val::Px(10.0),
                        top: Val::Percent(2.0),
                        // ..Default::default()
                    },
                    ..Default::default()
                },
                visibility: Visibility { is_visible: false },
                color: Color::rgba(0.0, 0.0, 0.0, bg_alpha).into(),
                ..Default::default()
            })
            .with_children(spawn_children)
            .insert(window_struct)
            .insert(UiZ(30.0));
    }
}

pub struct SectionHeader;

impl SectionHeader {
    pub fn from_title(parent: &mut ChildBuilder, title: &str, style: TextStyle) {
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
    }
}

#[derive(Component, Clone)]
pub struct UiButton {
    pub just_clicked: bool,
    pub button_id: String,
}

impl UiButton {
    pub fn new(button_id: &str) -> Self {
        Self {
            just_clicked: true,
            button_id: button_id.to_string(),
        }
    }

    pub fn spawn_from_text(
        &self,
        parent: &mut ChildBuilder,
        text: &str,
        text_style: TextStyle,
        button_style: Style,
        button_color: Color,
    ) {
        parent
            .spawn_bundle(ButtonBundle {
                style: button_style.clone(),
                color: button_color.into(),
                ..Default::default()
            })
            .with_children(|setting_button| {
                setting_button
                    .spawn_bundle(TextBundle {
                        text: Text::from_section(text.to_string(), text_style.clone()),
                        ..Default::default()
                    })
                    .insert(UiZ(34.0));
            })
            .insert(UiZ(33.0))
            .insert(self.clone());
    }
}
