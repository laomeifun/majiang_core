// 岭上牌区模块
// 处理麻将中的岭上牌、宝牌指示牌和杠后补牌等特殊牌区

use crate::tile::Tile;
use crate::errors::{MajiangError, Result};

/// 岭上牌区的配置选项
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeadWallConfig {
    /// 日本麻将: 14张牌，前4张用作宝牌指示牌
    Riichi {
        /// 里宝牌数量，通常为1
        dora_indicators: u8,
        /// 里宝牌数量，通常为1
        uradora_indicators: u8
    },
    
    /// 中国官方规则(MCR): 16张牌，前8张为补花区
    MCR {
        /// 花牌补充区大小
        replacement_count: u8
    },
    
    /// 上海麻将: 与MCR相似但可能有不同数量
    Shanghai {
        /// 花牌补充区大小
        replacement_count: u8
    },
    
    /// 无岭上牌区，如某些简化规则
    None,
}

impl Default for DeadWallConfig {
    fn default() -> Self {
        DeadWallConfig::MCR { replacement_count: 8 }
    }
}

/// 表示麻将牌墙末尾的特殊区域，包含岭上牌、宝牌指示牌等
/// 在不同的麻将规则中有不同的实现
#[derive(Debug, Clone)]
pub struct DeadWall {
    /// 岭上牌区的所有牌
    tiles: Vec<Tile>,
    
    /// 岭上牌区配置
    config: DeadWallConfig,
    
    /// 当前翻开的宝牌指示牌索引
    revealed_indicator_indices: Vec<usize>,
}

impl DeadWall {
    /// 从牌堆末尾创建岭上牌区
    /// 
    /// # 参数
    /// * `tiles` - 牌墙末尾的牌，将从中取出适量牌形成岭上牌区
    /// * `config` - 岭上牌区配置
    /// 
    /// # 返回值
    /// * `Result<Self>` - 成功则返回创建的岭上牌区，失败则返回错误
    pub fn new(tiles: &mut Vec<Tile>, config: DeadWallConfig) -> Result<Self> {
        match config {
            DeadWallConfig::None => Ok(Self {
                tiles: Vec::new(),
                config,
                revealed_indicator_indices: Vec::new(),
            }),
            
            DeadWallConfig::Riichi { dora_indicators, uradora_indicators } => {
                let dead_wall_size = 14;  // 日麻标准岭上牌数量
                
                if tiles.len() < dead_wall_size {
                    return Err(MajiangError::NotEnoughTiles);
                }
                
                // 从牌墙末尾取出牌形成岭上牌区
                let mut dead_wall_tiles = Vec::with_capacity(dead_wall_size);
                for _ in 0..dead_wall_size {
                    if let Some(tile) = tiles.pop() {
                        dead_wall_tiles.push(tile);
                    } else {
                        return Err(MajiangError::NotEnoughTiles);
                    }
                }
                
                // 初始宝牌指示牌是第5张(索引4)
                let mut revealed = Vec::with_capacity((dora_indicators + uradora_indicators) as usize);
                // 初始只有表宝牌
                for i in 0..dora_indicators as usize {
                    revealed.push(i * 2);  // 表宝牌在岭上牌的偶数位置
                }
                
                Ok(Self {
                    tiles: dead_wall_tiles,
                    config,
                    revealed_indicator_indices: revealed,
                })
            },
            
            DeadWallConfig::MCR { replacement_count } | DeadWallConfig::Shanghai { replacement_count } => {
                let dead_wall_size = 16;  // 中式麻将标准岭上牌数量
                
                if tiles.len() < dead_wall_size {
                    return Err(MajiangError::NotEnoughTiles);
                }
                
                // 从牌墙末尾取出牌形成岭上牌区
                let mut dead_wall_tiles = Vec::with_capacity(dead_wall_size);
                for _ in 0..dead_wall_size {
                    if let Some(tile) = tiles.pop() {
                        dead_wall_tiles.push(tile);
                    } else {
                        return Err(MajiangError::NotEnoughTiles);
                    }
                }
                
                Ok(Self {
                    tiles: dead_wall_tiles,
                    config,
                    revealed_indicator_indices: Vec::new(), // 中式麻将没有宝牌指示牌
                })
            }
        }
    }
    
    /// 获取剩余可用的岭上牌数量
    pub fn remaining_replacement_tiles(&self) -> usize {
        match self.config {
            DeadWallConfig::None => 0,
            DeadWallConfig::Riichi { .. } => {
                // 日麻的岭上牌区中，除了宝牌指示牌外的牌都可用作杠后补牌
                self.tiles.len() - self.revealed_indicator_indices.len()
            },
            DeadWallConfig::MCR { replacement_count } | DeadWallConfig::Shanghai { replacement_count } => {
                // 中式麻将的补牌区通常位于岭上牌的前半部分
                replacement_count as usize
            }
        }
    }
    
    /// 从岭上牌区取一张补牌(如杠后补牌或补花)
    /// 
    /// # 返回值
    /// * `Result<Tile>` - 成功则返回取出的牌，失败则返回错误
    pub fn draw_replacement_tile(&mut self) -> Result<Tile> {
        match self.config {
            DeadWallConfig::None => {
                Err(MajiangError::InvalidOperation("没有配置岭上牌区".to_string()))
            },
            
            DeadWallConfig::Riichi { .. } => {
                // 日麻从岭上牌区末尾取牌(倒数第一张)
                if self.tiles.is_empty() {
                    return Err(MajiangError::NotEnoughTiles);
                }
                
                Ok(self.tiles.remove(self.tiles.len() - 1))
            },
            
            DeadWallConfig::MCR { replacement_count } | DeadWallConfig::Shanghai { replacement_count } => {
                // 中式麻将从补牌区取牌(通常是前8张)
                let replacement_index = replacement_count as usize - 1;
                
                if replacement_index >= self.tiles.len() {
                    return Err(MajiangError::NotEnoughTiles);
                }
                
                Ok(self.tiles.remove(replacement_index))
            }
        }
    }
    
    /// 获取当前的宝牌指示牌(仅适用于日麻)
    /// 
    /// # 返回值
    /// * `Result<Vec<&Tile>>` - 成功则返回宝牌指示牌列表，失败则返回错误
    pub fn get_dora_indicators(&self) -> Result<Vec<&Tile>> {
        match self.config {
            DeadWallConfig::Riichi { dora_indicators, .. } => {
                let mut indicators = Vec::with_capacity(dora_indicators as usize);
                
                for &index in &self.revealed_indicator_indices {
                    if index < self.tiles.len() {
                        indicators.push(&self.tiles[index]);
                    } else {
                        return Err(MajiangError::InvalidOperation("宝牌指示牌索引越界".to_string()));
                    }
                }
                
                Ok(indicators)
            },
            _ => Err(MajiangError::InvalidOperation("该规则没有宝牌指示牌".to_string()))
        }
    }
    
    /// 获取里宝牌指示牌(仅适用于日麻，通常在游戏结束时才会公开)
    /// 
    /// # 返回值
    /// * `Result<Vec<&Tile>>` - 成功则返回里宝牌指示牌列表，失败则返回错误
    pub fn get_uradora_indicators(&self) -> Result<Vec<&Tile>> {
        match self.config {
            DeadWallConfig::Riichi { dora_indicators, uradora_indicators } => {
                let mut indicators = Vec::with_capacity(uradora_indicators as usize);
                
                // 里宝牌指示牌位于表宝牌指示牌的下一张
                for i in 0..uradora_indicators as usize {
                    if i < dora_indicators as usize && (i * 2 + 1) < self.tiles.len() {
                        indicators.push(&self.tiles[i * 2 + 1]);
                    } else {
                        return Err(MajiangError::InvalidOperation("里宝牌指示牌索引越界".to_string()));
                    }
                }
                
                Ok(indicators)
            },
            _ => Err(MajiangError::InvalidOperation("该规则没有里宝牌指示牌".to_string()))
        }
    }
    
    /// 在杠牌后翻开新的宝牌指示牌(仅适用于日麻)
    /// 
    /// # 返回值
    /// * `Result<&Tile>` - 成功则返回新翻开的宝牌指示牌，失败则返回错误
    pub fn reveal_next_dora_indicator(&mut self) -> Result<&Tile> {
        match self.config {
            DeadWallConfig::Riichi { dora_indicators, .. } => {
                let current_indicators = self.revealed_indicator_indices.len();
                
                if current_indicators >= dora_indicators as usize {
                    return Err(MajiangError::InvalidOperation("已经翻开了所有宝牌指示牌".to_string()));
                }
                
                // 计算下一个宝牌指示牌的索引
                let next_index = current_indicators * 2;
                if next_index >= self.tiles.len() {
                    return Err(MajiangError::InvalidOperation("宝牌指示牌索引越界".to_string()));
                }
                
                self.revealed_indicator_indices.push(next_index);
                Ok(&self.tiles[next_index])
            },
            _ => Err(MajiangError::InvalidOperation("该规则没有宝牌指示牌".to_string()))
        }
    }
    
    /// 获取所有岭上牌区的牌(用于调试或测试)
    pub fn get_all_tiles(&self) -> &[Tile] {
        &self.tiles
    }
}