// src/hand/representation.rs
// Hand 结构体定义及基础操作 (添加/删除牌)

use crate::tile::Tile;
use crate::meld::Meld; // 假设 Meld 定义在 crate::meld 模块
use std::collections::HashMap; // 或者使用固定大小的数组/Vec

/// 代表玩家的手牌，包括暗牌、明牌（副露）和摸到的牌
#[derive(Debug, Clone, Default)]
pub struct Hand {
    /// 手中的暗牌 (closed tiles)
    /// 使用 HashMap<Tile, usize> 来存储每种牌的数量，方便查找和计数
    /// 或者可以使用 Vec<Tile> 并保持排序
    closed_tiles: HashMap<Tile, usize>,
    /// 已经公开的副露 (open melds)
    melds: Vec<Meld>,
    /// 当前摸到的牌 (drawn tile) - 如果有的话
    drawn_tile: Option<Tile>,
    /// (可选) 花牌
    flowers: Vec<Tile>,
    // (可选) 是否立直
    // is_riichi: bool,
}

impl Hand {
    /// 创建一个空的手牌实例
    pub fn new() -> Self {
        Hand::default()
    }

    /// 向手牌中添加一张暗牌
    pub fn add_tile(&mut self, tile: Tile) {
        *self.closed_tiles.entry(tile).or_insert(0) += 1;
        // 如果使用 Vec<Tile>，则需要插入并排序
        // self.closed_tiles.push(tile);
        // self.closed_tiles.sort_unstable();
    }

    /// 从手牌中移除一张暗牌
    ///
    /// # Returns
    ///
    /// 如果成功移除，返回 `true`，否则返回 `false` (例如牌不存在)
    pub fn remove_tile(&mut self, tile: &Tile) -> bool {
        if let Some(count) = self.closed_tiles.get_mut(tile) {
            if *count > 0 {
                *count -= 1;
                if *count == 0 {
                    self.closed_tiles.remove(tile);
                }
                // 如果使用 Vec<Tile>，则需要查找并移除
                // if let Some(pos) = self.closed_tiles.iter().position(|&t| t == *tile) {
                //     self.closed_tiles.remove(pos);
                //     return true;
                // }
                return true;
            }
        }
        false
    }

    /// 设置当前摸到的牌
    pub fn set_drawn_tile(&mut self, tile: Option<Tile>) {
        self.drawn_tile = tile;
    }

    /// 获取当前摸到的牌
    pub fn get_drawn_tile(&self) -> Option<Tile> {
        self.drawn_tile
    }

    /// 添加一个副露
    pub fn add_meld(&mut self, meld: Meld) {
        // 副露的牌需要从暗牌中移除
        for tile in meld.tiles() {
            // 对于暗杠和加杠，需要特殊处理牌的来源
            if meld.meld_type() == &crate::meld::MeldType::Kan(crate::meld::KanType::Closed) {
                 // 暗杠的四张牌都来自手牌
                 if !self.remove_tile(tile) {
                     // 理论上不应发生，表示手牌状态错误
                     eprintln!("Error: Failed to remove tile {:?} for closed kan", tile);
                 }
            } else if meld.meld_type() == &crate::meld::MeldType::Kan(crate::meld::KanType::Added) {
                 // 加杠时，只有一张牌来自手牌（或摸牌），其他三张已在之前的碰副露中
                 // 假设加杠的牌是最后一张
                 if tile == meld.tiles().last().unwrap() {
                     if !self.remove_tile(tile) {
                         // 可能是从摸牌加杠
                         if self.drawn_tile == Some(*tile) {
                             self.drawn_tile = None;
                         } else {
                             eprintln!("Error: Failed to remove tile {:?} for added kan", tile);
                         }
                     }
                 }
            } else if meld.is_open() {
                // 对于明吃、明碰、明杠，只有部分牌来自手牌
                // 假设被吃/碰/杠的牌是第一张，它不来自手牌
                if tile != meld.tiles().first().unwrap() || meld.meld_type() == &crate::meld::MeldType::Kan(crate::meld::KanType::Open) {
                    // 对于大明杠，所有三张手牌都要移除
                    if !self.remove_tile(tile) {
                         eprintln!("Error: Failed to remove tile {:?} for open meld", tile);
                    }
                }
            }
        }
        self.melds.push(meld);
    }

    /// 获取所有暗牌的列表（展开 HashMap）
    pub fn get_closed_tiles(&self) -> Vec<Tile> {
        let mut tiles = Vec::new();
        for (tile, &count) in &self.closed_tiles {
            for _ in 0..count {
                tiles.push(*tile);
            }
        }
        tiles.sort_unstable(); // 保持排序
        tiles
    }

    /// 获取所有副露
    pub fn get_melds(&self) -> &[Meld] {
        &self.melds
    }

     /// 获取手牌总张数（暗牌 + 副露牌数 + 摸牌）
    pub fn tile_count(&self) -> usize {
        let closed_count = self.closed_tiles.values().sum::<usize>();
        let meld_count = self.melds.iter().map(|m| m.tiles().len()).sum::<usize>();
        let drawn_count = if self.drawn_tile.is_some() { 1 } else { 0 };
        closed_count + meld_count + drawn_count
    }

    /// 检查手牌是否是门清（没有明示的副露）
    pub fn is_menzen(&self) -> bool {
        self.melds.iter().all(|meld| !meld.is_open())
    }

    // TODO: 添加其他基础操作，如：
    // - is_valid() 检查手牌张数是否合法
    // - clone()
    // - display() / to_string()
}

// 为 Hand 实现 Display trait
use std::fmt;

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let closed_str = self.get_closed_tiles().iter().map(|t| t.to_string()).collect::<Vec<_>>().join(" ");
        let melds_str = self.melds.iter().map(|m| m.to_string()).collect::<Vec<_>>().join(" ");
        let drawn_str = self.drawn_tile.map_or("".to_string(), |t| format!(" Drawn: {}", t));
        // let flower_str = ...

        write!(f, "Closed: [{}] Melds: [{}] {}", closed_str, melds_str, drawn_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::{Tile, TileSuit, WindSuit};
    use crate::meld::{Meld, MeldType, KanType};

    #[test]
    fn test_hand_add_remove() {
        let mut hand = Hand::new();
        let tile1 = Tile::new_numbered(TileSuit::Characters, 1);
        let tile2 = Tile::new_wind(WindSuit::East);

        hand.add_tile(tile1);
        hand.add_tile(tile1);
        hand.add_tile(tile2);

        assert_eq!(hand.get_closed_tiles().len(), 3);
        assert_eq!(hand.closed_tiles.get(&tile1), Some(&2));
        assert_eq!(hand.closed_tiles.get(&tile2), Some(&1));

        assert!(hand.remove_tile(&tile1));
        assert_eq!(hand.get_closed_tiles().len(), 2);
        assert_eq!(hand.closed_tiles.get(&tile1), Some(&1));

        assert!(hand.remove_tile(&tile1));
        assert_eq!(hand.get_closed_tiles().len(), 1);
        assert_eq!(hand.closed_tiles.get(&tile1), None); // 数量为0后移除

        assert!(!hand.remove_tile(&tile1)); // 无法再次移除

        assert!(hand.remove_tile(&tile2));
        assert!(hand.get_closed_tiles().is_empty());
        assert!(hand.closed_tiles.is_empty());
    }

    #[test]
    fn test_hand_add_meld() {
        let mut hand = Hand::new();
        let t1m = Tile::new_numbered(TileSuit::Characters, 1);
        let t2m = Tile::new_numbered(TileSuit::Characters, 2);
        let t3m = Tile::new_numbered(TileSuit::Characters, 3);
        let t4m = Tile::new_numbered(TileSuit::Characters, 4);

        hand.add_tile(t2m);
        hand.add_tile(t3m);
        hand.add_tile(t4m);
        hand.add_tile(t4m);
        hand.add_tile(t4m);

        // 吃 1m (来自上家 3)
        let chi_meld = Meld::new(MeldType::Chi, vec![t1m, t2m, t3m], 3, true);
        hand.add_meld(chi_meld.clone());

        assert_eq!(hand.get_melds().len(), 1);
        assert_eq!(hand.get_melds()[0], chi_meld);
        // 检查手牌是否移除了 2m, 3m
        assert_eq!(hand.closed_tiles.get(&t2m), None);
        assert_eq!(hand.closed_tiles.get(&t3m), None);
        // 检查 4m 还在
        assert_eq!(hand.closed_tiles.get(&t4m), Some(&3));
        assert_eq!(hand.get_closed_tiles().len(), 3); // 剩下 3 个 4m

        // 碰 4m (来自对家 2)
        let pon_meld = Meld::new(MeldType::Pon, vec![t4m, t4m, t4m], 2, true);
        hand.add_meld(pon_meld.clone());

        assert_eq!(hand.get_melds().len(), 2);
        assert_eq!(hand.get_melds()[1], pon_meld);
        // 检查手牌是否移除了 2 个 4m (碰需要移除2张手牌)
        assert_eq!(hand.closed_tiles.get(&t4m), Some(&1));
        assert_eq!(hand.get_closed_tiles().len(), 1); // 剩下 1 个 4m
    }

     #[test]
    fn test_hand_display() {
        let mut hand = Hand::new();
        let t1m = Tile::new_numbered(TileSuit::Characters, 1);
        let t2m = Tile::new_numbered(TileSuit::Characters, 2);
        let t3m = Tile::new_numbered(TileSuit::Characters, 3);
        let t4m = Tile::new_numbered(TileSuit::Characters, 4);
        let east = Tile::new_wind(WindSuit::East);

        hand.add_tile(t1m);
        hand.add_tile(t2m);
        hand.add_tile(east);
        hand.add_tile(east);
        hand.set_drawn_tile(Some(t3m));

        let pon_meld = Meld::new(MeldType::Pon, vec![t4m, t4m, t4m], 1, true);
        hand.add_meld(pon_meld); // 假设碰之前手牌有 4m 4m

        // 预期输出顺序可能因 HashMap 迭代顺序而变，但牌应该都在
        // Closed: [1m 2m 东 东] Melds: [[Pon:4m4m4m(Open) From:R]] Drawn: 3m
        // 手动排序以确保一致性
        let expected_closed = "1m 2m 东 东";
        let expected_melds = "[Pon:4m4m4m(Open) From:R]";
        let expected_drawn = " Drawn: 3m";

        let display_str = format!("{}", hand);
        assert!(display_str.contains(expected_melds));
        assert!(display_str.contains(expected_drawn));
        // 检查 closed 部分，忽略顺序
        assert!(display_str.contains("1m"));
        assert!(display_str.contains("2m"));
        assert!(display_str.contains("东")); // 应该有两个东
        assert_eq!(display_str.matches("东").count(), 2);
    }
}
