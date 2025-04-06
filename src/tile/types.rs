// src/tile/types.rs

/// 数牌花色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Wan, // 万子
    Tong, // 筒子
    Tiao, // 索子
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Wind {
    East,
    South,
    West,
    North,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dragon {
    White, // 白板
    Green, // 发财
    Red,   // 红中
}

/// 花牌类型 (春夏秋冬梅兰竹菊)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flower {
    Spring, Summer, Autumn, Winter, // 季节
    Plum, Orchid, Bamboo, Chrysanthemum, // 植物
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
