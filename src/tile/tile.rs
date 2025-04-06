// src/tile/tile.rs

use super::types::*; // 导入同级目录下的 types 模块

/// 代表一张麻将牌
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tile {
    pub(crate) id: u8, // 改为 pub(crate) 以便 display 模块访问
    /// 是否是红宝牌 (仅对 5m, 5p, 5s 有效)
    pub(crate) is_red_dora: bool, // 改为 pub(crate)
}

impl Tile {
    // --- Private constructor ---
    fn new(id: u8, is_red_dora: bool) -> Self {
        // 红宝牌标记只对 5m, 5p, 5s 的 ID 有效
        let effective_red = is_red_dora && [RED_MAN5_ID, RED_PIN5_ID, RED_SOU5_ID].contains(&id);
        Tile { id, is_red_dora: effective_red }
    }

    /// 创建万字牌 (1-9)
    pub fn man(value: u8) -> Option<Self> {
        if (1..=9).contains(&value) {
            Some(Self::new(MAN_START + value - 1, false))
        } else {
            None
        }
    }

    /// 创建筒子牌 (1-9)
    pub fn pin(value: u8) -> Option<Self> {
        if (1..=9).contains(&value) {
            Some(Self::new(PIN_START + value - 1, false))
        } else {
            None
        }
    }

    /// 创建索子牌 (1-9)
    pub fn sou(value: u8) -> Option<Self> {
        if (1..=9).contains(&value) {
            Some(Self::new(SOU_START + value - 1, false))
        } else {
            None
        }
    }

    /// 创建风牌
    pub fn wind(wind: Wind) -> Self {
        let id = match wind {
            Wind::East => WIND_START,
            Wind::South => WIND_START + 1,
            Wind::West => WIND_START + 2,
            Wind::North => WIND_START + 3,
        };
        Self::new(id, false)
    }

    /// 创建箭牌
    pub fn dragon(dragon: Dragon) -> Self {
        let id = match dragon {
            Dragon::White => DRAGON_START,
            Dragon::Green => DRAGON_START + 1,
            Dragon::Red => DRAGON_START + 2,
        };
        Self::new(id, false)
    }

    /// 创建花牌
    pub fn flower(flower: Flower) -> Self {
        let id = match flower {
            Flower::Spring => FLOWER_START,
            Flower::Summer => FLOWER_START + 1,
            Flower::Autumn => FLOWER_START + 2,
            Flower::Winter => FLOWER_START + 3,
            Flower::Plum => FLOWER_START + 4,
            Flower::Orchid => FLOWER_START + 5,
            Flower::Bamboo => FLOWER_START + 6,
            Flower::Chrysanthemum => FLOWER_START + 7,
        };
        Self::new(id, false)
    }

    // --- Red Dora Constructors ---
    /// 创建红 5 万
    pub fn red_man5() -> Self { Self::new(RED_MAN5_ID, true) }
    /// 创建红 5 筒
    pub fn red_pin5() -> Self { Self::new(RED_PIN5_ID, true) }
    /// 创建红 5 索
    pub fn red_sou5() -> Self { Self::new(RED_SOU5_ID, true) }


    /// 获取牌的内部 ID (不区分红宝牌)
    pub fn id(&self) -> u8 { self.id }

    /// 检查是否是红宝牌
    pub fn is_red_dora(&self) -> bool { self.is_red_dora }

    /// 返回一个占位符牌 (例如用于不需要具体和牌的检查)
    pub fn placeholder() -> Self {
        Self::new(MAN_START, false) // 返回 1 万作为占位符
    }

    // --- Helper methods ---

    /// 获取牌的花色和数值 (如果是数牌)
    pub fn get_suit_and_value(&self) -> Option<(Suit, u8)> {
        match self.id {
            id @ MAN_START..=8 => Some((Suit::Wan, id - MAN_START + 1)),
            id @ PIN_START..=17 => Some((Suit::Tong, id - PIN_START + 1)),
            id @ SOU_START..=26 => Some((Suit::Tiao, id - SOU_START + 1)),
            _ => None,
        }
    }

    /// 获取风牌类型 (如果是风牌)
    pub fn get_wind(&self) -> Option<Wind> {
        match self.id {
            // 使用常量和 match guards 进行匹配
            WIND_START => Some(Wind::East),
            id if id == WIND_START + 1 => Some(Wind::South),
            id if id == WIND_START + 2 => Some(Wind::West),
            id if id == WIND_START + 3 => Some(Wind::North),
            _ => None,
        }
    }

    /// 获取箭牌类型 (如果是箭牌)
    pub fn get_dragon(&self) -> Option<Dragon> {
        match self.id {
            // 使用常量和 match guards 进行匹配
            DRAGON_START => Some(Dragon::White),
            id if id == DRAGON_START + 1 => Some(Dragon::Green),
            id if id == DRAGON_START + 2 => Some(Dragon::Red),
            _ => None,
        }
    }

    /// 获取花牌类型 (如果是花牌)
    pub fn get_flower(&self) -> Option<Flower> {
        match self.id {
            // 使用常量和 match guards 进行匹配
            FLOWER_START => Some(Flower::Spring),
            id if id == FLOWER_START + 1 => Some(Flower::Summer),
            id if id == FLOWER_START + 2 => Some(Flower::Autumn),
            id if id == FLOWER_START + 3 => Some(Flower::Winter),
            id if id == FLOWER_START + 4 => Some(Flower::Plum),
            id if id == FLOWER_START + 5 => Some(Flower::Orchid),
            id if id == FLOWER_START + 6 => Some(Flower::Bamboo),
            id if id == FLOWER_START + 7 => Some(Flower::Chrysanthemum),
            _ => None,
        }
    }

    /// 是否是万字牌
    pub fn is_man(&self) -> bool {
        (MAN_START..=8).contains(&self.id)
    }
    /// 是否是筒子牌
    pub fn is_pin(&self) -> bool {
        (PIN_START..=17).contains(&self.id)
    }
    /// 是否是索子牌
    pub fn is_sou(&self) -> bool {
        (SOU_START..=26).contains(&self.id)
    }
    /// 是否是数牌 (万筒索)
    pub fn is_suit(&self) -> bool {
        self.is_man() || self.is_pin() || self.is_sou()
    }

    /// 是否是风牌
    pub fn is_wind(&self) -> bool {
        (WIND_START..=WIND_START + 3).contains(&self.id)
    }
    /// 是否是箭牌
    pub fn is_dragon(&self) -> bool {
        (DRAGON_START..=MAX_HONOR_ID).contains(&self.id)
    }
    /// 是否是字牌 (风牌或箭牌)
    pub fn is_honor(&self) -> bool {
        self.is_wind() || self.is_dragon()
    }
    /// 是否是花牌
    pub fn is_flower(&self) -> bool {
        (FLOWER_START..=MAX_FLOWER_ID).contains(&self.id)
    }

    /// 是否是幺九牌 (1, 9 的数牌或字牌)
    pub fn is_terminal_or_honor(&self) -> bool {
        if self.is_honor() {
            return true;
        }
        if let Some((_, value)) = self.get_suit_and_value() {
            value == 1 || value == 9
        } else {
            false // Should not happen if not honor or flower
        }
    }

    /// 是否是老头牌 (1, 9 的数牌)
    pub fn is_terminal(&self) -> bool {
         if let Some((_, value)) = self.get_suit_and_value() {
            value == 1 || value == 9
        } else {
            false
        }
    }

    /// 是否是中张牌 (2-8 的数牌)
    pub fn is_simple(&self) -> bool {
        self.is_suit() && !self.is_terminal()
    }
}
