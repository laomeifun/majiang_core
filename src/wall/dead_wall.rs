// src/wall/dead_wall.rs
// 处理王牌区(Dora, Ura-Dora, LingShang)逻辑

use crate::tile::Tile;

/// 代表王牌区
#[derive(Debug, Clone)]
pub struct DeadWall {
    /// 王牌区的所有牌，通常是 14 张
    /// 顺序：[杠宝牌指示牌4, ..., 杠宝牌指示牌1, 里宝牌指示牌4, ..., 里宝牌指示牌1, 岭上牌4, ..., 岭上牌1]
    /// 注意：这个顺序是假设的，具体实现可能不同。日麻中宝牌和岭上牌是交错的。
    /// 更好的表示可能是分开存储：
    tiles: Vec<Tile>,
    /// 当前已翻开的宝牌指示牌 (不包括初始的一张)
    revealed_dora_indicators: Vec<Tile>,
    /// 初始宝牌指示牌
    initial_dora_indicators: Vec<Tile>,
    /// 已摸取的岭上牌数量
    replacement_tiles_drawn: usize,
    /// 王牌区总牌数
    total_size: usize,
    /// 岭上牌的总数 (通常是 4)
    replacement_tile_count: usize,
    /// 宝牌指示牌的总数 (包括初始和杠后翻开的，日麻最多 5)
    dora_indicator_count: usize,
    /// 里宝牌指示牌的总数 (日麻最多 5)
    ura_dora_indicator_count: usize,
}

// 定义王牌区各部分的索引（基于日麻规则，14张牌）
// 牌的顺序是从牌墙末尾往前数的
const LINGSHANG_START_INDEX: usize = 0; // 最后 4 张是岭上牌 (索引 0-3)
const LINGSHANG_END_INDEX: usize = 3;
const URA_DORA_START_INDEX: usize = 4; // 接下来 5 张是里宝牌指示牌 (索引 4-8)
const URA_DORA_END_INDEX: usize = 8;
const DORA_START_INDEX: usize = 9; // 最前面 5 张是宝牌指示牌 (索引 9-13)
const DORA_END_INDEX: usize = 13;
const INITIAL_DORA_INDEX: usize = 9; // 初始宝牌指示牌是索引 9 (第 14-5=9 张)

impl DeadWall {
    /// 创建一个新的 DeadWall 实例
    pub(crate) fn new(mut tiles: Vec<Tile>, initial_dora_indicators: Vec<Tile>) -> Self {
        let total_size = tiles.len();
        // 日麻规则：4张岭上牌，5张宝牌指示牌，5张里宝牌指示牌
        let replacement_tile_count = 4;
        let dora_indicator_count = 5;
        let ura_dora_indicator_count = 5;

        if total_size != replacement_tile_count + dora_indicator_count + ura_dora_indicator_count {
            // 可以选择 panic 或返回错误
            // panic!("Invalid dead wall size: {}", total_size);
            // 或者使用默认值/进行调整
            println!("Warning: Dead wall size ({}) does not match expected Riichi size (14). Adjusting counts.", total_size);
            // 这里简单处理，实际可能需要更复杂的逻辑
        }

        // 确保 tiles 顺序正确 (假设传入时已是 [杠宝指示4..1, 里宝指示4..1, 岭上4..1])
        // 如果不是，可能需要在这里调整顺序

        DeadWall {
            tiles,
            revealed_dora_indicators: Vec::new(), // 初始没有杠宝牌
            initial_dora_indicators, // 从 WallBuilder 传入
            replacement_tiles_drawn: 0,
            total_size,
            replacement_tile_count,
            dora_indicator_count,
            ura_dora_indicator_count,
        }
    }

    /// 获取王牌区的总长度
    pub fn len(&self) -> usize {
        self.total_size
    }

    /// 检查王牌区是否为空（理论上不应该）
    pub fn is_empty(&self) -> bool {
        self.total_size == 0
    }

     /// 获取已摸取的岭上牌数量
    pub fn replacement_tiles_drawn(&self) -> usize {
        self.replacement_tiles_drawn
    }

    /// 从岭上摸一张牌
    ///
    /// # Returns
    ///
    /// 如果岭上还有牌，返回 `Some(Tile)`，否则返回 `None`
    pub fn draw_replacement_tile(&mut self) -> Option<Tile> {
        if self.replacement_tiles_drawn < self.replacement_tile_count {
            // 岭上牌在 tiles 向量的末尾 (索引 0 到 replacement_tile_count - 1)
            // 每次摸走索引最小的那张
            let tile_index = self.replacement_tiles_drawn; // 摸第 0, 1, 2, 3 张
            if tile_index < self.tiles.len() { // 边界检查
                let tile = self.tiles[tile_index];
                self.replacement_tiles_drawn += 1;
                Some(tile)
            } else {
                None // 索引超出范围，理论上不应发生
            }
        } else {
            None // 岭上牌已摸完
        }
    }

    /// 开杠后，翻开一个新的（杠）宝牌指示牌
    ///
    /// # Returns
    ///
    /// 如果成功翻开，返回 `true`，否则返回 `false`
    pub fn reveal_new_dora_indicator(&mut self) -> bool {
        // 杠宝牌指示牌紧跟在初始宝牌指示牌之后
        // 初始宝牌是索引 DORA_START_INDEX (9)
        // 第一个杠宝牌是索引 DORA_START_INDEX + 1 (10)
        // 第二个是 DORA_START_INDEX + 2 (11) ...
        let current_revealed_count = self.revealed_dora_indicators.len();
        let max_revealed_dora = self.dora_indicator_count - self.initial_dora_indicators.len(); // 最多能翻开的杠宝牌数量

        if current_revealed_count < max_revealed_dora {
            let next_dora_index = DORA_START_INDEX + self.initial_dora_indicators.len() + current_revealed_count;
             if next_dora_index <= DORA_END_INDEX && next_dora_index < self.tiles.len() { // 边界检查
                self.revealed_dora_indicators.push(self.tiles[next_dora_index]);
                true
            } else {
                 false // 索引超出范围
            }
        } else {
            false // 宝牌指示牌已全部翻开
        }
    }

    /// 获取当前所有可见的宝牌指示牌（初始 + 杠）
    ///
    /// # Returns
    ///
    /// 一个包含所有宝牌指示牌的向量
    pub fn get_dora_indicators(&self) -> Vec<Tile> {
        let mut indicators = self.initial_dora_indicators.clone();
        indicators.extend_from_slice(&self.revealed_dora_indicators);
        indicators
    }

    /// 获取当前所有里宝牌指示牌（仅在立直和牌时使用）
    ///
    /// # Returns
    ///
    /// 一个包含所有里宝牌指示牌的向量
    pub fn get_ura_dora_indicators(&self) -> Vec<Tile> {
        // 里宝牌指示牌在宝牌指示牌“下方”，即索引 URA_DORA_START_INDEX 到 URA_DORA_END_INDEX
        let mut indicators = Vec::new();
        let count = self.initial_dora_indicators.len() + self.revealed_dora_indicators.len(); // 当前可见的宝牌指示牌数量

        // 有多少张表宝牌指示牌，就对应多少张里宝牌指示牌
        for i in 0..count {
            let ura_dora_index = URA_DORA_START_INDEX + i;
            if ura_dora_index <= URA_DORA_END_INDEX && ura_dora_index < self.tiles.len() { // 边界检查
                 indicators.push(self.tiles[ura_dora_index]);
            } else {
                break; // 超出范围则停止
            }
        }
        indicators
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::{Tile, TileSuit, WindSuit};

    // 创建一个简单的包含14张牌的王牌区用于测试
    fn create_test_dead_wall_tiles() -> (Vec<Tile>, Vec<Tile>) {
        // 顺序: [杠宝4..1, 里宝4..1, 岭上4..1]
        let mut tiles = Vec::new();
        // 岭上牌 (4张) - 东东南南
        tiles.push(Tile::new_wind(WindSuit::East));
        tiles.push(Tile::new_wind(WindSuit::East));
        tiles.push(Tile::new_wind(WindSuit::South));
        tiles.push(Tile::new_wind(WindSuit::South));
        // 里宝牌 (5张) - 西西北北中
        tiles.push(Tile::new_wind(WindSuit::West));
        tiles.push(Tile::new_wind(WindSuit::West));
        tiles.push(Tile::new_wind(WindSuit::North));
        tiles.push(Tile::new_wind(WindSuit::North));
        tiles.push(Tile::new_dragon(crate::tile::DragonSuit::Red)); // 中
        // 宝牌 (5张) - 发发白白 1万
        tiles.push(Tile::new_dragon(crate::tile::DragonSuit::Green)); // 发
        tiles.push(Tile::new_dragon(crate::tile::DragonSuit::Green)); // 发
        tiles.push(Tile::new_dragon(crate::tile::DragonSuit::White)); // 白
        tiles.push(Tile::new_dragon(crate::tile::DragonSuit::White)); // 白
        tiles.push(Tile::new_numbered(TileSuit::Characters, 1)); // 1万

        // 翻转以匹配内部存储顺序（假设内部是从0开始，对应牌墙末尾）
        // tiles.reverse();
        // 修正：根据上面的常量定义，索引0是岭上牌1，索引13是杠宝牌指示牌4
        // 所以不需要翻转，上面的顺序就是 [岭上1..4, 里宝1..5, 宝牌1..5]

        let initial_dora = vec![tiles[INITIAL_DORA_INDEX]]; // 初始宝牌是索引9 (发)
        (tiles, initial_dora)
    }

    #[test]
    fn test_dead_wall_initialization() {
        let (tiles, initial_dora) = create_test_dead_wall_tiles();
        let dead_wall = DeadWall::new(tiles.clone(), initial_dora.clone());

        assert_eq!(dead_wall.len(), 14);
        assert_eq!(dead_wall.replacement_tiles_drawn(), 0);
        assert_eq!(dead_wall.get_dora_indicators().len(), 1);
        assert_eq!(dead_wall.get_dora_indicators()[0], Tile::new_dragon(crate::tile::DragonSuit::Green)); // 发
        assert!(dead_wall.revealed_dora_indicators.is_empty());
        assert_eq!(dead_wall.get_ura_dora_indicators().len(), 1); // 初始对应1个里宝
        assert_eq!(dead_wall.get_ura_dora_indicators()[0], tiles[URA_DORA_START_INDEX]); // 西
    }

    #[test]
    fn test_draw_replacement_tile() {
        let (tiles, initial_dora) = create_test_dead_wall_tiles();
        let mut dead_wall = DeadWall::new(tiles.clone(), initial_dora);

        assert_eq!(dead_wall.draw_replacement_tile(), Some(tiles[0])); // 东
        assert_eq!(dead_wall.replacement_tiles_drawn(), 1);
        assert_eq!(dead_wall.draw_replacement_tile(), Some(tiles[1])); // 东
        assert_eq!(dead_wall.replacement_tiles_drawn(), 2);
        assert_eq!(dead_wall.draw_replacement_tile(), Some(tiles[2])); // 南
        assert_eq!(dead_wall.replacement_tiles_drawn(), 3);
        assert_eq!(dead_wall.draw_replacement_tile(), Some(tiles[3])); // 南
        assert_eq!(dead_wall.replacement_tiles_drawn(), 4);
        assert_eq!(dead_wall.draw_replacement_tile(), None); // 摸完了
        assert_eq!(dead_wall.replacement_tiles_drawn(), 4);
    }

    #[test]
    fn test_reveal_dora_and_ura_dora() {
        let (tiles, initial_dora) = create_test_dead_wall_tiles();
        let mut dead_wall = DeadWall::new(tiles.clone(), initial_dora.clone());

        // 初始状态
        assert_eq!(dead_wall.get_dora_indicators().len(), 1); // 发
        assert_eq!(dead_wall.get_ura_dora_indicators().len(), 1); // 西

        // 第一次杠
        assert!(dead_wall.reveal_new_dora_indicator());
        assert_eq!(dead_wall.get_dora_indicators().len(), 2); // 发, 发
        assert_eq!(dead_wall.get_dora_indicators()[1], tiles[DORA_START_INDEX + 1]); // 第二张宝牌指示牌 (索引10)
        assert_eq!(dead_wall.get_ura_dora_indicators().len(), 2); // 西, 西
        assert_eq!(dead_wall.get_ura_dora_indicators()[1], tiles[URA_DORA_START_INDEX + 1]); // 第二张里宝指示牌 (索引5)

        // 第二次杠
        assert!(dead_wall.reveal_new_dora_indicator());
        assert_eq!(dead_wall.get_dora_indicators().len(), 3); // 发, 发, 白
        assert_eq!(dead_wall.get_ura_dora_indicators().len(), 3); // 西, 西, 北

        // 第三次杠
        assert!(dead_wall.reveal_new_dora_indicator());
        assert_eq!(dead_wall.get_dora_indicators().len(), 4); // 发, 发, 白, 白
        assert_eq!(dead_wall.get_ura_dora_indicators().len(), 4); // 西, 西, 北, 北

        // 第四次杠
        assert!(dead_wall.reveal_new_dora_indicator());
        assert_eq!(dead_wall.get_dora_indicators().len(), 5); // 发, 发, 白, 白, 1万
        assert_eq!(dead_wall.get_ura_dora_indicators().len(), 5); // 西, 西, 北, 北, 中

        // 尝试第五次杠
        assert!(!dead_wall.reveal_new_dora_indicator()); // 不能再翻了
        assert_eq!(dead_wall.get_dora_indicators().len(), 5);
        assert_eq!(dead_wall.get_ura_dora_indicators().len(), 5);
    }
}
