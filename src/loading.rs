use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::atlas_data::{AnimationSpriteSheetLoader, AnimationSpriteSheetMeta};
use crate::GameState;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TextureAtlasSprite>()
            .add_asset::<AnimationSpriteSheetMeta>()
            .add_asset_loader(AnimationSpriteSheetLoader)
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Playing)
                    .with_collection::<FontAssets>()
                    .with_collection::<AudioAssets>()
                    .with_collection::<TextureAssets>(),
            );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/cyborg.yml")]
    pub cyborg: Handle<AnimationSpriteSheetMeta>,
    #[asset(path = "textures/hand.png")]
    pub hand: Handle<Image>,
    #[asset(path = "textures/gun.png")]
    pub gun: Handle<Image>,
    #[asset(path = "textures/shoot_effect.yml")]
    pub shoot_effect: Handle<AnimationSpriteSheetMeta>,
    #[asset(path = "textures/bullet.png")]
    pub bullet: Handle<Image>,
}
