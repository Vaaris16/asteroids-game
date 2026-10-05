use bevy::prelude::*;

use crate::{
    core::{background::background_plugin::Background, game_assets::game_assets::GameAssets},
    game_theme::{
        game_theme::GameTheme,
        theme_markers::{
            ThemeBackground, ThemeBorderColor, ThemeButtonBackground, ThemePrimaryTextColor,
            ThemeSecondaryTextColor,
        },
    },
};

#[derive(Resource, Debug)]
pub struct ThemeColors {
    border_color: Color,

    pub primary_text_color: Color,
    pub secondary_text_color: Color,
    pub hovered_text_color: Color,

    pub hovered_bg_color: Color,
    pub button_bg_color: Color,

    pub background_color: Color,
}

impl ThemeColors {
    pub fn theme_colors(theme: &GameTheme) -> Self {
        let orange = Color::hsl(14., 0.71, 0.62);
        let off_white = Color::hsl(42., 0.77, 0.93);
        let dark_blue = Color::hsl(210., 0.5, 0.05);
        match theme {
            GameTheme::Retro => Self {
                border_color: Color::WHITE,

                primary_text_color: Color::WHITE,
                secondary_text_color: Color::WHITE,

                hovered_bg_color: Color::WHITE,
                hovered_text_color: Color::BLACK,

                button_bg_color: Color::BLACK,
                background_color: Color::BLACK,
            },
            GameTheme::Cosmic => Self {
                border_color: orange,

                primary_text_color: off_white,
                secondary_text_color: orange,

                hovered_bg_color: orange,

                button_bg_color: dark_blue,
                hovered_text_color: dark_blue,

                background_color: dark_blue,
            },
        }
    }
}

pub fn set_changed_theme(
    game_theme: Res<GameTheme>,
    mut border_colors: Query<
        &mut BorderColor,
        (
            With<ThemeBorderColor>,
            Without<ThemePrimaryTextColor>,
            Without<ThemeSecondaryTextColor>,
        ),
    >,
    mut primary_text_colors: Query<
        &mut TextColor,
        (
            With<ThemePrimaryTextColor>,
            Without<ThemeBorderColor>,
            Without<ThemeSecondaryTextColor>,
        ),
    >,
    mut secondary_text_colors: Query<
        &mut TextColor,
        (
            With<ThemeSecondaryTextColor>,
            Without<ThemeBorderColor>,
            Without<ThemePrimaryTextColor>,
        ),
    >,
    button_background: Query<&mut BackgroundColor, With<ThemeButtonBackground>>,
    mut theme_colors: ResMut<ThemeColors>,
    mut background: Single<&mut Sprite, With<Background>>,
    game_assets: Res<GameAssets>,
) {
    if game_theme.is_changed() {
        *theme_colors = ThemeColors::theme_colors(&game_theme);

        for mut border_color in &mut border_colors {
            *border_color = BorderColor::all(theme_colors.border_color);
        }

        for mut text_color in &mut primary_text_colors {
            text_color.0 = theme_colors.primary_text_color;
        }

        for mut text_color in &mut secondary_text_colors {
            text_color.0 = theme_colors.secondary_text_color;
        }

        for mut button_background in button_background {
            button_background.0 = theme_colors.button_bg_color;
        }

        background.image = game_assets.get_bg_image(&game_theme);
    }
}

pub fn apply_theme_to_new_ui(
    theme_colors: Res<ThemeColors>,

    mut border_colors: Query<&mut BorderColor, Added<ThemeBorderColor>>,

    mut text_colors: ParamSet<(
        Query<&mut TextColor, Added<ThemePrimaryTextColor>>,
        Query<&mut TextColor, Added<ThemeSecondaryTextColor>>,
    )>,
    theme_background: Query<&mut BackgroundColor, With<ThemeBackground>>,
) {
    for mut color in &mut border_colors {
        *color = BorderColor::all(theme_colors.border_color);
    }

    for mut color in &mut text_colors.p0() {
        color.0 = theme_colors.primary_text_color;
    }

    for mut color in &mut text_colors.p1() {
        color.0 = theme_colors.secondary_text_color;
    }
    for mut bg_theme in theme_background {
        bg_theme.0 = theme_colors.background_color;
    }
}
