use bevy::prelude::*;
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState},
    standard_dynamic_asset::StandardDynamicAssetCollection,
};
use bevy_hui::prelude::{HtmlNode, HtmlTemplate};

use crate::app_state::AppState;

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::LoadingAssets)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("game.assets.ron")
                .load_collection::<GameImageAssets>()
                .load_collection::<UiComponentAssets>()
                .continue_to_state(AppState::Game)
                .on_failure_continue_to_state(AppState::BadStateSadEmoji),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct UiComponentAssets {
    #[asset(path = "ui/shop.html")]
    pub shop: Handle<HtmlTemplate>,
    #[asset(path = "ui/components/shop_item.html")]
    pub shop_item: Handle<HtmlTemplate>,
    #[asset(path = "ui/components/resource.html")]
    pub resource: Handle<HtmlTemplate>,
}

#[derive(AssetCollection, Resource)]
pub struct GameImageAssets {
    #[asset(key = "background")]
    pub background: Handle<Image>,

    #[asset(key = "crack")]
    pub crack: Handle<Image>,

    #[asset(key = "blue")]
    pub blue: Handle<Image>,
    #[asset(key = "dark_blue")]
    pub dark_blue: Handle<Image>,
    #[asset(key = "light_blue_transparent")]
    pub light_blue: Handle<Image>,
    #[asset(key = "purple")]
    pub purple: Handle<Image>,
    #[asset(key = "light_purple")]
    pub light_purple: Handle<Image>,
    #[asset(key = "pink")]
    pub pink: Handle<Image>,
    #[asset(key = "red")]
    pub red: Handle<Image>,
    #[asset(key = "orange")]
    pub orange: Handle<Image>,

    #[asset(key = "ufo_top")]
    pub ufo_top: Handle<Image>,
    #[asset(key = "ufo_bottom")]
    pub ufo_bottom: Handle<Image>,

    #[asset(key = "ball")]
    pub ball: Handle<Image>,

    #[asset(key = "damage_icon")]
    pub damage_icon: Handle<Image>,
    #[asset(key = "speed_icon")]
    pub speed_icon: Handle<Image>,
}
