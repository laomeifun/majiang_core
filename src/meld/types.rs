// src/meld/types.rs
//
// 定义与副露(吃、碰、杠)相关的数据类型和基本方法
// 这些类型构成了麻将游戏中副露操作的数据模型

use crate::errors::{MajiangError, MajiangResult};
use crate::tile::Tile;

/// 牌的来源，用于标识副露中每张牌的来源
/// 
/// 在麻将游戏中，副露的牌可能来自自己摸到的牌，也可能来自其他玩家打出的牌。
/// 这个枚举用于追踪每张牌的具体来源，对于判断副露的合法性和计分至关重要。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeldSource {
    /// 自己摸到的牌
    /// 
    /// 这种来源的牌是玩家通过摸牌阶段获得的，而非从他人处获取
    SelfDrawn,
    
    /// 从其他玩家处获得的牌
    /// 
    /// 包含一个玩家ID，表示牌来自哪位玩家
    /// 对于吃、碰、明杠，至少有一张牌应该是从其他玩家处获得的
    Player(u8),
}

/// 杠的类型
/// 
/// 麻将中的杠有三种不同类型，它们的形成方式、显示方式和计分方式各不相同。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KanType {
    /// 明杠
    /// 
    /// 使用手中三张相同的牌和其他玩家打出的一张牌组成，所有牌明示
    /// 例如：手中有三张5万，其他玩家打出一张5万，可以明杠
    Open,
    
    /// 暗杠
    /// 
    /// 使用自己手牌中的四张相同的牌组成，只有自己知道具体是什么牌
    /// 例如：手中有四张白板，可以暗杠
    Closed,
    
    /// 加杠
    /// 
    /// 在已有碰的基础上，摸到第四张相同的牌后进行的杠操作
    /// 例如：之前碰了三张7筒，后来又摸到一张7筒，可以加杠
    Added,
}

/// 副露的类型
/// 
/// 麻将中的副露分为三种基本类型：吃、碰、杠。其中杠又进一步细分为明杠、暗杠和加杠。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeldType {
    /// 吃
    /// 
    /// 使用手牌中的两张连续数牌，加上他人打出的一张数牌，组成顺子
    /// 例如：手中有3万、4万，上家打出2万，可以吃成2-3-4万的顺子
    Chi,
    
    /// 碰
    /// 
    /// 使用手牌中的两张相同的牌，加上他人打出的一张相同的牌，组成刻子
    /// 例如：手中有两张8条，任何人打出一张8条，可以碰成三张8条的刻子
    Pon,
    
    /// 杠 
    /// 
    /// 由四张完全相同的牌组成
    /// 有三种不同的杠：明杠(Open)、暗杠(Closed)、加杠(Added)
    /// 每种杠的形成方式和规则略有不同
    Kan(KanType),
}

/// 表示副露的结构，包含牌组、副露类型和牌的来源信息
/// 
/// 副露是指玩家通过吃、碰、杠等操作公开亮出的牌组合。这个结构保存了副露的所有
/// 必要信息，包括组成副露的具体牌、副露的类型以及每张牌的来源。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Meld {
    /// 组成副露的牌
    pub tiles: Vec<Tile>,
    
    /// 副露的类型（吃、碰、杠）
    pub meld_type: MeldType,
    
    /// 牌的来源，与tiles数组一一对应
    pub sources: Vec<MeldSource>,
}

impl Meld {
    /// 创建一个新的副露实例
    /// 
    /// 该方法会对输入进行验证，确保牌组和来源信息符合特定副露类型的要求。
    /// 
    /// # 参数
    /// 
    /// * `tiles` - 组成副露的牌
    /// * `meld_type` - 副露类型（吃、碰、杠）
    /// * `sources` - 每张牌的来源
    /// 
    /// # 返回值
    /// 
    /// 成功时返回创建的副露实例，失败时返回错误
    /// 
    /// # 错误
    /// 
    /// 如果输入不符合要求，将返回适当的错误：
    /// 
    /// * 牌和来源数量不匹配 - `InvalidMeld`
    /// * 牌数量不符合副露类型要求 - `InvalidMeld`
    /// * 牌组不符合副露类型的规则 - `InvalidMeld`
    /// 
    /// # 示例
    /// 
    /// ```
    /// use majiang_core::tile::{Tile, Suit};
    /// use majiang_core::meld::{Meld, MeldType, MeldSource};
    /// 
    /// // 创建一个碰副露
    /// let pon_tile = Tile::new_suit(Suit::Character, 7).unwrap();
    /// let tiles = vec![pon_tile; 3];
    /// let sources = vec![
    ///     MeldSource::SelfDrawn,
    ///     MeldSource::SelfDrawn,
    ///     MeldSource::Player(2)  // 从玩家2处获得一张牌
    /// ];
    /// 
    /// let pon = Meld::new(tiles, MeldType::Pon, sources).unwrap();
    /// ```
    pub fn new(tiles: Vec<Tile>, meld_type: MeldType, sources: Vec<MeldSource>) -> MajiangResult<Self> {
        // 验证牌和来源数量是否匹配
        if tiles.len() != sources.len() {
            return Err(MajiangError::InvalidMeld(
                "牌和来源数量必须匹配".to_string()
            ));
        }

        // 根据副露类型检查牌的数量
        let expected_count = match meld_type {
            MeldType::Chi => 3,
            MeldType::Pon => 3,
            MeldType::Kan(_) => 4,
        };

        if tiles.len() != expected_count {
            return Err(MajiangError::InvalidMeld(
                format!("{}副露需要{}张牌，但提供了{}张", 
                    meld_type_to_str(meld_type), expected_count, tiles.len())
            ));
        }

        // 验证通过，创建副露
        Ok(Self { 
            tiles, 
            meld_type, 
            sources 
        })
    }

    /// 获取副露中的关键牌
    /// 
    /// 对于不同类型的副露，关键牌定义如下：
    /// 
    /// * 吃：中间的牌（如2-3-4中的3）
    /// * 碰：其中一张牌（因为所有牌都一样）
    /// * 杠：其中一张牌（因为所有牌都一样）
    /// 
    /// # 返回值
    /// 
    /// 返回副露中的关键牌
    pub fn get_key_tile(&self) -> Tile {
        match self.meld_type {
            MeldType::Chi => {
                // 对于吃，返回中间的牌
                // 注意：此处假设tiles已按顺序排好，如2-3-4
                self.tiles[1]
            }
            MeldType::Pon | MeldType::Kan(_) => {
                // 对于碰和杠，所有牌都一样，返回第一张
                self.tiles[0]
            }
        }
    }
    
    /// 检查这个副露是否包含特定的牌
    /// 
    /// # 参数
    /// 
    /// * `tile` - 要查找的牌
    /// 
    /// # 返回值
    /// 
    /// 如果副露中包含该牌，则返回true，否则返回false
    pub fn contains(&self, tile: Tile) -> bool {
        self.tiles.contains(&tile)
    }
    
    /// 判断这个副露是否是明的（可见的）
    /// 
    /// 对于吃、碰和明杠，它们是明的；对于暗杠，它是暗的（只对自己可见）
    /// 
    /// # 返回值
    /// 
    /// 如果副露是明的，则返回true，否则返回false
    pub fn is_open(&self) -> bool {
        match self.meld_type {
            MeldType::Chi | MeldType::Pon => true,
            MeldType::Kan(kan_type) => kan_type != KanType::Closed,
        }
    }
}

/// 将副露类型转换为字符串，用于调试和错误信息
fn meld_type_to_str(meld_type: MeldType) -> &'static str {
    match meld_type {
        MeldType::Chi => "吃",
        MeldType::Pon => "碰",
        MeldType::Kan(KanType::Open) => "明杠",
        MeldType::Kan(KanType::Closed) => "暗杠",
        MeldType::Kan(KanType::Added) => "加杠",
    }
}

/// 将杠类型转换为字符串，用于调试和错误信息
/// 
/// 此函数将杠类型（明杠、暗杠、加杠）转换为对应的中文描述，
/// 主要用于生成用户友好的错误信息和日志。
/// 
/// # 参数
/// 
/// * `kan_type` - 要转换的杠类型
/// 
/// # 返回值
/// 
/// * `&'static str` - 杠类型对应的中文描述
#[allow(dead_code)]
fn kan_type_to_str(kan_type: KanType) -> &'static str {
    match kan_type {
        KanType::Open => "明杠",
        KanType::Closed => "暗杠",
        KanType::Added => "加杠",
    }
}