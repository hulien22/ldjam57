use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};

use crate::{app_state::AppState, asset_loading::AudioAssets, blocks::DespawnHack};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_kira_audio::AudioPlugin)
            .add_observer(play_pop)
            .add_systems(OnEnter(AppState::Game), play_music);
    }
}

fn play_music(assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play(assets.music.clone()).looped();
}

fn play_pop(trigger: Trigger<OnAdd, DespawnHack>, assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play(assets.pop.clone());
}
