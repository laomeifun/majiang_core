// src/tile/serialization.rs
//
// 提供麻将牌序列化辅助方法，使外部实现更方便。
// 本模块不执行实际的序列化，而是提供必要的转换函数，
// 使外部库可以更容易地实现自己的序列化逻辑。

use crate::tile::{Tile, Suit, Wind, Dragon, Flower};

/// 表示牌的简单整数形式，便于序列化和网络传输
/// 直接使用牌的ID值
pub type TileId = u8;

/// 将牌转换为整数ID（适合序列化）
/// 
/// 这个函数是Tile::to_id()的公开包装，外部可以用于序列化过程。
/// ID范围：
/// - 0-8:   一万到九万
/// - 9-17:  一筒到九筒
/// - 18-26: 一条到九条
/// - 27-30: 东南西北
/// - 31-33: 白发中
/// - 34-41: 花牌
/// - 42:    百搭牌
/// 
/// # 示例
/// ```
/// let tile = Tile::Suit(Suit::Character, 5); // 五万
/// let id = to_id(&tile); // id = 4
/// ```
pub fn to_id(tile: &Tile) -> TileId {
    tile.to_id()
}

/// 从整数ID创建牌（反序列化用）
/// 
/// 是Tile::from_id()的公开包装，用于从序列化的ID恢复牌对象。
/// 如果ID无效，返回None。
/// 
/// # 示例
/// ```
/// let id = 4; // 五万的ID
/// if let Some(tile) = from_id(id) {
///     println!("恢复的牌: {}", tile); // 显示"5万"
/// }
/// ```
pub fn from_id(id: TileId) -> Option<Tile> {
    Tile::from_id(id)
}

/// 牌的序列化表示结构
/// 
/// 提供一个符合Rust惯用法的结构来表示牌的数据。
/// 此结构易于序列化，可用作到JSON、MessagePack等格式的中间表示。
/// 
/// 注意：此结构设计为序列化友好，包含足够信息重建原始Tile。
/// 外部库可以直接使用此结构实现自己的序列化逻辑。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TileData {
    /// 牌的唯一ID
    pub id: TileId,
    /// 牌的类型（字符串形式，如"万子"、"风牌"等）
    pub kind: String,
    /// 牌的具体值（适用于数牌和字牌）
    pub value: String,
    /// 是否为红牌
    pub is_red: bool,
}

/// 将牌转换为序列化友好的数据结构
/// 
/// 为第三方序列化库提供便利，避免直接处理复杂的枚举。
/// 生成的TileData包含足够信息重建原始Tile对象。
/// 
/// # 示例
/// ```
/// let tile = Tile::Wind(Wind::East);
/// let data = to_data(&tile);
/// // data.kind = "风牌", data.value = "东", data.id = 27
/// ```
pub fn to_data(tile: &Tile) -> TileData {
    let id = to_id(tile);
    let is_red = tile.is_red();
    
    match tile {
        Tile::Suit(suit, num) => {
            let kind = match suit {
                Suit::Character => "万子",
                Suit::Dot => "筒子",
                Suit::Bamboo => "索子",
            };
            TileData {
                id,
                kind: kind.to_string(),
                value: num.to_string(),
                is_red,
            }
        },
        Tile::Wind(wind) => {
            let value = match wind {
                Wind::East => "东",
                Wind::South => "南",
                Wind::West => "西",
                Wind::North => "北",
            };
            TileData {
                id,
                kind: "风牌".to_string(),
                value: value.to_string(),
                is_red,
            }
        },
        Tile::Dragon(dragon) => {
            let value = match dragon {
                Dragon::White => "白",
                Dragon::Green => "发",
                Dragon::Red => "中",
            };
            TileData {
                id,
                kind: "三元牌".to_string(),
                value: value.to_string(),
                is_red,
            }
        },
        Tile::Flower(flower) => {
            let value = match flower {
                Flower::Spring => "春",
                Flower::Summer => "夏",
                Flower::Autumn => "秋",
                Flower::Winter => "冬",
                Flower::Plum => "梅",
                Flower::Orchid => "兰",
                Flower::Bamboo => "竹",
                Flower::Chrysanthemum => "菊",
            };
            TileData {
                id,
                kind: "花牌".to_string(),
                value: value.to_string(),
                is_red,
            }
        },
        Tile::Joker => TileData {
            id,
            kind: "百搭".to_string(),
            value: "百搭".to_string(),
            is_red,
        },
    }
}

/// 从序列化数据重建牌对象
/// 
/// 主要使用id字段来重建，其他字段作为备用或验证。
/// 如果ID无效，返回None。
/// 
/// # 示例
/// ```
/// let data = TileData {
///     id: 27,
///     kind: "风牌".to_string(),
///     value: "东".to_string(),
///     is_red: false,
/// };
/// if let Some(tile) = from_data(&data) {
///     println!("重建的牌: {}", tile); // 显示"东"
/// }
/// ```
pub fn from_data(data: &TileData) -> Option<Tile> {
    from_id(data.id)
}

/// 创建一组牌的ID序列，适合批量序列化
/// 
/// 便于网络传输或存储整组牌的快捷方法。
/// 
/// # 示例
/// ```
/// let tiles = vec![
///     Tile::Suit(Suit::Character, 1),
///     Tile::Suit(Suit::Character, 2),
///     Tile::Wind(Wind::East),
/// ];
/// let ids = tiles_to_ids(&tiles); // [0, 1, 27]
/// ```
pub fn tiles_to_ids(tiles: &[Tile]) -> Vec<TileId> {
    tiles.iter().map(|t| to_id(t)).collect()
}

/// 从ID序列重建一组牌，适合批量反序列化
/// 
/// 如果任何ID无效，对应位置返回None，不影响其他牌的重建。
/// 
/// # 示例
/// ```
/// let ids = vec![0, 1, 27];
/// let tiles = ids_to_tiles(&ids);
/// // tiles包含[一万, 二万, 东]
/// ```
pub fn ids_to_tiles(ids: &[TileId]) -> Vec<Option<Tile>> {
    ids.iter().map(|&id| from_id(id)).collect()
}