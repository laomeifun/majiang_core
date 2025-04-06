// src/rules/mcr/flower.rs

use crate::hand::Hand;
use crate::tile::Tile; // 假设 Tile 定义了花牌

/// 计算国标麻将中花牌的得分 (每张 1 分)
pub fn calculate_mcr_flower_score(hand: &Hand) -> u32 {
    // 国标规则中，每张花牌计 1 分，不计入手牌组合

    let mut flower_score = 0;

    // 假设手牌中有一个单独的字段存储补到的花牌
    // let flower_tiles = hand.get_flower_tiles();

    // for _flower in flower_tiles {
    //     // 检查是否是有效的花牌 (春夏秋冬梅兰竹菊)
    //     // if is_valid_mcr_flower(flower) {
    //          flower_score += 1;
    //     // }
    // }

    // 简化：暂不实现花牌逻辑
    flower_score
}

// --- Helper functions ---
// fn is_valid_mcr_flower(tile: Tile) -> bool { ... }
