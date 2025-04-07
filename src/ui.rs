use bevy::prelude::*;
// use bevy_cobweb::prelude::*;
// use bevy_cobweb_ui::{prelude::*, sickle::UiBuilderExt};
use bevy_hui::{HuiPlugin, prelude::*};

use crate::{
    app_state::AppState,
    asset_loading::UiComponentAssets,
    shop::{ShopState, ShopStats, on_shop_item_pressed},
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HuiPlugin)
            // .add_plugins(CobwebUiPlugin)
            // .load("ui/main.cob")
            // .add_systems(OnEnter(LoadState::Done), build_ui)
            .add_systems(OnEnter(ShopState::Open), display_shop)
            .add_systems(OnEnter(ShopState::Closed), hide_shop)
            .add_systems(Update, update_shop_ui.run_if(in_state(ShopState::Open)))
            .add_systems(OnEnter(AppState::Game), setup);
        //.add_plugins(HuiAutoLoadPlugin::new(&["ui"]))
        //.add_systems(OnEnter(AutoLoadState::Finished), setup);
    }
}

fn setup(
    mut html_funcs: HtmlFunctions,
    mut html_comps: HtmlComponents,
    assets: Res<UiComponentAssets>,
) {
    html_comps.register("resource", assets.resource.clone());
    html_funcs.register("shop_resources", init_shop_item);
    html_funcs.register("shop_item_pressed", on_shop_item_pressed);
}

fn display_shop(
    mut commands: Commands,
    assets: Res<UiComponentAssets>,
    shop_stats: Res<ShopStats>,
) {
    commands
        .spawn((ShopUi, HtmlNode(assets.shop.clone())))
        .with_children(|commands| {
            commands.spawn((
                ShopItem::Speed,
                HtmlNode(assets.shop_item.clone()),
                Transform::from_xyz(400.0, 50.0, 0.0),
                TemplateProperties::default()
                    .with("title", "Speed Up")
                    .with("level", shop_stats.speed_level.to_string().as_str())
                    .with("icon", "textures/ICON_bomb_ball.png"),
            ));
            commands.spawn((
                ShopItem::Damage,
                HtmlNode(assets.shop_item.clone()),
                Transform::from_xyz(400.0, 50.0, 0.0),
                TemplateProperties::default()
                    .with("title", "Damage Up")
                    .with("level", shop_stats.damage_level.to_string().as_str())
                    .with("icon", "textures/ICON_pro_ball.png"),
            ));
        });
}

fn hide_shop(mut commands: Commands, query: Query<Entity, With<ShopUi>>) {
    for ui in &query {
        commands.entity(ui).try_despawn_recursive();
    }
}

#[derive(Component)]
struct ShopUi;

fn update_shop_ui(
    shop_stats: Res<ShopStats>,
    mut shop_items: Query<(&ShopItem, &mut TemplateProperties, Entity)>,
    mut commands: Commands,
) {
    if shop_stats.is_changed() {
        for mut shop_item in shop_items.iter_mut() {
            match (shop_item.0) {
                ShopItem::Damage => {
                    shop_item
                        .1
                        .insert("level".to_string(), shop_stats.damage_level.to_string());
                }
                ShopItem::Speed => {
                    shop_item
                        .1
                        .insert("level".to_string(), shop_stats.speed_level.to_string());
                }
            }
            commands.trigger_targets(CompileContextEvent, shop_item.2);
        }
    }
}

fn init_shop_item(In(entity): In<Entity>, mut commands: Commands, assets: Res<UiComponentAssets>) {
    commands.entity(entity).with_children(|commands| {
        for i in 0..4 {
            commands.spawn((
                HtmlNode(assets.resource.clone()),
                TemplateProperties::default()
                    .with("count", &format!("{i}"))
                    .with("color", if i % 2 == 0 { "#FFF" } else { "#F88" }),
            ));
        }
    });
}

#[derive(Component, Debug)]
pub enum ShopItem {
    Damage,
    Speed,
}
