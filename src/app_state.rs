use bevy::{reflect::Reflect, state::state::States};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States, Reflect)]
pub enum AppState {
    #[default]
    LoadingAssets,
    Game,
    BadStateSadEmoji,
}
