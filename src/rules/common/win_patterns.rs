// src/rules/common/win_patterns.rs

use crate::hand::Hand;
use crate::tile::Tile;
use std::collections::HashMap;

/// 检查手牌是否满足标准和牌型（4面子+1雀头）
/// 这是一个非常基础的检查，实际实现会复杂得多，需要考虑国士无双、七对子等特殊牌型
/// 并且需要处理副露
pub fn is_standard_win_shape(hand: &Hand) -> bool { // Ensure it's public
    // 仅考虑门清的情况进行简化示例
    if !hand.get_melds().is_empty() { // Use the public getter
        // 对于有副露的情况，需要将副露考虑进去组合判断
        return false; // 简化：暂不处理有副露的情况
    }

    let tiles = hand.all_tiles(); // 获取所有牌 (包括摸到的牌)
    if tiles.len() != 14 {
        return false; // 标准和牌需要 14 张牌
    }

    // 使用 HashMap 统计牌的数量
    let mut counts = HashMap::new();
    for tile in &tiles {
        *counts.entry(*tile).or_insert(0) += 1;
    }

    // 尝试移除雀头，然后检查剩余的是否都是面子
    for (tile, count) in &counts {
        if *count >= 2 {
            let mut remaining_counts = counts.clone();
            *remaining_counts.get_mut(tile).unwrap() -= 2;
            if remaining_counts[tile] == 0 {
                remaining_counts.remove(tile);
            }

            // 递归检查剩余的牌是否能组成面子
            if can_form_melds(&mut remaining_counts) {
                return true;
            }
        }
    }

    false
}

/// 递归检查牌是否能组成面子 (刻子或顺子)
fn can_form_melds(counts: &mut HashMap<Tile, u8>) -> bool {
    if counts.is_empty() {
        return true; // 所有牌都组成面子了
    }

    // 找到当前最小的牌
    let min_tile = *counts.keys().min().unwrap();
    let count = counts[&min_tile];

    // 尝试组成刻子
    if count >= 3 {
        let mut next_counts = counts.clone();
        *next_counts.get_mut(&min_tile).unwrap() -= 3;
        if next_counts[&min_tile] == 0 {
            next_counts.remove(&min_tile);
        }
        if can_form_melds(&mut next_counts) {
            return true;
        }
    }

    // 尝试组成顺子 (需要判断是否是数字牌)
    // if is_suited_tile(min_tile) { // 需要 Tile 提供判断方法
        // let t1 = min_tile;
        // let t2 = next_tile(t1); // 需要获取下一张牌的逻辑
        // let t3 = next_tile(t2);
        // if let (Some(tile2), Some(tile3)) = (t2, t3) {
        //     if counts.contains_key(&tile2) && counts.contains_key(&tile3) {
        //         let mut next_counts = counts.clone();
        //         // 移除 t1, t2, t3 各一张
        //         // ... (移除逻辑) ...
        //         if can_form_melds(&mut next_counts) {
        //             return true;
        //         }
        //     }
        // }
    // }

    false // 无法用当前最小的牌组成面子
}

// --- 需要 Tile 模块提供更多辅助函数 ---
// fn is_suited_tile(tile: Tile) -> bool { ... }
// fn next_tile(tile: Tile) -> Option<Tile> { ... }
