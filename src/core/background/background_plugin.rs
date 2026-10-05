use bevy::{prelude::*, window::PrimaryWindow};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_default_bg)
            .add_systems(Update, resize_background);
    }
}

#[derive(Component)]
pub struct Background;

fn set_default_bg(window: Single<&Window, With<PrimaryWindow>>, mut commands: Commands) {
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(window.width(), window.height())),
            ..Default::default()
        },
        Background,
    ));
}

fn resize_background(
    mut background: Single<&mut Sprite, With<Background>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    background.custom_size = Some(Vec2::new(window.width(), window.height()));
}
