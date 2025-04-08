// 麻将牌墙模块
// 提供牌墙的基本功能，包括初始化、洗牌、发牌和摸牌

use std::collections::VecDeque;
use crate::tile::Tile;
use crate::errors::{MajiangError, MajiangResult};

// 导入子模块
pub mod builder;
pub mod dead_wall;

// 重导出常用类型
pub use builder::WallConfig;
pub use dead_wall::{DeadWall, DeadWallConfig};

/// 麻将牌墙，管理游戏中的所有牌
/// 包括主牌墙和岭上牌区(若规则需要)
#[derive(Debug, Clone)]
pub struct Wall {
    /// 主牌墙，存储所有可摸牌
    wall: VecDeque<Tile>,
    
    /// 岭上牌区，存储特殊用途牌(如宝牌指示牌、花牌补充等)
    dead_wall: Option<DeadWall>,
    
    /// 牌墙配置
    config: WallConfig,
    
    /// 已摸出的牌数量，用于判断是否荒牌
    drawn_count: usize,
    
    /// 是否已经开始游戏
    game_started: bool,
}

impl Wall {
    /// 创建一个新的麻将牌墙
    /// 
    /// # 参数
    /// * `wall_config` - 牌墙配置，决定使用哪种规则
    /// * `dead_wall_config` - 岭上牌配置，可以为None表示不使用岭上牌
    /// * `seed` - 可选的随机数种子，用于测试或复现
    /// 
    /// # 返回值
    /// * `Result<Self>` - 成功则返回创建的牌墙，失败则返回错误
    pub fn new(
        wall_config: WallConfig,
        dead_wall_config: Option<DeadWallConfig>,
        seed: Option<u64>
    ) -> MajiangResult<Self> {
        // 创建洗好的牌集
        let mut tiles = builder::create_shuffled_tiles(wall_config, seed)?;
        
        // 如果配置了岭上牌区，则从牌集末尾创建
        let dead_wall = match dead_wall_config {
            Some(config) => Some(DeadWall::new(&mut tiles, config)?),
            None => None,
        };
        
        // 剩余牌放入主牌墙
        let wall = VecDeque::from(tiles);
        
        Ok(Self {
            wall,
            dead_wall,
            config: wall_config,
            drawn_count: 0,
            game_started: false,
        })
    }
    
    /// 获取主牌墙中剩余的牌数量
    pub fn remaining_tiles(&self) -> usize {
        self.wall.len()
    }
    
    /// 检查牌墙是否已经空了(荒牌)
    pub fn is_empty(&self) -> bool {
        self.wall.is_empty()
    }
    
    /// 获取已摸出的牌数量
    pub fn drawn_count(&self) -> usize {
        self.drawn_count
    }
    
    /// 从牌墙摸一张牌
    /// 
    /// # 返回值
    /// * `Result<Tile>` - 成功则返回摸到的牌，失败则返回错误
    pub fn draw_tile(&mut self) -> MajiangResult<Tile> {
        if !self.game_started {
            return Err(MajiangError::InvalidOperation("游戏尚未开始，不能摸牌".to_string()));
        }
        
        if self.wall.is_empty() {
            return Err(MajiangError::NotEnoughTiles);
        }
        
        // 从牌墙前端取牌
        let tile = self.wall.pop_front()
            .ok_or_else(|| MajiangError::NotEnoughTiles)?;
        
        self.drawn_count += 1;
        Ok(tile)
    }
    
    /// 从牌墙发初始手牌(通常是13张)
    /// 
    /// # 参数
    /// * `count` - 要发的牌数量，通常为13
    /// 
    /// # 返回值
    /// * `Result<Vec<Tile>>` - 成功则返回发出的牌组，失败则返回错误
    pub fn deal_initial_hand(&mut self, count: usize) -> MajiangResult<Vec<Tile>> {
        if self.game_started {
            return Err(MajiangError::InvalidOperation("游戏已经开始，不能发初始手牌".to_string()));
        }
        
        if self.wall.len() < count {
            return Err(MajiangError::NotEnoughTiles);
        }
        
        let mut hand = Vec::with_capacity(count);
        
        // 从牌墙前端取指定数量的牌
        for _ in 0..count {
            if let Some(tile) = self.wall.pop_front() {
                hand.push(tile);
                self.drawn_count += 1;
            } else {
                return Err(MajiangError::NotEnoughTiles);
            }
        }
        
        Ok(hand)
    }
    
    /// 杠后从岭上牌区摸一张补牌
    /// 
    /// # 返回值
    /// * `Result<Tile>` - 成功则返回摸到的补牌，失败则返回错误
    pub fn draw_replacement_tile(&mut self) -> MajiangResult<Tile> {
        if !self.game_started {
            return Err(MajiangError::InvalidOperation("游戏尚未开始，不能摸补牌".to_string()));
        }
        
        match &mut self.dead_wall {
            Some(dead_wall) => {
                let tile = dead_wall.draw_replacement_tile()?;
                self.drawn_count += 1;
                Ok(tile)
            },
            None => Err(MajiangError::InvalidOperation("该规则没有岭上牌区".to_string()))
        }
    }
    
    /// 标记游戏已开始，此后不能再发初始手牌
    pub fn start_game(&mut self) {
        self.game_started = true;
    }
    
    /// 获取宝牌指示牌(仅适用于日麻)
    /// 
    /// # 返回值
    /// * `Result<Vec<&Tile>>` - 成功则返回宝牌指示牌列表，失败则返回错误
    pub fn get_dora_indicators(&self) -> MajiangResult<Vec<&Tile>> {
        match &self.dead_wall {
            Some(dead_wall) => dead_wall.get_dora_indicators(),
            None => Err(MajiangError::InvalidOperation("该规则没有岭上牌区".to_string()))
        }
    }
    
    /// 获取里宝牌指示牌(仅适用于日麻)
    /// 
    /// # 返回值
    /// * `Result<Vec<&Tile>>` - 成功则返回里宝牌指示牌列表，失败则返回错误
    pub fn get_uradora_indicators(&self) -> MajiangResult<Vec<&Tile>> {
        match &self.dead_wall {
            Some(dead_wall) => dead_wall.get_uradora_indicators(),
            None => Err(MajiangError::InvalidOperation("该规则没有岭上牌区".to_string()))
        }
    }
    
    /// 在杠后翻开新的宝牌指示牌(仅适用于日麻)
    /// 
    /// # 返回值
    /// * `Result<&Tile>` - 成功则返回新翻开的宝牌指示牌，失败则返回错误
    pub fn reveal_next_dora_indicator(&mut self) -> MajiangResult<&Tile> {
        match &mut self.dead_wall {
            Some(dead_wall) => dead_wall.reveal_next_dora_indicator(),
            None => Err(MajiangError::InvalidOperation("该规则没有岭上牌区".to_string()))
        }
    }
    
    /// 获取岭上牌区
    pub fn dead_wall(&self) -> Option<&DeadWall> {
        self.dead_wall.as_ref()
    }
    
    /// 获取可变的岭上牌区
    pub fn dead_wall_mut(&mut self) -> Option<&mut DeadWall> {
        self.dead_wall.as_mut()
    }
    
    /// 获取牌墙配置
    pub fn config(&self) -> WallConfig {
        self.config
    }
    
    /// 获取主牌墙中的所有牌(仅用于测试)
    #[cfg(test)]
    pub fn get_all_wall_tiles(&self) -> &VecDeque<Tile> {
        &self.wall
    }
}