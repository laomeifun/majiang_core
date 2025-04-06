// src/action.rs

use crate::tile::Tile;
use crate::meld::MeldType; // 可能需要 MeldType 来表示吃碰杠的具体组合

/// 玩家可以执行的动作类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Discard(Tile),      // 打牌
    Chi(Tile, Tile, Tile), // 吃牌 (指定吃的组合)
    Pon(Tile),          // 碰牌 (指定碰的牌)
    Kan(Tile),          // 杠牌 (指定杠的牌，可能是明杠、暗杠或加杠)
    Riichi(Tile),       // 立直 (指定打出的牌)
    Tsumo,              // 自摸和牌
    Ron(Tile),          // 荣和 (指定和的牌)
    Pass,               // 跳过 (例如不吃、不碰、不和)
    // 可能还有其他动作，例如九种九牌流局等
}

/// 描述一个动作请求或通知
#[derive(Debug, Clone)]
pub struct ActionEvent {
    pub player_index: usize, // 执行动作的玩家索引
    pub action: Action,      // 执行的动作
    // 可以添加其他上下文信息，例如动作发生的回合数等
}
