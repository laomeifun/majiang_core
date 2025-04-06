// src/tile/wall.rs

use crate::tile::{Tile, Wind, Dragon}; // 导入 Wind 和 Dragon 枚举
use rand::seq::SliceRandom; // 引入 shuffle 方法
use rand::thread_rng; // 引入随机数生成器

/// 代表牌墙
#[derive(Debug, Clone)] // 添加 Clone
pub struct Wall {
    tiles: Vec<Tile>,
    // 可以包含王牌指示牌、岭上牌等信息
    // dora_indicators: Vec<Tile>,
    // dead_wall_tiles: Vec<Tile>,
    current_index: usize, // 当前摸牌的位置
}

impl Wall {
    /// 创建一副新的、洗好的牌墙 (标准 136 张)
    pub fn new() -> Self {
        let mut tiles = Vec::with_capacity(136);

        // 生成万、筒、索各 4 套 (1-9)
        for _ in 0..4 { // 4 套
            for i in 1..=9 { // 1 到 9
                tiles.push(Tile::man(i).expect("Failed to create Man tile"));
                tiles.push(Tile::pin(i).expect("Failed to create Pin tile"));
                tiles.push(Tile::sou(i).expect("Failed to create Sou tile"));
            }
            // 生成风牌 (东、南、西、北) 各 4 张
            tiles.push(Tile::wind(Wind::East));
            tiles.push(Tile::wind(Wind::South));
            tiles.push(Tile::wind(Wind::West));
            tiles.push(Tile::wind(Wind::North));
            // 生成箭牌 (白、发、中) 各 4 张
            tiles.push(Tile::dragon(Dragon::White));
            tiles.push(Tile::dragon(Dragon::Green));
            tiles.push(Tile::dragon(Dragon::Red));
        }

        // 确认牌数是否正确
        assert_eq!(tiles.len(), 136, "Incorrect number of tiles generated");

        // 洗牌
        let mut rng = thread_rng();
        tiles.shuffle(&mut rng);

        Wall {
            tiles,
            current_index: 0,
            // dora_indicators: Vec::new(), // 初始化宝牌指示牌等
            // dead_wall_tiles: Vec::new(),
        }
    }

    /// 从牌墙摸一张牌
    pub fn draw_tile(&mut self) -> Option<Tile> {
        if self.current_index < self.tiles.len() {
            let tile = self.tiles[self.current_index];
            self.current_index += 1;
            Some(tile)
        } else {
            None // 牌墙摸完了
        }
    }

    /// 剩余牌数
    pub fn remaining_tiles(&self) -> usize {
        self.tiles.len() - self.current_index
    }

    // 其他逻辑，例如开杠摸岭上牌、翻宝牌指示牌等...
}

impl Default for Wall {
    fn default() -> Self {
        Self::new()
    }
}
