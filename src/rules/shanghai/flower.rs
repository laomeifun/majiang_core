// src/rules/shanghai/flower.rs

use crate::hand::Hand;
use crate::game::context::GameContext;
use crate::tile::Tile; // 假设 Tile 定义了花牌

/// 计算花牌带来的番数 (上海麻将特定逻辑)
pub fn calculate_flower_fan(hand: &Hand, context: &GameContext) -> u32 {
    // 上海麻将的花牌规则复杂多样，可能包括：
    // - 正花：与门风匹配的花牌
    // - 碰/杠特定牌型时，对应的花牌算分
    // - 集齐特定组合的花牌 (春夏秋冬、梅兰竹菊)
    // - 花杠等

    let mut flower_fan = 0;
    let player_wind = context.player_wind; // 获取玩家门风

    // 假设手牌中有一个单独的字段存储补到的花牌
    // let flower_tiles = hand.get_flower_tiles();

    // for flower in flower_tiles {
    //     if is_players_correct_flower(flower, player_wind) {
    //         flower_fan += 1; // 正花加一番
    //     }
    //     // 检查其他花牌组合...
    // }

    // 简化：暂不实现花牌逻辑
    flower_fan
}

// --- Helper functions ---

// 假设 Tile 有方法判断是否是花牌以及花牌类型
// fn is_flower_tile(tile: Tile) -> bool { ... }
// fn get_flower_type(tile: Tile) -> FlowerType { ... }
// enum FlowerType { Spring, Summer, ..., Plum, Orchid, ..., Cat, Mouse, ... }

// fn is_players_correct_flower(flower: Tile, player_wind: crate::player::model::WindDirection) -> bool {
//     // 根据门风判断对应的正花 (e.g., 东风对应春/梅/1)
//     ...
// }
