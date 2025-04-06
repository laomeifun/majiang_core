// src/wall/mod.rs
// 导出 Wall 相关内容, 定义 Wall 结构体主体

use crate::tile::Tile;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub mod builder;
pub mod dead_wall;

// 重新导出，方便外部使用
pub use builder::WallBuilder;
pub use dead_wall::DeadWall;

/// 代表麻将牌墙
#[derive(Debug, Clone)]
pub struct Wall {
    /// 牌墙中的所有牌，从牌墙尾部开始排列
    tiles: Vec<Tile>,
    /// 王牌区（杠宝牌、岭上牌等）
    dead_wall: DeadWall,
    /// 当前可摸牌的索引（从牌墙尾部开始计数）
    current_index: usize,
    /// 牌墙的总牌数
    total_tiles: usize,
}

impl Wall {
    /// 创建一个新的牌墙实例（通常通过 WallBuilder 创建）
    ///
    /// # Arguments
    ///
    /// * `tiles` - 排序好的牌墙牌组
    /// * `dead_wall_tiles` - 王牌区的牌
    /// * `dora_indicators` - 宝牌指示牌
    ///
    /// # Returns
    ///
    /// 一个新的 `Wall` 实例
    pub(crate) fn new(tiles: Vec<Tile>, dead_wall_tiles: Vec<Tile>, dora_indicators: Vec<Tile>) -> Self {
        let total_tiles = tiles.len();
        // 初始摸牌索引指向牌墙末尾第一张牌
        // 因为 tiles 是从尾部开始排列的，所以索引 0 是最后一张牌
        let current_index = 0;
        Wall {
            tiles,
            dead_wall: DeadWall::new(dead_wall_tiles, dora_indicators),
            current_index,
            total_tiles,
        }
    }

    /// 从牌墙摸一张牌
    ///
    /// # Returns
    ///
    /// 如果牌墙还有牌，返回 `Some(Tile)`，否则返回 `None`
    pub fn draw(&mut self) -> Option<Tile> {
        if self.can_draw() {
            let tile = self.tiles[self.current_index];
            self.current_index += 1;
            Some(tile)
        } else {
            None
        }
    }

    /// 检查牌墙是否还有可摸的牌（不包括王牌区）
    ///
    /// # Returns
    ///
    /// 如果还能摸牌，返回 `true`，否则返回 `false`
    pub fn can_draw(&self) -> bool {
        // 可摸牌的数量 = 总牌数 - 王牌区固定牌数 - 当前已摸牌数
        // 王牌区固定为 14 张
        self.total_tiles - self.dead_wall.len() > self.current_index
    }

    /// 获取剩余可摸牌的数量
    ///
    /// # Returns
    ///
    /// 剩余可摸牌的数量
    pub fn remaining_draws(&self) -> usize {
        if self.can_draw() {
            self.total_tiles - self.dead_wall.len() - self.current_index
        } else {
            0
        }
    }

    /// 从岭上摸一张牌
    ///
    /// # Returns
    ///
    /// 如果岭上还有牌，返回 `Some(Tile)`，否则返回 `None`
    pub fn draw_from_dead_wall(&mut self) -> Option<Tile> {
        self.dead_wall.draw_replacement_tile()
    }

    /// 开杠后，翻开一个新的宝牌指示牌
    ///
    /// # Returns
    ///
    /// 如果成功翻开，返回 `true`，否则返回 `false`
    pub fn reveal_new_dora_indicator(&mut self) -> bool {
        self.dead_wall.reveal_new_dora_indicator()
    }

    /// 获取当前所有宝牌指示牌
    ///
    /// # Returns
    ///
    /// 一个包含当前所有宝牌指示牌的向量
    pub fn get_dora_indicators(&self) -> Vec<Tile> {
        self.dead_wall.get_dora_indicators()
    }

    /// 获取当前所有里宝牌指示牌（仅在立直和牌时使用）
    ///
    /// # Returns
    ///
    /// 一个包含当前所有里宝牌指示牌的向量
    pub fn get_ura_dora_indicators(&self) -> Vec<Tile> {
        self.dead_wall.get_ura_dora_indicators()
    }

    /// 获取王牌区信息（主要用于调试或特殊规则）
    pub fn get_dead_wall(&self) -> &DeadWall {
        &self.dead_wall
    }

    // TODO: 可能需要添加其他与 Wall 交互的方法
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::RuleSet; // 假设有一个 RuleSet 枚举

    #[test]
    fn test_wall_creation_and_draw() {
        // 使用 MCR 规则集创建牌墙
        let mut wall = WallBuilder::new(RuleSet::MCR).build(); // 假设 RuleSet::MCR

        let initial_draws = wall.remaining_draws();
        let total_tiles = wall.total_tiles;
        let dead_wall_len = wall.dead_wall.len();

        // MCR 通常 136 张牌，王牌区 14 张，可摸 136 - 14 = 122 张
        // 这里假设 WallBuilder 正确设置了牌数
        assert!(total_tiles > dead_wall_len);
        assert_eq!(initial_draws, total_tiles - dead_wall_len);

        // 摸一张牌
        let tile = wall.draw();
        assert!(tile.is_some());
        assert_eq!(wall.remaining_draws(), initial_draws - 1);
        assert_eq!(wall.current_index, 1);

        // 摸光所有牌
        for _ in 0..wall.remaining_draws() {
            assert!(wall.draw().is_some());
        }

        assert_eq!(wall.remaining_draws(), 0);
        assert!(wall.draw().is_none()); // 无法再摸牌
        assert_eq!(wall.current_index, total_tiles - dead_wall_len); // 索引停在最后一张可摸牌之后
    }

    #[test]
    fn test_dead_wall_draw_and_dora() {
        let mut wall = WallBuilder::new(RuleSet::Riichi).build(); // 假设 RuleSet::Riichi

        let initial_dora = wall.get_dora_indicators();
        assert_eq!(initial_dora.len(), 1); // 日麻初始一个宝牌

        // 模拟从岭上摸牌
        let replacement_tile = wall.draw_from_dead_wall();
        assert!(replacement_tile.is_some());
        assert_eq!(wall.dead_wall.replacement_tiles_drawn(), 1);

        // 模拟开杠，翻开新宝牌
        let revealed = wall.reveal_new_dora_indicator();
        assert!(revealed);
        assert_eq!(wall.get_dora_indicators().len(), 2); // 增加了一个宝牌

        // 模拟四次杠
        for _ in 0..3 {
            assert!(wall.draw_from_dead_wall().is_some());
            assert!(wall.reveal_new_dora_indicator());
        }

        assert_eq!(wall.get_dora_indicators().len(), 5); // 最多5个宝牌指示牌
        assert_eq!(wall.dead_wall.replacement_tiles_drawn(), 4); // 摸了4张岭上牌

        // 尝试再摸岭上牌或翻宝牌
        assert!(wall.draw_from_dead_wall().is_none()); // 岭上牌摸完
        assert!(!wall.reveal_new_dora_indicator()); // 无法再翻宝牌
    }
}
