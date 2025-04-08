use std::collections::HashMap;

use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    app_state::AppState,
    ball::CollectedResources,
    blocks::BlockType,
    paddle::{Paddle, PaddleAction},
};

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShopStats>()
            .add_sub_state::<ShopState>()
            .add_systems(
                FixedUpdate,
                check_for_shop_toggle.run_if(in_state(AppState::Game)),
            );
    }
}

#[derive(Component, Debug)]
pub enum ShopItem {
    Damage,
    Speed,
    Capacity,
    Size,
}

#[derive(Resource)]
pub struct ShopStats {
    pub damage_level: u8,
    pub speed_level: u8,
    pub capacity_level: u8,
    pub size_level: u8,
}

impl ShopStats {
    pub fn damage(&self) -> u16 {
        if self.damage_level > 5 {
            return 20;
        }
        self.damage_level as u16
    }

    pub fn damage_cost(&self) -> Option<HashMap<BlockType, u32>> {
        match self.damage_level {
            1 => Some(HashMap::from([(BlockType::Purple, 6)])),
            2 => Some(HashMap::from([
                (BlockType::LightPurple, 5),
                (BlockType::Purple, 10),
            ])),
            3 => Some(HashMap::from([
                (BlockType::Pink, 5),
                (BlockType::Purple, 10),
            ])),
            4 => Some(HashMap::from([(BlockType::Red, 10)])),
            5 => Some(HashMap::from([(BlockType::Orange, 10)])),
            _ => None,
        }
    }

    pub fn speed(&self) -> f32 {
        self.speed_level as f32 * 100.0
    }

    pub fn speed_cost(&self) -> Option<HashMap<BlockType, u32>> {
        match self.speed_level {
            1 => Some(HashMap::from([(BlockType::LightBlue, 50)])),
            2 => Some(HashMap::from([(BlockType::LightBlue, 100)])),
            3 => Some(HashMap::from([
                (BlockType::LightBlue, 200),
                (BlockType::DarkBlue, 50),
            ])),
            4 => Some(HashMap::from([
                (BlockType::LightBlue, 500),
                (BlockType::DarkBlue, 200),
            ])),
            _ => None,
        }
    }

    pub fn capacity(&self) -> u32 {
        self.capacity_level as u32 * 3
    }

    pub fn capacity_cost(&self) -> Option<HashMap<BlockType, u32>> {
        match self.capacity_level {
            1 => Some(HashMap::from([(BlockType::LightBlue, 10)])),
            2 => Some(HashMap::from([(BlockType::Pink, 20)])),
            3 => Some(HashMap::from([(BlockType::Purple, 40)])),
            4 => Some(HashMap::from([(BlockType::Red, 5), (BlockType::Blue, 500)])),
            _ => None,
        }
    }

    pub fn size(&self) -> f32 {
        self.size_level as f32 * 10.0 + 30.0
    }

    pub fn size_cost(&self) -> Option<HashMap<BlockType, u32>> {
        match self.size_level {
            1 => Some(HashMap::from([(BlockType::LightBlue, 10)])),
            2 => Some(HashMap::from([
                (BlockType::Red, 1),
                (BlockType::LightPurple, 10),
            ])),
            3 => Some(HashMap::from([(BlockType::Red, 10), (BlockType::Pink, 10)])),
            4 => Some(HashMap::from([
                (BlockType::Orange, 5),
                (BlockType::Pink, 10),
            ])),
            _ => None,
        }
    }
}

impl Default for ShopStats {
    fn default() -> Self {
        Self {
            damage_level: 1,
            speed_level: 1,
            capacity_level: 1,
            size_level: 1,
        }
    }
}

#[derive(SubStates, Debug, Default, Hash, PartialEq, Eq, Clone)]
#[source(AppState = AppState::Game)]
pub enum ShopState {
    #[default]
    Closed,
    Open,
}

fn check_for_shop_toggle(
    mut query: Query<&ActionState<PaddleAction>>,
    time: Res<Time>,
    mut commands: Commands,
    shop_state: Res<State<ShopState>>,
    mut next_state: ResMut<NextState<ShopState>>,
) {
    for action in &query {
        if action.just_pressed(&PaddleAction::Interact) {
            next_state.set(if *shop_state.get() == ShopState::Open {
                ShopState::Closed
            } else {
                ShopState::Open
            });
        }
    }
}

pub fn on_shop_item_pressed(
    In(entity): In<Entity>,
    query: Query<&ShopItem>,
    mut shop_stats: ResMut<ShopStats>,
    mut commands: Commands,
    mut paddle_query: Query<&mut CollectedResources, With<Paddle>>,
) {
    let mut resources = paddle_query
        .get_single_mut()
        .expect("Need single paddle to try buy.");
    if let Ok(item) = query.get(entity) {
        if let Some(cost) = match item {
            ShopItem::Damage => shop_stats.damage_cost(),
            ShopItem::Speed => shop_stats.speed_cost(),
            ShopItem::Capacity => shop_stats.capacity_cost(),
            ShopItem::Size => shop_stats.size_cost(),
        } {
            if try_buy(&cost, &mut resources.counts) {
                match item {
                    ShopItem::Damage => shop_stats.damage_level += 1,
                    ShopItem::Speed => shop_stats.speed_level += 1,
                    ShopItem::Capacity => shop_stats.capacity_level += 1,
                    ShopItem::Size => shop_stats.size_level += 1,
                }
            } else {
                info!("Failed to buy: {:?}", item);
            }
        }
    }
}

pub fn try_buy(reqs: &HashMap<BlockType, u32>, owned: &mut HashMap<BlockType, u32>) -> bool {
    info!("Trying to buy, needs: {:?} has:{:?}", reqs, owned);
    let mut succeeded = true;
    for (block, &count) in reqs {
        if let Some(&has) = owned.get(block) {
            if has < count {
                succeeded = false;
                break;
            }
        } else if count > 0 {
            succeeded = false;
            break;
        }
    }
    if succeeded {
        for (block, &count) in reqs {
            if let Some(val) = owned.get_mut(block) {
                *val -= count;
            }
        }
    }
    succeeded
}
