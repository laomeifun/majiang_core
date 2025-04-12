// src/meld/types.rs
//
// 定义与副露相关的所有类型，包括副露类型、杠类型和副露来源
// 这些类型是麻将游戏中表示玩家公开面子组合的基础数据结构

use crate::errors::MajiangError;
use crate::tile::Tile;
use std::fmt;

/// 副露类型
/// 
/// 在麻将中，副露是指通过一定规则组合形成的、摆在玩家面前的牌组
/// 主要包括吃、碰、杠三种基本类型，每种类型都有特定的构成规则
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MeldType {
    /// 吃: 由三张连续的同花色数牌组成
    /// 例如: 2万、3万、4万
    Chi,

    /// 碰: 由三张相同牌组成
    /// 例如: 三张5筒
    Pon,

    /// 杠: 由四张相同牌组成
    /// 杠有三种形式: 明杠、暗杠和加杠，具体由KanType指定
    Kan(KanType),
}

/// 杠的类型
/// 
/// 杠虽然都是由四张相同牌组成，但根据形成方式不同有三种类型，
/// 这些类型在不同规则中会有不同的计分和策略意义
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KanType {
    /// 明杠: 用其他玩家打出的一张牌和自己手里的三张相同牌组成
    Open,

    /// 暗杠: 完全由自己手里的四张相同牌组成，不需要别人打出的牌
    Closed,

    /// 加杠: 在已经碰过的三张牌的基础上，摸到或拿到第四张相同牌后组成
    /// 加杠是一种特殊情况，需要先有碰，然后在游戏过程中升级为杠
    Added,
}

/// 副露来源
/// 
/// 标识一个副露组合中牌的来源，用于追踪每张牌是从哪里获得的
/// 这对于实现正确的游戏流程和记录非常重要
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MeldSource {
    /// 自己摸到的牌
    SelfDrawn,

    /// 从特定玩家处获得的牌(通常是他们打出的牌)
    /// u8表示玩家的位置索引
    Player(u8),
}

/// 副露结构体
/// 
/// 描述一组特定类型的副露，包含了组成副露的全部牌、
/// 副露类型和牌的来源信息
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Meld {
    /// 组成副露的牌
    pub tiles: Vec<Tile>,

    /// 副露类型(吃/碰/杠)
    pub meld_type: MeldType,

    /// 标记哪些牌来自哪个玩家
    /// 索引与tiles对应，表示每张牌的来源
    pub sources: Vec<MeldSource>,
}

impl Meld {
    /// 创建新的副露
    /// 
    /// # 参数
    /// * `tiles` - 构成副露的牌列表
    /// * `meld_type` - 副露类型(吃/碰/杠)
    /// * `sources` - 每张牌的来源
    /// 
    /// # 返回
    /// * `Result<Meld, MajiangError>` - 成功创建的副露或错误
    pub fn new(
        tiles: Vec<Tile>, 
        meld_type: MeldType, 
        sources: Vec<MeldSource>
    ) -> Result<Self, MajiangError> {
        // 验证牌数量与副露类型是否匹配
        match meld_type {
            MeldType::Chi | MeldType::Pon => {
                if tiles.len() != 3 {
                    return Err(MajiangError::InvalidMeld(
                        format!("{:?}副露必须由3张牌组成", meld_type)
                    ));
                }
            },
            MeldType::Kan(_) => {
                if tiles.len() != 4 {
                    return Err(MajiangError::InvalidMeld(
                        "杠必须由4张牌组成".to_string()
                    ));
                }
            },
        }
        
        // 验证来源数量与牌数量一致
        if tiles.len() != sources.len() {
            return Err(MajiangError::InvalidMeld(
                "每张牌必须有对应的来源信息".to_string()
            ));
        }
        
        Ok(Meld {
            tiles,
            meld_type,
            sources,
        })
    }
    
    /// 获取副露中的关键牌
    /// 对于吃，返回中间的牌
    /// 对于碰和杠，返回其中一张牌
    pub fn get_key_tile(&self) -> Tile {
        match self.meld_type {
            MeldType::Chi => self.tiles[1], // 吃的中间牌
            MeldType::Pon | MeldType::Kan(_) => self.tiles[0], // 碰/杠的任意一张牌(因为都相同)
        }
    }
    
    /// 判断是否为明露(公开)的副露
    /// 明露副露影响和牌判定(如门清、暗杠等)
    pub fn is_open(&self) -> bool {
        match self.meld_type {
            MeldType::Chi | MeldType::Pon => true,
            MeldType::Kan(kan_type) => match kan_type {
                KanType::Open | KanType::Added => true,
                KanType::Closed => false,
            },
        }
    }
    
    /// 获取副露包含的牌数量
    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }
}

impl fmt::Display for MeldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MeldType::Chi => write!(f, "吃"),
            MeldType::Pon => write!(f, "碰"),
            MeldType::Kan(kan_type) => match kan_type {
                KanType::Open => write!(f, "明杠"),
                KanType::Closed => write!(f, "暗杠"),
                KanType::Added => write!(f, "加杠"),
            },
        }
    }
}

impl fmt::Display for Meld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tiles_str: Vec<String> = self.tiles.iter().map(|t| t.to_string()).collect();
        write!(f, "{}: [{}]", self.meld_type, tiles_str.join(", "))
    }
}