// src/tile/mod.rs

// 声明子模块
mod types;
mod tile;
mod display;

// 重新导出需要公开的类型和函数
pub use types::{Suit, Wind, Dragon, Flower};
pub use tile::Tile;

// 如果需要，可以导出特定的常量，但通常枚举和 Tile 结构体就足够了
// pub use types::{MAN_START, PIN_START, ...};
