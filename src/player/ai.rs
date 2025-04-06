// src/player/ai.rs

use crate::action::Action;
use crate::hand::Hand;
use crate::player::actions::get_possible_actions; // 假设我们需要获取所有可能动作
// use crate::game::GameContext; // AI 可能需要游戏上下文信息

/// AI 决策函数
/// 根据当前手牌和可选动作（以及可能的上下文）选择一个动作
pub fn choose_action(hand: &Hand, possible_actions: &[Action] /*, context: &GameContext */) -> Option<Action> {
    if possible_actions.is_empty() {
        return None; // 没有可选动作
    }

    // 简单的 AI 逻辑：
    // 1. 如果可以自摸，则自摸 (需要先判断 Tsumo 是否在 possible_actions 中)
    // 2. 如果轮到自己打牌，优先打掉刚摸的牌 (如果 Discard(drawn_tile) 在列表中)
    // 3. 否则，打掉手牌中的第一张牌 (选择第一个 Discard 动作)
    // 4. 如果有其他非 Pass 动作 (吃碰杠荣和)，优先执行？ (需要更复杂的逻辑)
    // 5. 否则，选择 Pass (如果 Pass 在列表中)

    // 查找 Tsumo 动作
    if let Some(tsumo_action) = possible_actions.iter().find(|&a| matches!(a, Action::Tsumo)) {
        return Some(tsumo_action.clone());
    }

    // 查找打出摸到的牌的动作
    if let Some(drawn_tile) = hand.get_drawn_tile() {
        if let Some(discard_drawn) = possible_actions.iter().find(|&a| *a == Action::Discard(drawn_tile)) {
            return Some(discard_drawn.clone());
        }
    }

    // 查找第一个打牌动作
    if let Some(first_discard) = possible_actions.iter().find(|&a| matches!(a, Action::Discard(_))) {
        return Some(first_discard.clone());
    }

    // 查找第一个非 Pass 动作 (简化逻辑，实际 AI 会更复杂)
     if let Some(other_action) = possible_actions.iter().find(|&a| !matches!(a, Action::Pass)) {
        return Some(other_action.clone());
    }

    // 如果只剩下 Pass，或者没有其他动作，则返回第一个动作 (可能是 Pass)
    possible_actions.first().cloned()

}

/// 另一个 AI 决策函数示例：只负责选择打哪张牌
pub fn choose_discard(hand: &Hand) -> Option<Action> {
     // 简单逻辑：打掉摸到的牌，否则打第一张暗牌
     if let Some(drawn) = hand.get_drawn_tile() {
         Some(Action::Discard(drawn))
     } else if let Some(first_closed) = hand.get_closed_tiles().first() {
         Some(Action::Discard(*first_closed))
     } else {
         None // 没有牌可打？ (异常情况)
     }
}
