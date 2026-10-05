use bevy::prelude::*;

use crate::game_theme::{
    game_theme::GameTheme,
    theme_button::{set_theme_icon, spawn_theme_button, theme_button_interactions},
    theme_colors::{ThemeColors, apply_theme_to_new_ui, set_changed_theme},
};

pub struct GameThemePlugin;

impl Plugin for GameThemePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_theme_button)
            .add_systems(
                Update,
                (
                    theme_button_interactions,
                    set_changed_theme,
                    apply_theme_to_new_ui,
                    set_theme_icon,
                ),
            )
            .insert_resource(ThemeColors::theme_colors(&GameTheme::default()))
            .init_resource::<GameTheme>();
    }
}
