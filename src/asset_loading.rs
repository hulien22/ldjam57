use bevy::prelude::*;
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState},
    standard_dynamic_asset::StandardDynamicAssetCollection,
};

use crate::app_state::AppState;

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::LoadingAssets)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("game.assets.ron")
                .load_collection::<GameImageAssets>()
                .continue_to_state(AppState::Game)
                .on_failure_continue_to_state(AppState::BadStateSadEmoji),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct GameImageAssets {
    #[asset(key = "crack")]
    pub crack: Handle<Image>,
    #[asset(key = "dirt")]
    pub dirt: Handle<Image>,
    #[asset(key = "iron")]
    pub iron: Handle<Image>,
    #[asset(key = "obsidian")]
    pub obsidian: Handle<Image>,
}
