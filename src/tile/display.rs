// src/tile/display.rs
//
// 为麻将牌提供多种显示格式，包括Unicode符号、ASCII艺术表示等。
// 本模块专注于牌的视觉呈现，采用特性（trait）来实现多样化的显示风格。
// 
// 设计思想：
// 1. 通过trait分离核心牌逻辑和显示逻辑，符合单一职责原则
// 2. 提供多种显示风格以适应不同的输出环境（终端、GUI、日志等）
// 3. 使用建造者模式（Builder pattern）简化表格布局配置

use crate::tile::{Tile, Suit, Wind, Dragon, Flower};
use std::fmt;

/// 显示风格枚举
/// 
/// 定义了多种牌面的显示格式，适应不同场景需求：
/// - 默认中文风格适合人类阅读
/// - 简洁风格适合紧凑显示和调试
/// - Unicode风格适合支持Unicode的现代终端
/// - ASCII风格确保在任何终端环境可用
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

/// 用于自定义显示风格的特质（trait）
/// 
/// 将显示逻辑从牌的核心实现中分离，遵循关注点分离原则。
/// 通过trait实现，允许未来扩展更多显示风格而无需修改核心Tile结构。
pub trait TileDisplay {
    /// 按指定风格显示麻将牌
    fn display(&self, style: DisplayStyle) -> String;
    
    /// 显示一组牌（按指定风格）
    /// 
    /// 这是一个默认实现，为使用方提供便利，避免重复编写遍历逻辑。
    /// 使用空格分隔每张牌，生成整个牌组的字符串表示。
    fn display_tiles(tiles: &[Tile], style: DisplayStyle) -> String {
        tiles.iter()
            .map(|tile| tile.display(style))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// 为Tile类型实现TileDisplay特质
/// 
/// 这里是核心的显示逻辑分发，根据请求的风格调用相应的显示函数。
/// 设计为match语句而非if-else链，以便编译器检查穷尽性，确保所有风格都有处理。
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
/// 
/// 采用国际象棋记谱法类似的简洁表示：
/// - 数字+花色首字母（m=万/man, p=筒/pin, s=索/sou）
/// - 单字母表示字牌（E=东, S=南, W=西, N=北）
/// - 字母表示三元牌（W=白, G=绿, R=红）
/// - F+数字表示花牌
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
/// 
/// 使用Unicode麻将区块（U+1F000-U+1F02F）中的符号表示麻将牌。
/// 优点是直观，一个符号即可表示一张牌，缺点是需要终端支持这些Unicode符号。
/// 
/// 注意：花牌使用通用Unicode符号，因为麻将Unicode区块不包含花牌专用符号。
fn unicode_display(tile: &Tile) -> String {
    // 使用静态映射表替代大量模式匹配，提高可维护性和性能
    // 这种实现更符合"避免频繁分配内存"和"函数短小，复杂逻辑拆分"的原则
    
    // 万子牌面映射 (1-9)
    static MAN_TILES: [&str; 9] = ["🀇", "🀈", "🀉", "🀊", "🀋", "🀌", "🀍", "🀎", "🀏"];
    
    // 筒子牌面映射 (1-9)
    static PIN_TILES: [&str; 9] = ["🀙", "🀚", "🀛", "🀜", "🀝", "🀞", "🀟", "🀠", "🀡"];
    
    // 索子牌面映射 (1-9)
    static SOU_TILES: [&str; 9] = ["🀐", "🀑", "🀒", "🀓", "🀔", "🀕", "🀖", "🀗", "🀘"];
    
    // 风牌映射 (东南西北)
    static WIND_TILES: [&str; 4] = ["🀀", "🀁", "🀂", "🀃"];
    
    // 三元牌映射 (白发中)
    static DRAGON_TILES: [&str; 3] = ["🀆", "🀅", "🀄"];
    
    match tile {
        Tile::Suit(suit, n) if *n >= 1 && *n <= 9 => {
            let index = (*n - 1) as usize;
            match suit {
                Suit::Character => MAN_TILES[index],
                Suit::Dot => PIN_TILES[index],
                Suit::Bamboo => SOU_TILES[index],
            }.to_string()
        },
        Tile::Wind(wind) => {
            let index = match wind {
                Wind::East => 0,
                Wind::South => 1,
                Wind::West => 2,
                Wind::North => 3,
            };
            WIND_TILES[index].to_string()
        },
        Tile::Dragon(dragon) => {
            let index = match dragon {
                Dragon::White => 0,
                Dragon::Green => 1,
                Dragon::Red => 2,
            };
            DRAGON_TILES[index].to_string()
        },
        Tile::Flower(_) => "🎴".to_string(),
        Tile::Joker => "🃟".to_string(),
        _ => "?".to_string(),
    }
}

/// ASCII艺术风格的牌面显示
/// 
/// 使用基本ASCII字符表示牌面，确保在任何终端环境都能正确显示。
/// 当前实现较为简单，可以根据需要扩展为更详细的多行ASCII艺术。
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
/// 
/// 颜色增强了牌面的可读性，特别是在复杂牌组中快速识别不同类型的牌。
/// 当前支持无色和ANSI颜色两种模式，可扩展支持其他颜色系统。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorStyle {
    /// 无色
    None,
    /// 使用ANSI颜色显示
    Ansi,
}

/// 提供带颜色显示牌的功能
/// 
/// 扩展了TileDisplay，增加了颜色支持。
/// 设计为特质继承（trait inheritance）模式，确保实现ColoredTileDisplay
/// 的类型必须也实现基本的TileDisplay。
pub trait ColoredTileDisplay: TileDisplay {
    /// 获取带颜色的牌面显示
    fn display_colored(&self, style: DisplayStyle, color_style: ColorStyle) -> String;
    
    /// 显示一组带颜色的牌
    /// 
    /// 类似于TileDisplay中的display_tiles，为批量操作提供便利。
    /// 这种默认实现减少了重复代码，同时保持了接口的一致性。
    fn display_tiles_colored(tiles: &[Tile], style: DisplayStyle, color_style: ColorStyle) -> String {
        tiles.iter()
            .map(|tile| tile.display_colored(style, color_style))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// 为Tile实现ColoredTileDisplay特质
/// 
/// 不同牌型使用不同颜色表示，增强可读性：
/// - 万子：红色
/// - 筒子：绿色
/// - 索子：蓝色
/// - 风牌：青色
/// - 三元牌：紫色
/// - 花牌：黄色
/// - 百搭：亮白色
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
/// 
/// 用于将一组牌按指定列数排列显示，特别适合展示手牌、牌山等。
/// 实现了Builder模式，可链式调用配置显示风格和颜色。
pub struct TileGrid {
    tiles: Vec<Tile>,
    columns: usize,
    style: DisplayStyle,
    color_style: ColorStyle,
}

impl TileGrid {
    /// 创建新的牌组网格
    /// 
    /// # 参数
    /// * `tiles` - 要显示的牌组
    /// * `columns` - 每行显示的牌数
    pub fn new(tiles: Vec<Tile>, columns: usize) -> Self {
        Self {
            tiles,
            columns,
            style: DisplayStyle::Default,
            color_style: ColorStyle::None,
        }
    }
    
    /// 设置显示风格
    /// 
    /// 链式调用模式，使配置过程更直观。
    /// 
    /// # 参数
    /// * `style` - 显示风格
    /// 
    /// # 返回
    /// 配置后的TileGrid实例
    pub fn with_style(mut self, style: DisplayStyle) -> Self {
        self.style = style;
        self
    }
    
    /// 设置颜色风格
    /// 
    /// 链式调用模式，使配置过程更直观。
    /// 
    /// # 参数
    /// * `color_style` - 颜色风格
    /// 
    /// # 返回
    /// 配置后的TileGrid实例
    pub fn with_color(mut self, color_style: ColorStyle) -> Self {
        self.color_style = color_style;
        self
    }
}

/// 实现Display特性，使TileGrid可以直接打印
/// 
/// 将牌组按指定列数排列，支持多行显示，并应用配置的显示风格和颜色。
/// 这种实现允许直接使用println!("{}", grid)或format!("{}", grid)等。
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