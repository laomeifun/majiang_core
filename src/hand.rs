// src/hand.rs

use crate::tile::Tile;
use crate::meld::Meld;
use std::collections::HashMap;

/// 代表玩家的手牌（包括暗牌和副露）
#[derive(Debug, Clone)]
pub struct Hand {
    /// 手中的暗牌 (通常是 13 张，和牌时 14 张)
    /// 使用 HashMap<Tile, u8> 来存储牌及其数量可能更高效
    closed_tiles: Vec<Tile>,
    /// 已经公开的副露
    melds: Vec<Meld>,
    /// 摸到的牌 (如果当前轮到该玩家)
    drawn_tile: Option<Tile>,
}

impl Hand {
    /// 创建一个空手牌
    pub fn new() -> Self {
        Hand {
            closed_tiles: Vec::new(),
            melds: Vec::new(),
            drawn_tile: None,
        }
    }

    /// 添加一张暗牌
    pub fn add_closed_tile(&mut self, tile: Tile) {
        self.closed_tiles.push(tile);
        // 最好保持排序，便于分析
        self.closed_tiles.sort();
    }

    /// 移除一张暗牌
    pub fn remove_closed_tile(&mut self, tile: &Tile) -> bool {
        if let Some(pos) = self.closed_tiles.iter().position(|&t| t == *tile) {
            self.closed_tiles.remove(pos);
            true
        } else {
            false
        }
    }

    /// 添加一个副露
    pub fn add_meld(&mut self, meld: Meld) {
        self.melds.push(meld);
    }

    /// 设置摸到的牌
    pub fn set_drawn_tile(&mut self, tile: Option<Tile>) {
        self.drawn_tile = tile;
    }

    /// 检查摸到的牌是否是指定的牌
    pub fn is_drawn_tile(&self, tile: &Tile) -> bool {
        self.drawn_tile == Some(*tile)
    }

    /// 检查暗牌中是否包含指定的牌
    pub fn contains_closed_tile(&self, tile: &Tile) -> bool {
        self.closed_tiles.contains(tile)
    }

    /// 获取暗牌的引用
    pub fn get_closed_tiles(&self) -> &Vec<Tile> {
        &self.closed_tiles
    }

    /// 获取副露的引用
    pub fn get_melds(&self) -> &Vec<Meld> {
        &self.melds
    }

     /// 获取摸到的牌 (Option)
    pub fn get_drawn_tile(&self) -> Option<Tile> {
        self.drawn_tile
    }

    /// 获取所有牌（暗牌 + 副露牌 + 摸到的牌）
    pub fn all_tiles(&self) -> Vec<Tile> {
        let mut all = self.closed_tiles.clone();
        for meld in &self.melds {
            all.extend(meld.tiles());
        }
        if let Some(drawn) = self.drawn_tile {
            all.push(drawn);
        }
        all.sort();
        all
    }

    // 其他手牌分析逻辑，例如计算向听数、判断是否听牌等...
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}
