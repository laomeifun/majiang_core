// src/rules/mcr/win_check.rs

use crate::hand::Hand;
use crate::game::context::GameContext;
use crate::rules::common::win_patterns;

/// 检查手牌是否满足国标麻将的和牌条件 (至少 8 分)
pub fn can_win_mcr(hand: &Hand, context: &GameContext, is_tsumo: bool) -> bool {
    // 1. 检查基本和牌型 (4 面子 + 1 雀头 或 特殊牌型)
    if !is_mcr_winning_shape(hand) {
        return false;
    }

    // 2. 计算番种得分，必须达到 8 分
    let score_result = super::scoring::calculate_mcr_score(hand, context, hand.get_drawn_tile().unwrap_or(crate::tile::Tile::placeholder()), is_tsumo); // 传入一个占位符和牌
    match score_result {
        Some(result) => result.score >= 8, // 国标要求至少 8 分
        None => false, // 无法计算分数或牌型错误
    }
}

/// 检查是否是国标麻将允许的和牌牌型 (包括特殊牌型)
fn is_mcr_winning_shape(hand: &Hand) -> bool {
    // 检查标准型 (4 面子 + 1 雀头)
    if win_patterns::is_standard_win_shape(hand) {
        return true;
    }

    // 检查国标的特殊牌型，例如：
    // - 七对 (需要特定实现，国标七对允许有四张一样的)
    // - 十三幺 (国标叫十三幺)
    // - 全不靠 / 组合龙 等...

    // 简化：只允许标准型
    false
}

// 可能需要国标特有的牌型检查函数
// fn is_mcr_seven_pairs(...) -> bool { ... }
// fn is_mcr_thirteen_orphans(...) -> bool { ... }
// fn is_mcr_all_not_related(...) -> bool { ... }
