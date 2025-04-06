// src/tile/display.rs
//
// 为麻将牌提供多种显示格式，包括Unicode符号、ASCII艺术表示等。
// 本模块专注于牌的视觉呈现，提供多种显示风格以适应不同的输出环境。

use crate::tile::{Tile, Suit, Wind, Dragon, Flower};
use std::fmt;

/// 显示风格枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayStyle {
    /// 默认风格（中文）：1万，2条，东
    Default,
    /// 简洁风格：数字+花色首字母：1m, 2s, E
    Compact,
    /// Unicode符号风格：使用特殊符号表示
    Unicode,
    /// ASCII艺术风格：用ASCII字符绘制牌面
    Ascii,
}

/// 用于自定义显示风格的特质
pub trait TileDisplay {
    /// 按指定风格显示麻将牌
    fn display(&self, style: DisplayStyle) -> String;
    
    /// 显示一组牌（按指定风格）
    fn display_tiles(tiles: &[Tile], style: DisplayStyle) -> String {
        tiles.iter()
            .map(|tile| tile.display(style))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl TileDisplay for Tile {
    fn display(&self, style: DisplayStyle) -> String {
        match style {
            DisplayStyle::Default => self.to_string(),
            DisplayStyle::Compact => compact_display(self),
            DisplayStyle::Unicode => unicode_display(self),
            DisplayStyle::Ascii => ascii_display(self),
        }
    }
}

/// 简洁风格的牌面显示（如1m，2p，3s，E，C等）
fn compact_display(tile: &Tile) -> String {
    match tile {
        Tile::Suit(Suit::Character, n) => format!("{}m", n),
        Tile::Suit(Suit::Dot, n) => format!("{}p", n),
        Tile::Suit(Suit::Bamboo, n) => format!("{}s", n),
        Tile::Wind(Wind::East) => "E".to_string(),
        Tile::Wind(Wind::South) => "S".to_string(),
        Tile::Wind(Wind::West) => "W".to_string(),
        Tile::Wind(Wind::North) => "N".to_string(),
        Tile::Dragon(Dragon::White) => "W".to_string(),
        Tile::Dragon(Dragon::Green) => "G".to_string(),
        Tile::Dragon(Dragon::Red) => "R".to_string(),
        Tile::Flower(Flower::Spring) => "F1".to_string(),
        Tile::Flower(Flower::Summer) => "F2".to_string(),
        Tile::Flower(Flower::Autumn) => "F3".to_string(),
        Tile::Flower(Flower::Winter) => "F4".to_string(),
        Tile::Flower(Flower::Plum) => "F5".to_string(),
        Tile::Flower(Flower::Orchid) => "F6".to_string(),
        Tile::Flower(Flower::Bamboo) => "F7".to_string(),
        Tile::Flower(Flower::Chrysanthemum) => "F8".to_string(),
        Tile::Joker => "J".to_string(),
    }
}

/// Unicode符号风格的牌面显示
fn unicode_display(tile: &Tile) -> String {
    match tile {
        // 数牌 - 万
        Tile::Suit(Suit::Character, 1) => "🀇".to_string(),
        Tile::Suit(Suit::Character, 2) => "🀈".to_string(),
        Tile::Suit(Suit::Character, 3) => "🀉".to_string(),
        Tile::Suit(Suit::Character, 4) => "🀊".to_string(),
        Tile::Suit(Suit::Character, 5) => "🀋".to_string(),
        Tile::Suit(Suit::Character, 6) => "🀌".to_string(),
        Tile::Suit(Suit::Character, 7) => "🀍".to_string(),
        Tile::Suit(Suit::Character, 8) => "🀎".to_string(),
        Tile::Suit(Suit::Character, 9) => "🀏".to_string(),
        
        // 数牌 - 筒
        Tile::Suit(Suit::Dot, 1) => "🀙".to_string(),
        Tile::Suit(Suit::Dot, 2) => "🀚".to_string(),
        Tile::Suit(Suit::Dot, 3) => "🀛".to_string(),
        Tile::Suit(Suit::Dot, 4) => "🀜".to_string(),
        Tile::Suit(Suit::Dot, 5) => "🀝".to_string(),
        Tile::Suit(Suit::Dot, 6) => "🀞".to_string(),
        Tile::Suit(Suit::Dot, 7) => "🀟".to_string(),
        Tile::Suit(Suit::Dot, 8) => "🀠".to_string(),
        Tile::Suit(Suit::Dot, 9) => "🀡".to_string(),
        
        // 数牌 - 条
        Tile::Suit(Suit::Bamboo, 1) => "🀐".to_string(),
        Tile::Suit(Suit::Bamboo, 2) => "🀑".to_string(),
        Tile::Suit(Suit::Bamboo, 3) => "🀒".to_string(),
        Tile::Suit(Suit::Bamboo, 4) => "🀓".to_string(),
        Tile::Suit(Suit::Bamboo, 5) => "🀔".to_string(),
        Tile::Suit(Suit::Bamboo, 6) => "🀕".to_string(),
        Tile::Suit(Suit::Bamboo, 7) => "🀖".to_string(),
        Tile::Suit(Suit::Bamboo, 8) => "🀗".to_string(),
        Tile::Suit(Suit::Bamboo, 9) => "🀘".to_string(),
        
        // 风牌
        Tile::Wind(Wind::East) => "🀀".to_string(),
        Tile::Wind(Wind::South) => "🀁".to_string(),
        Tile::Wind(Wind::West) => "🀂".to_string(),
        Tile::Wind(Wind::North) => "🀃".to_string(),
        
        // 三元牌
        Tile::Dragon(Dragon::White) => "🀆".to_string(),
        Tile::Dragon(Dragon::Green) => "🀅".to_string(),
        Tile::Dragon(Dragon::Red) => "🀄".to_string(),
        
        // 花牌 (使用通用表示，因为Unicode没有专门的花牌符号)
        Tile::Flower(_) => "🎴".to_string(),
        
        // 百搭
        Tile::Joker => "🃟".to_string(),
        
        // 其他无效牌型
        _ => "?".to_string(),
    }
}

/// ASCII艺术风格的牌面显示
fn ascii_display(tile: &Tile) -> String {
    // 简单的ASCII牌面实现，可以根据需要扩展为多行艺术
    match tile {
        Tile::Suit(suit, n) => {
            let suit_char = match suit {
                Suit::Character => 'm',
                Suit::Dot => 'p',
                Suit::Bamboo => 's',
            };
            format!("|{}{}|", n, suit_char)
        },
        Tile::Wind(wind) => {
            let wind_char = match wind {
                Wind::East => 'E',
                Wind::South => 'S',
                Wind::West => 'W',
                Wind::North => 'N',
            };
            format!("|{}|", wind_char)
        },
        Tile::Dragon(dragon) => {
            let dragon_char = match dragon {
                Dragon::White => 'W',
                Dragon::Green => 'G',
                Dragon::Red => 'R',
            };
            format!("|{}|", dragon_char)
        },
        Tile::Flower(flower) => {
            let flower_num = match flower {
                Flower::Spring => 1,
                Flower::Summer => 2,
                Flower::Autumn => 3,
                Flower::Winter => 4,
                Flower::Plum => 5,
                Flower::Orchid => 6,
                Flower::Bamboo => 7,
                Flower::Chrysanthemum => 8,
            };
            format!("|F{}|", flower_num)
        },
        Tile::Joker => "|J|".to_string(),
    }
}

/// 用于在终端中显示带颜色的牌
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorStyle {
    /// 无色
    None,
    /// 使用ANSI颜色显示
    Ansi,
}

/// 提供带颜色显示牌的功能
pub trait ColoredTileDisplay: TileDisplay {
    /// 获取带颜色的牌面显示
    fn display_colored(&self, style: DisplayStyle, color_style: ColorStyle) -> String;
    
    /// 显示一组带颜色的牌
    fn display_tiles_colored(tiles: &[Tile], style: DisplayStyle, color_style: ColorStyle) -> String {
        tiles.iter()
            .map(|tile| tile.display_colored(style, color_style))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl ColoredTileDisplay for Tile {
    fn display_colored(&self, style: DisplayStyle, color_style: ColorStyle) -> String {
        if color_style == ColorStyle::None {
            return self.display(style);
        }
        
        // 获取基本显示文本
        let display_text = self.display(style);
        
        // 根据牌的类型添加颜色
        match self {
            Tile::Suit(Suit::Character, _) => format!("\x1b[31m{}\x1b[0m", display_text), // 红色
            Tile::Suit(Suit::Dot, _) => format!("\x1b[32m{}\x1b[0m", display_text),       // 绿色
            Tile::Suit(Suit::Bamboo, _) => format!("\x1b[34m{}\x1b[0m", display_text),    // 蓝色
            Tile::Wind(_) => format!("\x1b[36m{}\x1b[0m", display_text),                 // 青色
            Tile::Dragon(_) => format!("\x1b[35m{}\x1b[0m", display_text),               // 紫色
            Tile::Flower(_) => format!("\x1b[33m{}\x1b[0m", display_text),               // 黄色
            Tile::Joker => format!("\x1b[1;37m{}\x1b[0m", display_text),                 // 亮白色
        }
    }
}

/// 为牌组提供可打印的表格格式
pub struct TileGrid {
    tiles: Vec<Tile>,
    columns: usize,
    style: DisplayStyle,
    color_style: ColorStyle,
}

impl TileGrid {
    /// 创建新的牌组网格
    pub fn new(tiles: Vec<Tile>, columns: usize) -> Self {
        Self {
            tiles,
            columns,
            style: DisplayStyle::Default,
            color_style: ColorStyle::None,
        }
    }
    
    /// 设置显示风格
    pub fn with_style(mut self, style: DisplayStyle) -> Self {
        self.style = style;
        self
    }
    
    /// 设置颜色风格
    pub fn with_color(mut self, color_style: ColorStyle) -> Self {
        self.color_style = color_style;
        self
    }
}

impl fmt::Display for TileGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        
        for (i, chunk) in self.tiles.chunks(self.columns).enumerate() {
            if i > 0 {
                result.push('\n');
            }
            
            let row = chunk.iter()
                .map(|tile| {
                    if self.color_style == ColorStyle::None {
                        tile.display(self.style)
                    } else {
                        tile.display_colored(self.style, self.color_style)
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
                
            result.push_str(&row);
        }
        
        write!(f, "{}", result)
    }
}