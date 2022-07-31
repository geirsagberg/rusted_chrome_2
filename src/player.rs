use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::actions::Actions;
use crate::GameState;
use crate::loading::{AtlasJson, TextureAssets};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            GameState::Playing,
            SystemSet::new().with_system(spawn_player),
        )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(move_player)
                    .into(),
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    texts: Res<Assets<AtlasJson>>,
) {
    let json = texts.get(&textures.cyborg_atlas_json).unwrap();
    println!("{:?}", json);
    let mut sprite_sheet_bundle = SpriteSheetBundle {
        texture_atlas: textures.cyborg_atlas.clone(),
        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        ..Default::default()
    };
    sprite_sheet_bundle.sprite.index = 2;
    commands
        .spawn_bundle(sprite_sheet_bundle)

        .insert(Player);
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in player_query.iter_mut() {
        player_transform.translation += movement;
    }
}
