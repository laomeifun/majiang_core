// src/meld/types.rs
// 定义 Meld 结构体, MeldType 枚举, KanType 枚举等

use crate::tile::Tile;
use std::fmt;

/// 杠的类型 (与 action/types.rs 中的定义保持一致或共享)
/// 如果 action 模块已经定义了 KanType，可以考虑将其移到更公共的位置（如 tile 或一个新的 common 模块）
/// 或者在这里重新导出/定义。为简单起见，暂时在这里复制定义。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KanType {
    /// 暗杠 (Closed Kan)
    Closed,
    /// 加杠 (Added Kan)
    Added,
    /// 大明杠 (Open Kan)
    Open,
}

/// 副露（Meld）的类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MeldType {
    /// 吃 (Chi)
    Chi,
    /// 碰 (Pon)
    Pon,
    /// 杠 (Kan)
    Kan(KanType),
    // Sequence, // 考虑是否需要区分顺子和刻子，或者统一用 Chi/Pon
    // Triplet,
    // Kong,
}

/// 代表一个副露（吃、碰、杠）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Meld {
    /// 副露的类型
    meld_type: MeldType,
    /// 组成副露的牌，按照一定顺序排列
    /// - Chi: [吃入的牌, 手牌1, 手牌2] (例如，吃1万，手牌23万，则 [1m, 2m, 3m])
    /// - Pon: [碰的牌, 碰的牌, 碰的牌]
    /// - Kan (Open/Added): [杠的牌, 杠的牌, 杠的牌, 杠的牌]
    /// - Kan (Closed): [杠的牌, 杠的牌, 杠的牌, 杠的牌] (但来源不同)
    tiles: Vec<Tile>,
    /// 指示这张牌是从哪个玩家那里获得的（用于判断来源）
    /// 0: 自己 (暗杠)
    /// 1: 下家 (右)
    /// 2: 对家 (中)
    /// 3: 上家 (左)
    /// 对于暗杠，来源是自己 (0)
    /// 对于加杠，来源是最初碰牌的玩家
    from_player: usize, // 0: self, 1: right, 2: opposite, 3: left
    /// 对于暗杠，需要标记哪些牌是暗的 (通常是全部4张)
    /// 对于加杠，需要标记哪张是后加的 (通常是第4张)
    /// 可以用一个 `is_open` 标志，或者更复杂的表示
    is_open: bool,
}

impl Meld {
    /// 创建一个新的副露实例
    ///
    /// # Arguments
    ///
    /// * `meld_type` - 副露类型
    /// * `tiles` - 组成副露的牌 (需要按规则排序)
    /// * `from_player` - 牌的来源玩家索引
    /// * `is_open` - 副露是否明示
    ///
    /// # Returns
    ///
    /// 一个新的 `Meld` 实例
    pub fn new(meld_type: MeldType, tiles: Vec<Tile>, from_player: usize, is_open: bool) -> Self {
        // TODO: 添加验证逻辑，确保 tiles 数量和类型与 meld_type 匹配
        Meld {
            meld_type,
            tiles,
            from_player,
            is_open,
        }
    }

    /// 获取副露类型
    pub fn meld_type(&self) -> &MeldType {
        &self.meld_type
    }

    /// 获取组成副露的牌
    pub fn tiles(&self) -> &[Tile] {
        &self.tiles
    }

    /// 获取牌的来源玩家索引
    pub fn from_player(&self) -> usize {
        self.from_player
    }

    /// 副露是否是明示的
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// 检查副露是否包含指定的牌
    pub fn contains(&self, tile: &Tile) -> bool {
        self.tiles.contains(tile)
    }

    /// 获取组成该副露的特定牌（例如，碰/杠的牌，或吃的牌）
    /// 对于吃，通常是第一张牌；对于碰/杠，是任意一张牌（因为它们相同）
    pub fn get_key_tile(&self) -> Option<Tile> {
        self.tiles.first().cloned()
    }
}

// 为 Meld 实现 Display trait 以便打印
impl fmt::Display for Meld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match &self.meld_type {
            MeldType::Chi => "Chi",
            MeldType::Pon => "Pon",
            MeldType::Kan(kan_type) => match kan_type {
                KanType::Open => "OpenKan",
                KanType::Added => "AddedKan",
                KanType::Closed => "ClosedKan",
            },
        };
        let tiles_str = self.tiles.iter().map(|t| t.to_string()).collect::<Vec<_>>().join("");
        let open_str = if self.is_open { "Open" } else { "Closed" };
        // 显示来源：0=Self, 1=R, 2=O, 3=L
        let from_str = match self.from_player {
            0 => "Self",
            1 => "R",
            2 => "O",
            3 => "L",
            _ => "?",
        };
        write!(f, "[{}:{}({}) From:{}]", type_str, tiles_str, open_str, from_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::{Tile, TileSuit, WindSuit};

    #[test]
    fn test_meld_creation() {
        let chi_meld = Meld::new(
            MeldType::Chi,
            vec![
                Tile::new_numbered(TileSuit::Characters, 1),
                Tile::new_numbered(TileSuit::Characters, 2),
                Tile::new_numbered(TileSuit::Characters, 3),
            ],
            3, // 从上家吃
            true,
        );
        assert_eq!(chi_meld.meld_type(), &MeldType::Chi);
        assert_eq!(chi_meld.tiles().len(), 3);
        assert!(chi_meld.is_open());
        assert_eq!(chi_meld.from_player(), 3);

        let pon_meld = Meld::new(
            MeldType::Pon,
            vec![
                Tile::new_wind(WindSuit::East),
                Tile::new_wind(WindSuit::East),
                Tile::new_wind(WindSuit::East),
            ],
            1, // 从下家碰
            true,
        );
        assert_eq!(pon_meld.meld_type(), &MeldType::Pon);
        assert_eq!(pon_meld.tiles().len(), 3);
        assert!(pon_meld.is_open());
        assert_eq!(pon_meld.from_player(), 1);

        let closed_kan_meld = Meld::new(
            MeldType::Kan(KanType::Closed),
            vec![
                Tile::new_numbered(TileSuit::Dots, 5),
                Tile::new_numbered(TileSuit::Dots, 5),
                Tile::new_numbered(TileSuit::Dots, 5),
                Tile::new_numbered(TileSuit::Dots, 5),
            ],
            0, // 自己暗杠
            false, // 暗杠是 closed
        );
        assert_eq!(closed_kan_meld.meld_type(), &MeldType::Kan(KanType::Closed));
        assert_eq!(closed_kan_meld.tiles().len(), 4);
        assert!(!closed_kan_meld.is_open());
        assert_eq!(closed_kan_meld.from_player(), 0);
    }

    #[test]
    fn test_meld_display() {
        let chi_meld = Meld::new(
            MeldType::Chi,
            vec![
                Tile::new_numbered(TileSuit::Characters, 1),
                Tile::new_numbered(TileSuit::Characters, 2),
                Tile::new_numbered(TileSuit::Characters, 3),
            ],
            3, true,
        );
        assert_eq!(format!("{}", chi_meld), "[Chi:1m2m3m(Open) From:L]");

        let closed_kan_meld = Meld::new(
            MeldType::Kan(KanType::Closed),
            vec![
                Tile::new_numbered(TileSuit::Dots, 5),
                Tile::new_numbered(TileSuit::Dots, 5),
                Tile::new_numbered(TileSuit::Dots, 5),
                Tile::new_numbered(TileSuit::Dots, 5),
            ],
            0, false,
        );
        // 注意：暗杠的显示可能需要特殊处理，这里仅作基础显示
        assert_eq!(format!("{}", closed_kan_meld), "[ClosedKan:5p5p5p5p(Closed) From:Self]");
    }
}
