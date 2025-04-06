// src/tile/display.rs

use std::fmt;
use super::tile::Tile; // 导入同级目录下的 tile 模块中的 Tile
use super::types::*;   // 导入同级目录下的 types 模块中的所有内容

// --- Display Implementation ---

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((suit, value)) = self.get_suit_and_value() {
            let display_value = if self.is_red_dora {
                '0' // Use '0' for red fives (e.g., 0m, 0p, 0s)
            } else {
                // Convert number to char
                std::char::from_digit(value as u32, 10).unwrap_or('?')
            };
            let suit_char = match suit {
                Suit::Wan => 'm',
                Suit::Tong => 'p',
                Suit::Tiao => 's',
            };
            write!(f, "{}{}", display_value, suit_char)
        } else if let Some(wind) = self.get_wind() {
            let wind_char = match wind {
                Wind::East => 'E', // 东
                Wind::South => 'S', // 南
                Wind::West => 'W', // 西
                Wind::North => 'N', // 北
            };
            write!(f, "{}", wind_char)
        } else if let Some(dragon) = self.get_dragon() {
            let dragon_char = match dragon {
                Dragon::White => 'P', // 白 (Po) / 白板
                Dragon::Green => 'F', // 發 (Fa) / 发财
                Dragon::Red => 'C',   // 中 (Chun) / 红中
            };
            write!(f, "{}", dragon_char)
        } else if let Some(flower) = self.get_flower() {
             // 花牌表示 (可以自定义)
             let flower_repr = match flower {
                 Flower::Spring => "Fl1",        // 春
                 Flower::Summer => "Fl2",        // 夏
                 Flower::Autumn => "Fl3",        // 秋
                 Flower::Winter => "Fl4",        // 冬
                 Flower::Plum => "Fp1",          // 梅
                 Flower::Orchid => "Fp2",        // 兰
                 Flower::Bamboo => "Fp3",        // 竹
                 Flower::Chrysanthemum => "Fp4", // 菊
             };
             write!(f, "{}", flower_repr)
        } else {
            write!(f, "?{}", self.id) // Unknown tile ID
        }
    }
}


// --- Debug Implementation (more readable) ---
// We derive Ord/PartialOrd based on id, so Debug should reflect that for clarity
impl fmt::Debug for Tile {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the Display implementation but add red dora indicator if needed
        if self.is_red_dora {
            write!(f, "{}(R)", self)
        } else {
            write!(f, "{}", self)
        }
    }
}
