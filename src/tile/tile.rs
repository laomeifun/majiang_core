#[allow(unused_imports)]
use crate::tile::{Tile, Suit, Wind, Dragon, Flower};
use crate::tile::types::{
    MAN_START, PIN_START, SOU_START, WIND_START, DRAGON_START, 
    FLOWER_START, JOKER_ID, RED_MAN5_ID, RED_PIN5_ID, RED_SOU5_ID
};

/// Tile 结构的核心操作实现
/// 
/// 本文件实现了麻将牌的所有核心操作，包括创建、转换和类型判断。
/// 设计原则：
/// 1. 提供安全直观的创建方法，避免非法牌的构造
/// 2. 高效的ID转换系统，便于内部存储和比较
/// 3. 丰富的类型判断方法，为规则判定提供便捷接口
impl Tile {
    /// 创建数牌，会验证点数范围(1-9)
    /// 
    /// 设计为返回Option以处理非法输入，比如超出范围的点数。
    /// 这比直接panic更安全，允许调用者灵活处理错误。
    /// 
    /// # 参数
    /// * `suit` - 花色（万、筒、条）
    /// * `number` - 点数(1-9)
    /// 
    /// # 返回
    /// * `Some(Tile)` - 创建成功
    /// * `None` - 点数超出范围
    pub fn new_suit(suit: Suit, number: u8) -> Option<Self> {
        if (1..=9).contains(&number) {
            Some(Tile::Suit(suit, number))
        } else {
            None
        }
    }
    
    /// 从牌ID转换为牌类型
    /// 
    /// ID系统是内部优化的关键，允许将复杂的牌类型映射到简单的整数，
    /// 便于快速比较、排序和存储。
    /// 
    /// # 参数
    /// * `id` - 牌的唯一ID值
    /// 
    /// # 返回
    /// * `Some(Tile)` - 有效的牌
    /// * `None` - 无效ID
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            // 数牌：万
            0..=8 => {
                let number = id - MAN_START + 1;
                Some(Tile::Suit(Suit::Character, number))
            }
            // 数牌：筒
            9..=17 => {
                let number = id - PIN_START + 1;
                Some(Tile::Suit(Suit::Dot, number))
            }
            // 数牌：条
            18..=26 => {
                let number = id - SOU_START + 1;
                Some(Tile::Suit(Suit::Bamboo, number))
            }
            // 风牌
            27 => Some(Tile::Wind(Wind::East)),
            28 => Some(Tile::Wind(Wind::South)),
            29 => Some(Tile::Wind(Wind::West)),
            30 => Some(Tile::Wind(Wind::North)),
            // 三元牌
            31 => Some(Tile::Dragon(Dragon::White)),
            32 => Some(Tile::Dragon(Dragon::Green)),
            33 => Some(Tile::Dragon(Dragon::Red)),
            // 花牌
            34..=41 => {
                let flower_idx = id - FLOWER_START;
                match flower_idx {
                    0 => Some(Tile::Flower(Flower::Spring)),
                    1 => Some(Tile::Flower(Flower::Summer)),
                    2 => Some(Tile::Flower(Flower::Autumn)),
                    3 => Some(Tile::Flower(Flower::Winter)),
                    4 => Some(Tile::Flower(Flower::Plum)),
                    5 => Some(Tile::Flower(Flower::Orchid)),
                    6 => Some(Tile::Flower(Flower::Bamboo)),
                    7 => Some(Tile::Flower(Flower::Chrysanthemum)),
                    _ => None,
                }
            },
            // 百搭牌
            JOKER_ID => Some(Tile::Joker),
            _ => None,
        }
    }

    /// 转换为牌ID
    /// 
    /// 是from_id的反向操作，将牌对象转换回唯一ID。
    /// 这种双向转换确保了ID系统的一致性。
    /// 
    /// # 返回
    /// * `u8` - 牌的唯一ID
    pub fn to_id(&self) -> u8 {
        match self {
            Tile::Suit(Suit::Character, n) => MAN_START + n - 1,
            Tile::Suit(Suit::Dot, n) => PIN_START + n - 1,
            Tile::Suit(Suit::Bamboo, n) => SOU_START + n - 1,
            Tile::Wind(Wind::East) => WIND_START,
            Tile::Wind(Wind::South) => WIND_START + 1,
            Tile::Wind(Wind::West) => WIND_START + 2,
            Tile::Wind(Wind::North) => WIND_START + 3,
            Tile::Dragon(Dragon::White) => DRAGON_START,
            Tile::Dragon(Dragon::Green) => DRAGON_START + 1,
            Tile::Dragon(Dragon::Red) => DRAGON_START + 2,
            Tile::Flower(Flower::Spring) => FLOWER_START,
            Tile::Flower(Flower::Summer) => FLOWER_START + 1,
            Tile::Flower(Flower::Autumn) => FLOWER_START + 2,
            Tile::Flower(Flower::Winter) => FLOWER_START + 3,
            Tile::Flower(Flower::Plum) => FLOWER_START + 4,
            Tile::Flower(Flower::Orchid) => FLOWER_START + 5,
            Tile::Flower(Flower::Bamboo) => FLOWER_START + 6,
            Tile::Flower(Flower::Chrysanthemum) => FLOWER_START + 7,
            Tile::Joker => JOKER_ID,
        }
    }
    
    /// 判断是否为数牌
    /// 
    /// 数牌是麻将中最基础的牌型，由三种花色(万、筒、条)的1-9点组成。
    /// 在和牌构成中扮演重要角色。
    pub fn is_suit(&self) -> bool {
        matches!(self, Tile::Suit(_, _))
    }
    
    /// 判断是否为风牌
    /// 
    /// 风牌(东南西北)是字牌的一种，在特定规则中与场风、自风相关，
    /// 可影响役种判定和得分计算。
    pub fn is_wind(&self) -> bool {
        matches!(self, Tile::Wind(_))
    }
    
    /// 判断是否为三元牌
    /// 
    /// 三元牌(中发白)是字牌的一种，在多数规则中可构成特殊役种，
    /// 如"小三元"、"大三元"等高分组合。
    pub fn is_dragon(&self) -> bool {
        matches!(self, Tile::Dragon(_))
    }
    
    /// 判断是否为花牌
    /// 
    /// 花牌在不同规则中处理方式不同：
    /// - 日麻通常不使用花牌
    /// - 中式麻将中通常摸到花牌需立即补牌
    pub fn is_flower(&self) -> bool {
        matches!(self, Tile::Flower(_))
    }
    
    /// 判断是否为字牌（风牌或三元牌）
    /// 
    /// 字牌是风牌和三元牌的统称，在和牌结构和得分计算中，
    /// 通常作为一类特殊牌型统一处理。
    pub fn is_honor(&self) -> bool {
        self.is_wind() || self.is_dragon()
    }
    
    /// 判断是否为百搭牌
    /// 
    /// 百搭牌是一种特殊牌型，在某些规则变体中允许替代其他牌型。
    /// 注意：并非所有麻将规则都使用百搭牌。
    pub fn is_joker(&self) -> bool {
        matches!(self, Tile::Joker)
    }
    
    /// 判断是否为红宝牌（红五）
    /// 
    /// 红宝牌是一种特殊计分牌型，通常是红色的五万、五筒或五条，
    /// 在日式和部分中式规则中可提供额外得分。
    /// 
    /// 实现时使用ID比较而非直接匹配，为将来支持多种红牌配置预留空间。
    pub fn is_red(&self) -> bool {
        match self {
            Tile::Suit(_suit, 5) => {
                let id = self.to_id();
                id == RED_MAN5_ID || id == RED_PIN5_ID || id == RED_SOU5_ID
            }
            _ => false,
        }
    }
}

/// 为 Tile 实现 Display 特性，用于美观地打印牌
/// 
/// 基本格式约定：
/// - 数牌：数字+花色，如"1万"
/// - 字牌：直接显示，如"东"、"中"
/// - 花牌：直接显示，如"春"
/// - 百搭：显示为"百搭"
impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Suit(suit, n) => write!(f, "{}{}", n, suit),
            Tile::Wind(wind) => write!(f, "{}", wind),
            Tile::Dragon(dragon) => write!(f, "{}", dragon),
            Tile::Flower(flower) => write!(f, "{}", flower),
            Tile::Joker => write!(f, "百搭"),
        }
    }
}