// src/player/actions.rs

use crate::tile::Tile;
use crate::action::Action;
use crate::hand::Hand;
use crate::meld::Meld;
// 可能需要引入游戏状态或上下文来判断动作合法性
// use crate::game::GameState;

/// 检查玩家是否可以执行某个动作
/// 这个函数可能需要更多上下文信息 (例如其他玩家的弃牌，游戏状态等)
pub fn can_perform_action(hand: &Hand, action: &Action /*, context: &GameContext */) -> bool {
    match action {
        Action::Discard(tile) => {
            // 检查手牌中是否有这张牌 (包括摸到的牌)
            hand.contains_closed_tile(tile) || hand.is_drawn_tile(tile)
        }
        Action::Riichi(tile) => {
            // 检查是否满足立直条件 (门清, 听牌, 足够分数等)
            // 并且检查打出的牌是否在手牌中
            // is_menzen(hand) && is_tenpai(hand) && score >= 1000 &&
            // (hand.contains_closed_tile(tile) || hand.is_drawn_tile(tile))
            // 实际检查会更复杂
            // 暂时返回 false 或使用 unimplemented!() 宏
            unimplemented!("Riichi check not implemented") // Macro call as expression
        }
        Action::Tsumo => {
            // 检查是否和牌 (需要规则和上下文)
            // can_win(hand, context, true);
            unimplemented!("Tsumo check not implemented") // Macro call as expression
        }
        Action::Ron(_tile) => { // 参数名改为 _tile 避免未使用警告
            // 检查是否能荣和这张牌 (需要规则和上下文)
            // can_win(hand + tile, context, false);
            unimplemented!("Ron check not implemented") // Macro call as expression
        }
        Action::Chi(_, _, _) => {
            // 检查是否能吃这张牌 (需要上家打出的牌和上下文)
            unimplemented!("Chi check not implemented") // Macro call as expression
        }
        Action::Pon(_) => {
            // 检查是否能碰这张牌 (需要其他玩家打出的牌和上下文)
            unimplemented!("Pon check not implemented") // Macro call as expression
        }
        Action::Kan(_) => {
            // 检查是否能杠这张牌 (需要上下文，区分明杠/暗杠/加杠)
            unimplemented!("Kan check not implemented") // Macro call as expression
        }
        Action::Pass => {
            // 通常总是可以 Pass
            true
        }
        // ... 其他动作检查
    }
}

/// 获取玩家所有可能的合法动作
/// 这个函数同样需要更多上下文
pub fn get_possible_actions(hand: &Hand /*, context: &GameContext */) -> Vec<Action> {
    let mut actions = Vec::new();
    let drawn_tile_opt = hand.get_drawn_tile();

    // 检查打牌动作 (必须打一张)
    if let Some(drawn) = drawn_tile_opt {
        actions.push(Action::Discard(drawn));
    }
    for tile in hand.get_closed_tiles() {
         // 避免重复添加打摸到的牌的选项
        if drawn_tile_opt != Some(*tile) {
            actions.push(Action::Discard(*tile));
        }
    }


    // 检查其他动作 (立直, 和牌, 吃碰杠等)
    // ... 根据上下文判断是否可以添加 Action::Riichi, Action::Tsumo, etc. ...

    actions
}

// --- Helper functions (examples, need proper implementation) ---

// fn is_menzen(hand: &Hand) -> bool {
//     hand.melds.is_empty() // 简化判断，实际可能需要区分暗杠
// }

// fn is_tenpai(hand: &Hand) -> bool {
//     // 计算向听数，判断是否为 0
//     unimplemented!()
// }
