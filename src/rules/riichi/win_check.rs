// src/rules/riichi/win_check.rs

use crate::hand::Hand;
use crate::rules::common::win_patterns; // 使用通用的和牌型检查

/// 检查手牌是否满足日麻的和牌型 (包括特殊牌型)
pub fn can_form_winning_hand(hand: &Hand) -> bool {
    // 1. 检查标准型 (4面子+1雀头)
    if win_patterns::is_standard_win_shape(hand) {
        return true;
    }

    // 2. 检查七对子
    if is_seven_pairs(hand) {
        return true;
    }

    // 3. 检查国士无双
    if is_kokushi_musou(hand) {
        return true;
    }

    // 其他可能的特殊牌型...

    false
}

/// 检查是否是七对子
pub fn is_seven_pairs(hand: &Hand) -> bool { // Make public
    if !hand.get_melds().is_empty() { return false; } // 七对子必须门清
    let tiles = hand.all_tiles();
    if tiles.len() != 14 { return false; }

    let mut counts = std::collections::HashMap::new();
    for tile in tiles {
        *counts.entry(tile).or_insert(0) += 1;
    }

    counts.len() == 7 && counts.values().all(|&count| count == 2)
}

/// 检查是否是国士无双
pub fn is_kokushi_musou(hand: &Hand) -> bool { // Make public (needed for yaku check later)
    if !hand.get_melds().is_empty() { return false; } // 国士无双必须门清
    let tiles = hand.all_tiles();
    if tiles.len() != 14 { return false; } // 国士无双听牌是13张，和牌是14张

    // 定义幺九牌集合 (需要 Tile 提供方法或常量)
    // let terminals_and_honors: std::collections::HashSet<Tile> = get_terminals_and_honors();

    let mut counts = std::collections::HashMap::new();
    let mut has_pair = false;
    for tile in tiles {
         // if !terminals_and_honors.contains(&tile) { return false; } // 必须是幺九牌
         let count = counts.entry(tile).or_insert(0);
         *count += 1;
         if *count > 2 { return false; } // 不能有超过2张的相同牌
         if *count == 2 {
             if has_pair { return false; } // 不能有多个对子
             has_pair = true;
         }
    }

    // 必须包含所有种类的幺九牌 (13种)，并且有一个对子
    // counts.len() == 13 && has_pair
    unimplemented!("Kokushi check needs Tile constants/methods for terminals/honors");
}

// fn get_terminals_and_honors() -> std::collections::HashSet<Tile> { ... }
