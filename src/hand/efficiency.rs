// src/hand/efficiency.rs
// 牌效率计算

use super::representation::Hand;
use super::analysis::HandAnalysisResult; // 可能需要分析结果
use crate::tile::Tile;
use std::collections::HashMap;

/// 牌效率信息
#[derive(Debug, Clone)]
pub struct EfficiencyInfo {
    /// 打掉这张牌后的向听数变化 (负数表示向听数减少，即变好)
    pub shanten_change: i8,
    /// 打掉这张牌后的有效进张种类数量
    pub useful_tile_kinds: usize,
    /// 打掉这张牌后的有效进张总张数 (考虑剩余牌)
    pub useful_tile_count: usize,
    /// (可选) 具体的有效进张及其数量
    pub useful_tiles: HashMap<Tile, usize>,
}

/// 计算打出特定牌后的牌效率
///
/// # Arguments
///
/// * `hand` - 当前手牌 (通常是13张或14张)
/// * `tile_to_discard` - 准备打出的牌
/// * `remaining_tiles` - (可选) 牌墙/牌河中剩余牌的信息
///
/// # Returns
///
/// `EfficiencyInfo` 包含打出该牌的效率信息，如果无法打出则返回 None
pub fn calculate_discard_efficiency(
    hand: &Hand,
    tile_to_discard: &Tile,
    _remaining_tiles: Option<&HashMap<Tile, usize>>,
) -> Option<EfficiencyInfo> {
    // TODO: 实现牌效率计算逻辑
    // 1. 复制当前手牌
    let mut temp_hand = hand.clone();

    // 2. 尝试从手牌中移除要打的牌
    //    - 如果是摸到的牌，则移除 drawn_tile
    //    - 否则，从 closed_tiles 移除
    let removed = if hand.get_drawn_tile() == Some(*tile_to_discard) {
        temp_hand.set_drawn_tile(None);
        true
    } else {
        temp_hand.remove_tile(tile_to_discard)
    };

    if !removed {
        return None; // 手牌中没有这张牌，无法打出
    }

    // 3. 计算打牌前的向听数 (如果未传入，需要计算)
    // let initial_shanten = ShantenCalculator::calculate(hand); // 假设有 ShantenCalculator

    // 4. 计算打牌后的向听数
    // let new_shanten = ShantenCalculator::calculate(&temp_hand);

    // 5. 计算有效进张 (Ukeire)
    //    - 遍历所有可能的牌 (34种)
    //    - 模拟摸到这张牌
    //    - 计算摸牌后的向听数
    //    - 如果向听数减少，则该牌为有效进张
    let mut useful_tiles_map = HashMap::new();
    let mut useful_count = 0;
    // for potential_tile in all_possible_tiles() { // 需要一个所有牌类型的迭代器
    //     let mut check_hand = temp_hand.clone();
    //     check_hand.set_drawn_tile(Some(potential_tile));
    //     let shanten_after_draw = ShantenCalculator::calculate(&check_hand);
    //     if shanten_after_draw < new_shanten {
    //         // 这是有效进张
    //         let remaining_count = remaining_tiles.map_or(4, |r| *r.get(&potential_tile).unwrap_or(&0));
    //         if remaining_count > 0 {
    //              useful_tiles_map.insert(potential_tile, remaining_count);
    //              useful_count += remaining_count;
    //         }
    //     }
    // }

    // 占位符返回值
    Some(EfficiencyInfo {
        shanten_change: 0, // new_shanten - initial_shanten,
        useful_tile_kinds: useful_tiles_map.len(),
        useful_tile_count: useful_count,
        useful_tiles: useful_tiles_map,
    })
}

/// (可选) 计算整个手牌的最佳打牌选择（基于牌效率）
///
/// # Arguments
///
/// * `hand` - 当前手牌
/// * `remaining_tiles` - (可选) 剩余牌信息
///
/// # Returns
///
/// 一个元组 `(Tile, EfficiencyInfo)`，表示最佳打出的牌及其效率信息
pub fn find_best_discard(
    hand: &Hand,
    remaining_tiles: Option<&HashMap<Tile, usize>>,
) -> Option<(Tile, EfficiencyInfo)> {
    // TODO: 实现寻找最佳打牌选择的逻辑
    // 1. 获取所有可能的打牌选择 (手牌中的暗牌 + 摸到的牌)
    let mut possible_discards = hand.get_closed_tiles();
    if let Some(drawn) = hand.get_drawn_tile() {
        possible_discards.push(drawn);
    }
    possible_discards.sort_unstable();
    possible_discards.dedup(); // 去重

    // 2. 遍历每种可能的打牌选择，计算其效率
    let mut best_discard = None;
    let mut best_efficiency: Option<EfficiencyInfo> = None;

    for tile in possible_discards {
        if let Some(efficiency) = calculate_discard_efficiency(hand, &tile, remaining_tiles) {
            // 3. 比较效率，选择最优的打牌
            //    比较逻辑：
            //    - 优先选择向听数能减少最多的 (shanten_change 最小)
            //    - 如果向听数变化相同，优先选择有效进张种类最多的 (useful_tile_kinds 最大)
            //    - 如果种类数也相同，优先选择有效进张总数最多的 (useful_tile_count 最大)
            //    - (可选) 其他启发式规则，如保留好型搭子等
            let is_better = match &best_efficiency {
                None => true,
                Some(current_best) => {
                    if efficiency.shanten_change < current_best.shanten_change {
                        true
                    } else if efficiency.shanten_change == current_best.shanten_change {
                        if efficiency.useful_tile_kinds > current_best.useful_tile_kinds {
                            true
                        } else if efficiency.useful_tile_kinds == current_best.useful_tile_kinds {
                            efficiency.useful_tile_count > current_best.useful_tile_count
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            };

            if is_better {
                best_discard = Some(tile);
                best_efficiency = Some(efficiency);
            }
        }
    }

    best_discard.zip(best_efficiency)
}

// 占位符函数，实际需要 tile 模块提供
// fn all_possible_tiles() -> impl Iterator<Item = Tile> {
//     // ...
//     std::iter::empty()
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hand::representation::Hand;
    use crate::tile::{Tile, TileSuit};

    #[test]
    fn test_efficiency_placeholder() {
        // 由于依赖未完成的向听数计算，这里只做基本测试
        let mut hand = Hand::new();
        hand.add_tile(Tile::new_numbered(TileSuit::Characters, 1));
        hand.add_tile(Tile::new_numbered(TileSuit::Characters, 2));
        hand.add_tile(Tile::new_numbered(TileSuit::Characters, 3));
        hand.add_tile(Tile::new_numbered(TileSuit::Characters, 4));
        hand.set_drawn_tile(Some(Tile::new_numbered(TileSuit::Characters, 5))); // 1234 m 摸 5m

        let discard_4m = Tile::new_numbered(TileSuit::Characters, 4);
        let efficiency = calculate_discard_efficiency(&hand, &discard_4m, None);

        assert!(efficiency.is_some());
        // TODO: 添加更具体的断言，当向听数和进张计算实现后
        // let info = efficiency.unwrap();
        // assert_eq!(info.shanten_change, ...);
        // assert_eq!(info.useful_tile_kinds, ...);

        let best_choice = find_best_discard(&hand, None);
        assert!(best_choice.is_some());
        // TODO: 断言最佳选择
        // assert_eq!(best_choice.unwrap().0, ...); // 应该打哪张牌
    }
}
