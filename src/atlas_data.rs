use std::path::{Path, PathBuf};

use anyhow::Error;
use bevy::asset::{Asset, AssetLoader, AssetPath, BoxedFuture, LoadContext, LoadedAsset};
use bevy::math::UVec2;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;
use serde::Deserialize;

use crate::animation::Clip;

#[derive(TypeUuid, Debug, Clone, Deserialize)]
#[uuid = "079be8bd-9044-4d0a-beda-88edb139b5bf"]
pub struct AnimationSpriteSheetMeta {
    pub image: String,
    pub tile_size: UVec2,
    pub columns: usize,
    pub rows: usize,
    pub animation_frame_duration: f32,
    pub animations: HashMap<String, Clip>,
    #[serde(skip)]
    pub atlas_handle: Handle<TextureAtlas>,
}

/// Calculate an asset's full path relative to another asset
fn relative_asset_path(asset_path: &Path, relative_path: &str) -> PathBuf {
    let is_relative = !relative_path.starts_with('/');

    if is_relative {
        let base = asset_path.parent().unwrap_or_else(|| Path::new(""));
        base.join(relative_path)
    } else {
        Path::new(relative_path)
            .strip_prefix("/")
            .unwrap()
            .to_owned()
    }
}

/// Helper to get relative asset paths and handles
fn get_relative_asset<T: Asset>(
    load_context: &LoadContext,
    self_path: &Path,
    relative_path: &str,
) -> (AssetPath<'static>, Handle<T>) {
    let asset_path = relative_asset_path(self_path, relative_path);
    let asset_path = AssetPath::new(asset_path, None);
    let handle = load_context.get_handle(asset_path.clone());

    (asset_path, handle)
}

#[derive(Default)]
pub struct AnimationSpriteSheetLoader;

impl AssetLoader for AnimationSpriteSheetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), Error>> {
        Box::pin(async move {
            let mut meta: AnimationSpriteSheetMeta = serde_yaml::from_slice(bytes)?;
            let (image_path, image_handle) =
                get_relative_asset(load_context, load_context.path(), &meta.image);
            let label = "atlas";
            let atlas_handle = load_context.set_labeled_asset(
                label,
                LoadedAsset::new(TextureAtlas::from_grid(
                    image_handle,
                    meta.tile_size.as_vec2(),
                    meta.columns,
                    meta.rows,
                    None,
                    None,
                ))
                .with_dependency(image_path),
            );
            meta.atlas_handle = atlas_handle;
            load_context.set_default_asset(LoadedAsset::new(meta));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["yaml", "yml"]
    }
}
