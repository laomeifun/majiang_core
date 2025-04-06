// src/meld.rs

use crate::tile::Tile;

/// 副露类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeldType {
    Chi(Tile, Tile, Tile), // 吃牌，包含三张牌
    Pon(Tile, Tile, Tile), // 碰牌，包含三张相同的牌
    Kan(Tile, Tile, Tile, Tile), // 杠牌，包含四张相同的牌 (明杠或暗杠)
    // 可以进一步区分明杠、暗杠、加杠
}

/// 代表一个副露
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Meld {
    meld_type: MeldType,
    // 可以添加来源信息，例如从哪个玩家吃/碰
    // from_player_index: Option<usize>,
}

impl Meld {
    // 创建副露的构造函数...
    pub fn new_chi(t1: Tile, t2: Tile, t3: Tile) -> Self {
        // 需要确保 t1, t2, t3 是合法的顺子
        Meld { meld_type: MeldType::Chi(t1, t2, t3) }
    }

    pub fn new_pon(tile: Tile) -> Self {
        Meld { meld_type: MeldType::Pon(tile, tile, tile) }
    }

    pub fn new_kan(tile: Tile) -> Self {
        Meld { meld_type: MeldType::Kan(tile, tile, tile, tile) }
    }

    // 获取副露类型的函数...
    pub fn get_type(&self) -> &MeldType {
        &self.meld_type
    }

    // 获取组成副露的牌...
    pub fn tiles(&self) -> Vec<Tile> {
        match &self.meld_type {
            MeldType::Chi(t1, t2, t3) => vec![*t1, *t2, *t3],
            MeldType::Pon(t1, t2, t3) => vec![*t1, *t2, *t3],
            MeldType::Kan(t1, t2, t3, t4) => vec![*t1, *t2, *t3, *t4],
        }
    }
}
