use bevy::prelude::*;
use rand::RngExt;

use crate::game_theme::game_theme::GameTheme;

#[derive(Resource, Clone)]
pub struct GameAssets {
    pub retro_spaceship_image: Handle<Image>,
    pub cosmic_spaceship_image: Handle<Image>,

    pub retro_bullet_image: Handle<Image>,
    pub cosmic_bullet_image: Handle<Image>,

    pub retro_asteroids: [Handle<Image>; 3],
    pub cosmic_asteroids: [Handle<Image>; 3],

    pub explosion_sound: Handle<AudioSource>,
    pub shoot_sound: Handle<AudioSource>,
    pub bg_sound: Handle<AudioSource>,
    pub life_decrement_sound: Handle<AudioSource>,

    pub retro_bg_image: Handle<Image>,
    pub cosmic_bg_image: Handle<Image>,

    pub retro_theme_icon: Handle<Image>,
    pub cosmic_theme_icon: Handle<Image>,
}

// Background Image.
const RETRO_BACKGROUND_IMAGE: &str = "backgrounds/retro_star_background.png";
const COSMIC_BACKGROUND_IMAGE: &str = "backgrounds/cosmic_star_background.png";
// Image path of the space ship.
const RETRO_SPACE_SHIP_IMAGE_PATH: &str = "spaceship/retro_space_ship.png";
const COSMIC_SPACE_SHIP_IMAGE_PATH: &str = "spaceship/cosmic_space_ship.png";
// Image path of the bullet.
const RETRO_BULLET_IMAGE_PATH: &str = "bullet/retro_bullet.png";
const COSMIC_BULLET_IMAGE_PATH: &str = "bullet/cosmic_bullet.png";
// Defines the image paths for the asteroids.
const RETRO_ASTEROID_PATH_1: &str = "asteroids_images/retro_asteroids/retro_asteroid_1.png";
const RETRO_ASTEROID_PATH_2: &str = "asteroids_images/retro_asteroids/retro_asteroid_2.png";
const RETRO_ASTEROID_PATH_3: &str = "asteroids_images/retro_asteroids/retro_asteroid_3.png";

const COSMIC_ASTEROID_PATH_1: &str = "asteroids_images/cosmic_asteroids/cosmic_asteroid_1.png";
const COSMIC_ASTEROID_PATH_2: &str = "asteroids_images/cosmic_asteroids/cosmic_asteroid_2.png";
const COSMIC_ASTEROID_PATH_3: &str = "asteroids_images/cosmic_asteroids/cosmic_asteroid_3.png";
// Sound effects.
const EXPLOSION_SOUND_EFFECT: &str = "sounds/explosion.wav";
const SHOOT_SOUND_EFFECT: &str = "sounds/shoot_sound.wav";
const BACKGROUND_MUSIC_PATH: &str = "sounds/background_music.wav";
const LOSE_LIFE_SOUND_PATH: &str = "sounds/lose_life_sound_effect.wav";
// Theme icons.
const RETRO_THEME_ICON: &str = "theme icons/retro_theme_icon.png";
const COSMIC_THEME_ICON: &str = "theme icons/cosmic_theme_icon.png";

impl FromWorld for GameAssets {
    fn from_world(world: &mut World) -> Self {
        let assets_server = world.resource::<AssetServer>();

        GameAssets {
            retro_spaceship_image: assets_server.load(RETRO_SPACE_SHIP_IMAGE_PATH),
            cosmic_spaceship_image: assets_server.load(COSMIC_SPACE_SHIP_IMAGE_PATH),
            retro_bullet_image: assets_server.load(RETRO_BULLET_IMAGE_PATH),
            cosmic_bullet_image: assets_server.load(COSMIC_BULLET_IMAGE_PATH),
            retro_asteroids: [
                assets_server.load(RETRO_ASTEROID_PATH_1),
                assets_server.load(RETRO_ASTEROID_PATH_2),
                assets_server.load(RETRO_ASTEROID_PATH_3),
            ],
            cosmic_asteroids: [
                assets_server.load(COSMIC_ASTEROID_PATH_1),
                assets_server.load(COSMIC_ASTEROID_PATH_2),
                assets_server.load(COSMIC_ASTEROID_PATH_3),
            ],
            explosion_sound: assets_server.load(EXPLOSION_SOUND_EFFECT),
            shoot_sound: assets_server.load(SHOOT_SOUND_EFFECT),
            bg_sound: assets_server.load(BACKGROUND_MUSIC_PATH),
            life_decrement_sound: assets_server.load(LOSE_LIFE_SOUND_PATH),
            retro_bg_image: assets_server.load(RETRO_BACKGROUND_IMAGE),
            cosmic_bg_image: assets_server.load(COSMIC_BACKGROUND_IMAGE),

            retro_theme_icon: assets_server.load(RETRO_THEME_ICON),
            cosmic_theme_icon: assets_server.load(COSMIC_THEME_ICON),
        }
    }
}

impl GameAssets {
    pub fn rand_asteroids(&self, game_theme: &GameTheme) -> Handle<Image> {
        let mut rng = rand::rng();

        let asteroids = match game_theme {
            GameTheme::Retro => &self.retro_asteroids,
            GameTheme::Cosmic => &self.cosmic_asteroids,
        };

        asteroids[rng.random_range(0..asteroids.len())].clone()
    }

    pub fn bullet_image(&self, game_theme: &GameTheme) -> Handle<Image> {
        match game_theme {
            GameTheme::Retro => self.retro_bullet_image.clone(),
            GameTheme::Cosmic => self.cosmic_bullet_image.clone(),
        }
    }

    pub fn spaceship_image(&self, game_theme: &GameTheme) -> Handle<Image> {
        match game_theme {
            GameTheme::Retro => self.retro_spaceship_image.clone(),
            GameTheme::Cosmic => self.cosmic_spaceship_image.clone(),
        }
    }

    pub fn get_bg_image(&self, game_theme: &GameTheme) -> Handle<Image> {
        match game_theme {
            GameTheme::Retro => self.retro_bg_image.clone(),
            GameTheme::Cosmic => self.cosmic_bg_image.clone(),
        }
    }

    pub fn get_theme_icon(&self, game_theme: &GameTheme) -> Handle<Image> {
        match game_theme {
            GameTheme::Retro => self.retro_theme_icon.clone(),
            GameTheme::Cosmic => self.cosmic_theme_icon.clone(),
        }
    }
}
