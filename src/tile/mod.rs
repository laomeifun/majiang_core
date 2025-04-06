mod types;
mod tile;
mod display;

// 直接从 types 模块导出所需的类型
pub use types::{Tile, Suit, Wind, Dragon, Flower};

// 从 tile 模块导出其他公共项
// pub use tile::*;

// 从 display 模块导出显示相关功能
pub use display::{
    DisplayStyle, 
    TileDisplay, 
    ColorStyle, 
    ColoredTileDisplay, 
    TileGrid
};