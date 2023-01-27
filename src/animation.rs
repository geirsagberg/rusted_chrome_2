use std::ops::Range;

use bevy::{prelude::*, sprite::TextureAtlasSprite, utils::HashMap};
use iyes_loopless::condition::ConditionSet;
use serde::de::SeqAccess;
use serde::Deserializer;

use crate::{CoreStage, GameState};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::Last,
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .with_system(animation_cycling)
                .into(),
        );
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Clip {
    #[serde(deserialize_with = "deserialize_range_from_array")]
    pub frames: Range<usize>,
    #[serde(default)]
    pub repeat: bool,
}

fn deserialize_range_from_array<'de, D>(de: D) -> Result<Range<usize>, D::Error>
where
    D: Deserializer<'de>,
{
    de.deserialize_tuple(2, RangeVisitor)
}

struct RangeVisitor;

impl<'de> serde::de::Visitor<'de> for RangeVisitor {
    type Value = Range<usize>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A sequence of 2 integers")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let start: usize = if let Some(start) = seq.next_element()? {
            start
        } else {
            return Err(serde::de::Error::invalid_length(
                0,
                &"a sequence with a length of 2",
            ));
        };
        let end: usize = if let Some(end) = seq.next_element()? {
            end
        } else {
            return Err(serde::de::Error::invalid_length(
                1,
                &"a sequence with a length of 2",
            ));
        };

        Ok(start..end)
    }
}

#[derive(Component)]
pub struct Animation {
    pub animations: HashMap<String, Clip>,
    pub current_frame: usize,
    pub current_animation: Option<String>,
    pub timer: Timer,
    pub played_once: bool,
}

impl Animation {
    pub fn new(frame_duration: f32, animations: HashMap<String, Clip>) -> Self {
        Self {
            animations,
            current_frame: 0,
            current_animation: None,
            timer: Timer::from_seconds(frame_duration, TimerMode::Repeating),
            played_once: false,
        }
    }

    /// Start playing a new animation
    pub fn play(&mut self, name: &str, repeating: bool) {
        self.current_animation = Some(name.to_owned());
        self.current_frame = 0;
        self.timer.reset();
        self.timer.unpause();
        self.timer.set_mode(if repeating {
            TimerMode::Repeating
        } else {
            TimerMode::Once
        });
        self.played_once = false;
    }

    pub fn is_finished(&self) -> bool {
        self.played_once
    }

    pub fn is_repeating(&self) -> bool {
        if let Some(animation) = &self.current_animation {
            if let Some(clip) = self.animations.get(animation) {
                return clip.repeat;
            }
        }

        false
    }

    pub fn is_last_frame(&self) -> bool {
        if let Some(indices) = self.get_current_indices() {
            if let Some(index) = self.get_current_index() {
                return index >= indices.end;
            }
        }

        false
    }

    pub fn get_current_indices(&self) -> Option<&Range<usize>> {
        if let Some(animation) = &self.current_animation {
            return match self.animations.get(animation) {
                Some(clip) => Some(&clip.frames),
                None => None,
            };
        }

        None
    }

    pub fn get_current_index(&self) -> Option<usize> {
        if let Some(indices) = self.get_current_indices() {
            return Some(indices.start + self.current_frame);
        }

        None
    }
}

fn animation_cycling(mut query: Query<(&mut TextureAtlasSprite, &mut Animation)>, time: Res<Time>) {
    //TODO: Add a tick method on Animation
    for (mut texture_atlas_sprite, mut animation) in query.iter_mut() {
        if animation.is_finished() && !animation.is_repeating() {
            continue;
        }

        animation.timer.tick(time.delta());

        if animation.timer.finished() {
            animation.timer.reset();

            if animation.is_last_frame() {
                animation.played_once = true; // Check if animation player here because we need to wait the last frame

                if animation.is_repeating() {
                    animation.current_frame = 0;
                }
            } else {
                animation.current_frame += 1;
            }
        }

        if let Some(index) = animation.get_current_index() {
            texture_atlas_sprite.index = index;
        }
    }
}
