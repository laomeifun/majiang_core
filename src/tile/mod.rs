// src/tile/mod.rs
//
// 麻将牌模块，定义了麻将牌的核心数据结构和基本操作。
// 本模块是麻将游戏的基础，提供了所有与麻将牌相关的类型和功能。
// 模块结构设计遵循"关注点分离"原则：
// - types.rs: 定义基本牌型枚举和常量
// - tile.rs: 实现牌的核心操作和判断
// - display.rs: 负责牌的可视化表示
// - serialization.rs: 提供序列化辅助方法

mod types;
mod tile;
mod display;
mod serialization;

// 直接从 types 模块导出所需的类型
pub use types::{Tile, Suit, Wind, Dragon, Flower};

// 从 display 模块导出显示相关功能
pub use display::{
    DisplayStyle, 
    TileDisplay, 
    ColorStyle, 
    ColoredTileDisplay, 
    TileGrid
};

// 从 serialization 模块导出序列化辅助功能
pub use serialization::{
    TileId, TileData,
    to_id, from_id,
    to_data, from_data,
    tiles_to_ids, ids_to_tiles
};