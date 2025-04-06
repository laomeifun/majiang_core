// src/meld/utils.rs
// (可选) Meld 相关辅助函数 (如显示, 包含判断)

use super::types::Meld;
use crate::tile::Tile;

// TODO: 添加 Meld 相关的辅助函数

/// 示例：检查一组副露中是否包含特定的牌
pub fn check_if_melds_contain(melds: &[Meld], tile: &Tile) -> bool {
    melds.iter().any(|meld| meld.contains(tile))
}

// 可以添加更多函数，例如：
// - 将 Meld 转换为字符串表示 (虽然 Display trait 已经做了基础实现)
// - 计算副露占用的牌数
// - 检查副露的有效性等
