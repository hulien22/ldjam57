use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    ActiveCollisionTypes, ActiveEvents, Collider, CollisionGroups, Sensor,
};
use strum::IntoEnumIterator;

use crate::{
    app_state::AppState,
    asset_loading::GameImageAssets,
    blocks::BlockType,
    physics::PADDLE_SHOP_GROUP,
    shop::{ShopItem, ShopStats},
};

pub struct ShopPanelPlugin;

impl Plugin for ShopPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_shop)
            .add_observer(update_shop_panels);
    }
}

#[derive(Component)]
pub struct ShopPanel {
    pub enabled: bool,
    pub item: ShopItem,
    pub upgrade: String,
}

#[derive(Component)]
pub struct ShopPanelText;

#[derive(Component)]
pub struct ShopResourceCost(pub BlockType);

fn spawn_shop(mut commands: Commands, assets: Res<GameImageAssets>) {
    // Spawn text
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                bottom: Val::Px(80.0),
                ..default()
            },
            Name::new("Shop Text Background"),
        ))
        .with_children(|parent| {
            parent.spawn((
                ShopPanelText,
                Text::new(""),
                TextFont { ..default() },
                TextColor(Color::WHITE),
                Node { ..default() },
                BackgroundColor(Color::srgba(33.0 / 256.0, 33.0 / 256.0, 33.0 / 256.0, 1.0)),
                Name::new("Shop Text"),
            ));

            let mut costs = parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    ..default()
                },
                BackgroundColor(Color::srgba(33.0 / 256.0, 33.0 / 256.0, 33.0 / 256.0, 1.0)),
            ));
            for block_type in BlockType::iter() {
                costs.with_children(|parent| {
                    parent.spawn((
                        Text::new(format!(" 1 ")),
                        TextFont { ..default() },
                        TextColor(block_type.colour()),
                        ShopResourceCost(block_type),
                    ));
                });
            }
        });

    commands.spawn((
        Sprite {
            image: assets.shop_background.clone(),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        },
        ShopPanel {
            enabled: false,
            item: ShopItem::Speed,
            upgrade: "Speed Upgrade".to_string(),
        },
        Transform::from_xyz(-100.0, 100.0, -100.0),
        Name::new("Speed Upgrade"),
        (
            Collider::cuboid(25.0, 25.0),
            ActiveCollisionTypes::all(),
            ActiveEvents::COLLISION_EVENTS,
            Sensor,
            CollisionGroups::new(PADDLE_SHOP_GROUP, PADDLE_SHOP_GROUP),
        ),
    ));
    commands.spawn((
        Sprite {
            image: assets.shop_background.clone(),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        },
        ShopPanel {
            enabled: false,
            item: ShopItem::Damage,
            upgrade: "Ball Damage Upgrade".to_string(),
        },
        Transform::from_xyz(-200.0, 100.0, -100.0),
        Name::new("Ball Upgrade"),
        (
            Collider::cuboid(25.0, 25.0),
            ActiveCollisionTypes::all(),
            ActiveEvents::COLLISION_EVENTS,
            Sensor,
            CollisionGroups::new(PADDLE_SHOP_GROUP, PADDLE_SHOP_GROUP),
        ),
    ));

    commands.trigger(UpdateShopPanelsEvent);
}

#[derive(Event, Debug, Default)]
pub struct UpdateShopPanelsEvent;

fn update_shop_panels(
    trigger: Trigger<UpdateShopPanelsEvent>,
    mut query: Query<(&mut Sprite, &ShopPanel)>,
    mut shop_text_query: Query<&mut Text, (With<ShopPanelText>, Without<ShopResourceCost>)>,
    mut shop_cost_query: Query<(&mut Text, &ShopResourceCost)>,
    mut shop_stats: ResMut<ShopStats>,
) {
    let mut shop_text = shop_text_query
        .get_single_mut()
        .expect("Need single shop text to update.");

    let mut any_enabled = false;
    for (mut sprite, shop_panel) in query.iter_mut() {
        if shop_panel.enabled {
            let level = match shop_panel.item {
                ShopItem::Damage => shop_stats.damage_level,
                ShopItem::Speed => shop_stats.speed_level,
            };
            any_enabled = true;
            sprite.color = Color::WHITE; // Normal color

            shop_text.0 = format!("Press <E> to buy {} (level {})", shop_panel.upgrade, level);
            // update costs
            if let Some(cost) = match shop_panel.item {
                ShopItem::Damage => shop_stats.damage_cost(),
                ShopItem::Speed => shop_stats.speed_cost(),
            } {
                for (mut text, block_type) in shop_cost_query.iter_mut() {
                    if let Some(&count) = cost.get(&block_type.0) {
                        text.0 = format!(" {} ", count);
                    } else {
                        text.0 = format!("");
                    }
                }
            }
        } else {
            sprite.color = Color::srgb(0.5, 0.5, 0.5); // Dull color
        }
    }

    if !any_enabled {
        shop_text.0 = "".to_string();
        // clear all costs
        for (mut text, _) in shop_cost_query.iter_mut() {
            text.0 = format!("");
        }
    }
}
