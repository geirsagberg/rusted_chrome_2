use std::collections::HashMap;

use bevy::asset::{AssetLoader, BoxedFuture, Error, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_asset_loader::prelude::*;
use serde::{Deserialize, Serialize};

use crate::GameState;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<AtlasJson>()
            .init_asset_loader::<AtlasJsonLoader>()
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
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
    #[asset(path = "textures/cyborg.png")]
    #[asset(texture_atlas(tile_size_x = 48., tile_size_y = 48., columns = 24, rows = 1))]
    pub cyborg_atlas: Handle<TextureAtlas>,
    #[asset(path = "textures/cyborg.json")]
    pub cyborg_atlas_json: Handle<AtlasJson>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bounds {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Size {
    pub w: u16,
    pub h: u16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtlasFrame {
    pub frame: Bounds,
    pub rotated: bool,
    pub trimmed: bool,
    pub sprite_source_size: Bounds,
    pub source_size: Size,
}

#[derive(TypeUuid, Debug, Serialize, Deserialize)]
#[uuid = "079be8bd-9044-4d0a-beda-88edb139b5bf"]
pub struct AtlasJson {
    pub frames: HashMap<String, AtlasFrame>,
}

#[derive(Default)]
pub struct AtlasJsonLoader;

impl AssetLoader for AtlasJsonLoader {
    fn load<'a>(&'a self, bytes: &'a [u8], load_context: &'a mut LoadContext) -> BoxedFuture<'a, anyhow::Result<(), Error>> {
        Box::pin(async move {
            let text = std::str::from_utf8(bytes).unwrap();
            let json: AtlasJson = serde_json::from_str(text).unwrap();
            load_context.set_default_asset(LoadedAsset::new(json));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}
