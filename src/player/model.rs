// src/player/model.rs

use crate::hand::Hand;
use crate::tile::Tile;
use strum::Display; // 引入 Display trait

/// 玩家状态与数据结构
#[derive(Debug, Clone)]
pub struct Player {
    pub id: usize,          // 玩家唯一标识符或座位索引 (0-3)
    pub hand: Hand,         // 玩家手牌
    pub score: i32,         // 玩家分数
    pub discards: Vec<Tile>, // 玩家打出的牌
    pub is_riichi: bool,    // 是否立直
    pub wind: WindDirection, // 玩家的门风 (东/南/西/北)
    // 可以添加更多状态，例如是否是庄家 (is_dealer) 等
}

/// 风向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)] // 添加 Display
pub enum WindDirection {
    #[strum(to_string = "东")] // 自定义显示名称
    East,
    #[strum(to_string = "南")]
    South,
    #[strum(to_string = "西")]
    West,
    #[strum(to_string = "北")]
    North,
}

impl Player {
    /// 创建一个新玩家
    pub fn new(id: usize, initial_score: i32, wind: WindDirection) -> Self {
        Player {
            id,
            hand: Hand::new(),
            score: initial_score,
            discards: Vec::new(),
            is_riichi: false,
            wind,
        }
    }

    /// 玩家打出一张牌
    /// 尝试从手牌暗牌或摸到的牌中移除指定的牌，并将其加入弃牌堆
    pub fn discard_tile(&mut self, tile: Tile) -> Result<(), String> {
        // 优先尝试移除摸到的牌
        if self.hand.is_drawn_tile(&tile) {
            self.hand.set_drawn_tile(None); // 清空摸到的牌
            self.discards.push(tile);
            Ok(())
        }
        // 否则，尝试从暗牌中移除
        else if self.hand.remove_closed_tile(&tile) {
            self.discards.push(tile);
            Ok(())
        }
        // 如果都找不到
        else {
            Err(format!("Tile {:?} not found in hand or drawn tile for discard", tile))
        }
    }

    // 其他玩家相关的逻辑...
}
