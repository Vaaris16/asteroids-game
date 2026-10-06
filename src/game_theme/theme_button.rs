use bevy::prelude::*;

use crate::{
    core::game_assets::game_assets::GameAssets,
    game_theme::{
        game_theme::GameTheme,
        theme_markers::{ThemeBackground, ThemeBorderColor},
    },
};

pub fn spawn_theme_button(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexEnd,
            ..Default::default()
        },
        children![(theme_button())],
    ));
}

#[derive(Component)]
pub struct ThemeButton;

#[derive(Component)]
pub struct ThemeButtonImage;

pub fn theme_button() -> impl Bundle {
    (
        Button,
        ThemeButton,
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(px(1)),
            position_type: PositionType::Absolute,
            top: px(20),
            left: px(20),
            padding: UiRect::all(px(10)),
            ..Default::default()
        },
        BorderColor::all(Color::WHITE),
        ThemeBorderColor,
        BackgroundColor(Color::BLACK),
        ThemeBackground,
        children![(
            ThemeButtonImage,
            ImageNode::default(),
            Node {
                width: px(45),
                height: px(45),
                ..Default::default()
            },
        )],
    )
}

pub fn theme_button_interactions(
    mut game_theme: ResMut<GameTheme>,
    button: Single<&Interaction, (With<ThemeButton>, Changed<Interaction>)>,
) {
    match *button {
        Interaction::Pressed => {
            *game_theme = game_theme.toggle_game_theme();
        }
        _ => (),
    }
}

pub fn set_theme_icon(
    mut button_theme_icon: Single<&mut ImageNode, With<ThemeButtonImage>>,
    game_theme: Res<GameTheme>,
    game_assets: Res<GameAssets>,
) {
    button_theme_icon.image = game_assets.get_theme_icon(&game_theme);
}

pub fn cleanup_theme_button(
    mut commands: Commands,
    theme_button: Single<Entity, With<ThemeButton>>,
) {
    commands.entity(*theme_button).despawn();
}
