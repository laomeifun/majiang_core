// src/wall/builder.rs
// 负责根据配置生成初始牌墙

use crate::tile::{Tile, TileSuit, DragonSuit, WindSuit};
use crate::rules::RuleSet; // 假设 RuleSet 定义在 crate::rules 模块
use super::Wall; // 引用同级目录下的 mod.rs 中的 Wall
use rand::seq::SliceRandom;
use rand::thread_rng;

/// 用于构建牌墙的结构体
pub struct WallBuilder {
    rule_set: RuleSet,
    // 可以添加其他配置，例如是否包含红宝牌等
    // use_red_dora: bool,
}

impl WallBuilder {
    /// 创建一个新的 WallBuilder 实例
    pub fn new(rule_set: RuleSet) -> Self {
        WallBuilder { rule_set }
    }

    /// 构建牌墙
    ///
    /// # Returns
    ///
    /// 一个根据规则集初始化好的 `Wall` 实例
    pub fn build(&self) -> Wall {
        let mut tiles = self.generate_base_tiles();
        self.add_optional_tiles(&mut tiles);

        // 洗牌
        let mut rng = thread_rng();
        tiles.shuffle(&mut rng);

        // 根据规则确定王牌区大小和初始宝牌指示牌
        // TODO: 这部分逻辑可能需要根据具体规则细化
        let dead_wall_size = 14; // 标准日麻/国标等通常是 14 张
        let initial_dora_count = match self.rule_set {
            RuleSet::Riichi => 1, // 日麻初始1张宝牌指示牌
            RuleSet::MCR => 0,    // 国标没有宝牌概念（除非特殊规则）
            RuleSet::Shanghai => 0, // 上海麻将通常没有宝牌
            // ... 其他规则集
        };

        if tiles.len() < dead_wall_size {
            // 牌不够组成王牌区，这通常是一个错误
            panic!("Not enough tiles to form the dead wall for rule set: {:?}", self.rule_set);
        }

        // 分割牌墙和王牌区
        // 王牌区从洗好的牌的末尾取
        let dead_wall_tiles_vec = tiles.split_off(tiles.len() - dead_wall_size);

        // 从王牌区确定初始宝牌指示牌
        // 日麻：王牌从后往前数，第3墩上层是宝牌指示牌 (即索引 dead_wall_size - 5)
        //       第1,2墩是岭上牌 (索引 dead_wall_size-1 到 dead_wall_size-4)
        //       第3墩下层到第7墩是其他王牌 (索引 dead_wall_size-6 到 0)
        let mut initial_dora_indicators = Vec::new();
        if initial_dora_count > 0 && dead_wall_size >= 5 { // 确保王牌区足够大
             // 假设宝牌指示牌是王牌区的特定牌，例如日麻的第5张牌（从后往前数）
             // 注意：索引是从0开始的
             let dora_indicator_index = dead_wall_size - 5; // 第 14-5 = 9 张牌 (索引9)
             initial_dora_indicators.push(dead_wall_tiles_vec[dora_indicator_index]);
        }
        // 注意：更复杂的规则可能需要不同的宝牌确定方式

        // 创建 Wall 实例
        // tiles 此时是剩余的普通牌墙部分
        Wall::new(tiles, dead_wall_tiles_vec, initial_dora_indicators)
    }

    /// 生成基础牌组（万筒索、字牌）
    fn generate_base_tiles(&self) -> Vec<Tile> {
        let mut tiles = Vec::new();
        let suits = [TileSuit::Characters, TileSuit::Dots, TileSuit::Bamboos];
        let numbers = 1..=9;

        // 添加万筒索
        for _ in 0..4 { // 每种牌有4张
            for &suit in &suits {
                for number in numbers.clone() {
                    tiles.push(Tile::new_numbered(suit, number));
                }
            }
            // 添加字牌
            for &wind in WindSuit::VALUES.iter() {
                tiles.push(Tile::new_wind(wind));
            }
            for &dragon in DragonSuit::VALUES.iter() {
                tiles.push(Tile::new_dragon(dragon));
            }
        }
        tiles
    }

    /// 根据规则添加可选牌（如花牌、红宝牌）
    fn add_optional_tiles(&self, tiles: &mut Vec<Tile>) {
        match self.rule_set {
            RuleSet::MCR | RuleSet::Shanghai => {
                // 添加花牌 (春夏秋冬梅兰竹菊)
                for i in 1..=8 {
                    tiles.push(Tile::new_flower(i));
                }
            }
            RuleSet::Riichi => {
                // TODO: 添加红宝牌逻辑 (例如替换一张 5 万/筒/索)
                // self.add_red_dora(tiles);
            }
            // ... 其他规则集
        }
    }

    // fn add_red_dora(&self, tiles: &mut Vec<Tile>) {
    //     // 实现添加红宝牌的逻辑
    //     // 例如，找到一张普通的 5 万，替换成红 5 万
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::RuleSet;

    #[test]
    fn test_mcr_wall_build() {
        let builder = WallBuilder::new(RuleSet::MCR);
        let wall = builder.build();
        // 国标麻将 136 (基础) + 8 (花) = 144 张牌
        assert_eq!(wall.total_tiles, 144);
        assert_eq!(wall.dead_wall.len(), 14); // 王牌区 14 张
        assert_eq!(wall.remaining_draws(), 144 - 14); // 可摸 130 张
        assert!(wall.get_dora_indicators().is_empty()); // 国标无宝牌
    }

    #[test]
    fn test_riichi_wall_build() {
        let builder = WallBuilder::new(RuleSet::Riichi);
        let wall = builder.build();
        // 日麻 136 张牌 (无花，暂无红宝牌)
        assert_eq!(wall.total_tiles, 136);
        assert_eq!(wall.dead_wall.len(), 14);
        assert_eq!(wall.remaining_draws(), 136 - 14); // 可摸 122 张
        assert_eq!(wall.get_dora_indicators().len(), 1); // 日麻初始1个宝牌
    }

     #[test]
    fn test_shanghai_wall_build() {
        let builder = WallBuilder::new(RuleSet::Shanghai);
        let wall = builder.build();
        // 上海麻将 136 (基础) + 8 (花) = 144 张牌
        assert_eq!(wall.total_tiles, 144);
        assert_eq!(wall.dead_wall.len(), 14); // 假设王牌区也是 14 张
        assert_eq!(wall.remaining_draws(), 144 - 14); // 可摸 130 张
        assert!(wall.get_dora_indicators().is_empty()); // 上海麻将无宝牌
    }

    // TODO: 添加包含红宝牌的测试
}
