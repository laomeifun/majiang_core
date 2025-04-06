// src/rules/riichi/riichi_specific.rs

use crate::hand::Hand;
use crate::game::context::GameContext;

/// 检查是否满足立直的条件
pub fn check_riichi_conditions(hand: &Hand, context: &GameContext) -> bool {
    // 1. 门前清 (没有副露，暗杠除外 - 简化：不允许任何副露)
    if !hand.get_melds().is_empty() {
        return false;
    }

    // 2. 听牌 (向听数为 0)
    if !is_tenpai(hand) {
        return false;
    }

    // 3. 剩余牌数足够 (通常至少4张)
    if context.game_state.wall.remaining_tiles() < 4 {
        return false;
    }

    // 4. 分数足够 (至少 1000 点)
    if context.game_state.players[context.current_turn()].score < 1000 {
        return false;
    }

    // 5. 不能是已经立直的状态
    if context.game_state.players[context.current_turn()].is_riichi {
        return false;
    }

    true
}

/// 检查是否听牌 (向听数为 0) - 简化版
fn is_tenpai(hand: &Hand) -> bool {
    // 实际需要计算向听数
    // 这里暂时简化为：如果手牌是13张，就认为可能听牌
    hand.all_tiles().len() == 13 || hand.all_tiles().len() == 14 // 考虑摸牌后的情况
    // 这是一个非常不准确的简化！
}

// 其他日麻特有的规则逻辑，例如振听 (furiten) 判断等
// pub fn is_furiten(...) -> bool { ... }
