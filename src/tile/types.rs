// src/tile/types.rs
use strum_macros::{Display, EnumString, EnumIter, AsRefStr, EnumCount};

/// 数牌花色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter, AsRefStr, EnumCount)]
pub enum Suit {
    #[strum(serialize = "万")]
    Wan, // 万子
    #[strum(serialize = "筒")]
    Tong, // 筒子
    #[strum(serialize = "条")]
    Tiao, // 索子
}


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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter, AsRefStr, EnumCount)]
pub enum Dragon {
    #[strum(serialize = "白")]
    White, // 白板
    #[strum(serialize = "发")]
    Green, // 发财
    #[strum(serialize = "中")]
    Red,   // 红中
}

/// 花牌类型 (春夏秋冬梅兰竹菊)
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

// --- Tile ID Constants ---
// 0-8:   1m-9m
// 9-17:  1p-9p
// 18-26: 1s-9s
// 27:    East
// 28:    South
// 29:    West
// 30:    North
// 31:    White Dragon
// 32:    Green Dragon
// 33:    Red Dragon
// 34-41: Flowers

pub(crate) const MAN_START: u8 = 0;
pub(crate) const PIN_START: u8 = 9;
pub(crate) const SOU_START: u8 = 18;
pub(crate) const WIND_START: u8 = 27;
pub(crate) const DRAGON_START: u8 = 31;
pub(crate) const FLOWER_START: u8 = 34; // 花牌 ID 起始
pub(crate) const MAX_HONOR_ID: u8 = 33; // 字牌最大 ID
pub(crate) const MAX_FLOWER_ID: u8 = 41; // 花牌最大 ID (34-41)

// ID for red fives (used internally for checks)
pub(crate) const RED_MAN5_ID: u8 = MAN_START + 5 - 1; // 4
pub(crate) const RED_PIN5_ID: u8 = PIN_START + 5 - 1; // 13
pub(crate) const RED_SOU5_ID: u8 = SOU_START + 5 - 1; // 22
