// src/tile/types.rs
//
// 定义麻将牌的基本类型和常量，作为整个麻将系统的基础数据结构。
// 本文件通过枚举类型替代魔法数字/字符串，确保类型安全，并提供语义清晰的接口。
// 
// 麻将牌的ID系统设计：
// - 每种牌分配唯一ID，便于高效存储和比较
// - ID规则有意设计为连续区间，便于批量操作和范围检查
// - 红宝牌（红五）使用特殊常量标识，便于规则判定

#![allow(dead_code)]
use strum_macros::{EnumString, EnumIter, AsRefStr, EnumCount, Display};

/// 数牌花色（万子、筒子、索子）
/// 
/// 使用枚举确保类型安全，避免使用魔法字符串表示花色。
/// strum宏提供了序列化和显示功能，方便调试和数据交换。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter, AsRefStr, EnumCount)]
pub enum Suit {
    #[strum(serialize = "万")]
    Character, // 万子
    #[strum(serialize = "筒")]
    Dot, // 筒子
    #[strum(serialize = "条")]
    Bamboo, // 索子
}

/// 为Suit实现Default trait
/// 默认选择Character(万子)作为默认值
impl Default for Suit {
    fn default() -> Self {
        Suit::Character
    }
}

/// 风牌（东南西北）
/// 
/// 在麻将中，风牌不仅是普通牌型，还与玩家位置和场风相关，
/// 影响特定规则下的役种判定和得分计算。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter, AsRefStr, EnumCount)]
pub enum Wind {
    #[strum(serialize = "东")]
    East,
    #[strum(serialize = "南")]
    South,
    #[strum(serialize = "西")]
    West,
    #[strum(serialize = "北")]
    North,
}

/// 为Wind实现Default trait
/// 默认选择East(东)作为默认值
impl Default for Wind {
    fn default() -> Self {
        Wind::East
    }
}

/// 三元牌（中发白）
/// 
/// 在大多数麻将规则中，三元牌可以构成特定的役种，
/// 如三元牌全部收集可以构成"大三元"等高分组合。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter, AsRefStr, EnumCount)]
pub enum Dragon {
    #[strum(serialize = "白")]
    White, // 白板
    #[strum(serialize = "发")]
    Green, // 发财
    #[strum(serialize = "中")]
    Red,   // 红中
}

/// 为Dragon实现Default trait
/// 默认选择White(白板)作为默认值
impl Default for Dragon {
    fn default() -> Self {
        Dragon::White
    }
}

/// 花牌（春夏秋冬梅兰竹菊）
/// 
/// 花牌在不同麻将规则中有不同处理方式：
/// - 日本麻将中通常不使用花牌
/// - 中国麻将中花牌通常需要立即补牌，并可能提供额外得分
/// - 部分规则下，花牌配对可形成特殊得分组合
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter, AsRefStr, EnumCount)]
pub enum Flower {
    #[strum(serialize = "春")]
    Spring, 
    #[strum(serialize = "夏")]
    Summer, 
    #[strum(serialize = "秋")]
    Autumn, 
    #[strum(serialize = "冬")]
    Winter, // 季节
    #[strum(serialize = "梅")]
    Plum, 
    #[strum(serialize = "兰")]
    Orchid, 
    #[strum(serialize = "竹")]
    Bamboo, 
    #[strum(serialize = "菊")]
    Chrysanthemum, // 植物
}

/// 为Flower实现Default trait
/// 默认选择Spring(春)作为默认值
impl Default for Flower {
    fn default() -> Self {
        Flower::Spring
    }
}

/// 麻将牌类型，统一表示所有牌
/// 
/// 这是所有麻将牌的核心枚举类型，通过代数数据类型（ADT）优雅地表示所有可能的牌型。
/// 设计遵循Rust的类型安全原则，确保在编译时捕获牌型错误，而不是运行时。
///
/// # 示例
/// ```
/// let man5 = Tile::Suit(Suit::Character, 5);
/// let east = Tile::Wind(Wind::East);
/// let red_dragon = Tile::Dragon(Dragon::Red);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString)]
pub enum Tile {
    /// 数牌：花色和点数(1-9)
    #[strum(serialize = "数牌")]
    Suit(Suit, u8),
    
    /// 风牌：东南西北
    #[strum(serialize = "风牌")]
    Wind(Wind),
    
    /// 三元牌：中发白
    #[strum(serialize = "三元")]
    Dragon(Dragon),
    
    /// 花牌：春夏秋冬梅兰竹菊
    #[strum(serialize = "花牌")]
    Flower(Flower),
    
    /// 百搭/万能牌：可替代任何非花牌
    /// 注意：不是所有麻将规则都使用百搭牌
    #[strum(serialize = "百搭")]
    Joker,
}

/// 为Tile实现Default trait
/// 默认选择一万作为默认值，这也是麻将牌中最常见的起始牌
impl Default for Tile {
    fn default() -> Self {
        Tile::Suit(Suit::Character, 1)
    }
}

// --- Tile ID Constants ---
// 牌ID系统设计：为每种牌分配唯一标识符
// 设计为连续区间，便于范围检查和批量操作
// 0-8:   1m-9m  (万子)
// 9-17:  1p-9p  (筒子)
// 18-26: 1s-9s  (索子)
// 27:    东
// 28:    南
// 29:    西
// 30:    北
// 31:    白板
// 32:    发财
// 33:    红中
// 34-41: 花牌
// 42:    百搭牌

/// 万子起始ID (1万对应ID=0)
pub(crate) const CHARACTER_START: u8 = 0;
/// 筒子起始ID (1筒对应ID=9)
pub(crate) const DOT_START: u8 = 9;
/// 索子起始ID (1索对应ID=18)
pub(crate) const BAMBOO_START: u8 = 18;
/// 风牌起始ID (东风对应ID=27)
pub(crate) const WIND_START: u8 = 27;
/// 三元牌起始ID (白板对应ID=31)
pub(crate) const DRAGON_START: u8 = 31;
/// 花牌起始ID (春对应ID=34)
pub(crate) const FLOWER_START: u8 = 34;
/// 字牌(风牌+三元牌)最大ID
pub(crate) const MAX_HONOR_ID: u8 = 33;
/// 花牌最大ID
pub(crate) const MAX_FLOWER_ID: u8 = 41;
/// 百搭牌ID
pub(crate) const JOKER_ID: u8 = 42;

/// 红五万ID - 用于表示红宝牌
pub(crate) const RED_MAN5_ID: u8 = CHARACTER_START + 5 - 1; // 4
/// 红五筒ID - 用于表示红宝牌
pub(crate) const RED_PIN5_ID: u8 = DOT_START + 5 - 1; // 13
/// 红五索ID - 用于表示红宝牌
pub(crate) const RED_SOU5_ID: u8 = BAMBOO_START + 5 - 1; // 22
