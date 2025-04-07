// 牌墙构建器模块
// 负责按照指定规则初始化麻将牌，洗牌并构建初始牌墙

use rand::prelude::*;
use crate::tile::{Tile, Suit, Wind, Dragon, Flower};
use crate::errors::{MajiangError, Result};

/// 表示不同类型的麻将牌组构成
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallConfig {
    /// 日本麻将: 无花牌，四副基本牌(万、筒、条、字牌)，共136张
    Riichi,
    
    /// 中国官方规则(MCR): 有花牌，四副基本牌，共144张
    MCR,
    
    /// 上海麻将: 有花牌，可能还有百搭牌，根据配置可能有其他特殊牌
    Shanghai { 
        /// 是否包含百搭牌
        with_joker: bool 
    },
    
    /// 自定义配置: 允许灵活指定各类牌的数量
    Custom {
        /// 花牌数量(0-8)
        flowers: u8,
        /// 是否包含百搭牌
        with_joker: bool,
        /// 每种数牌的副数(万、筒、条，通常为4)
        suit_sets: u8,
        /// 字牌的副数(东南西北中发白，通常为4)
        honor_sets: u8,
    },
}

impl Default for WallConfig {
    fn default() -> Self {
        WallConfig::MCR
    }
}

/// 构建指定配置的完整麻将牌集
/// 
/// # 参数
/// * `config` - 牌墙配置，指定使用哪种规则的牌组
/// 
/// # 返回值
/// * `Result<Vec<Tile>>` - 成功则返回构建好的牌集，失败则返回错误
pub fn build_tiles(config: WallConfig) -> Result<Vec<Tile>> {
    match config {
        WallConfig::Riichi => build_riichi_tiles(),
        WallConfig::MCR => build_mcr_tiles(),
        WallConfig::Shanghai { with_joker } => build_shanghai_tiles(with_joker),
        WallConfig::Custom { flowers, with_joker, suit_sets, honor_sets } => {
            build_custom_tiles(flowers, with_joker, suit_sets, honor_sets)
        }
    }
}

/// 构建日本麻将使用的牌组(无花牌)
fn build_riichi_tiles() -> Result<Vec<Tile>> {
    let mut tiles = Vec::with_capacity(136);
    
    // 添加4副数牌: 万子、筒子、索子
    add_suit_tiles(&mut tiles, 4);
    
    // 添加4副字牌: 风牌(东南西北)和三元牌(中发白)
    add_honor_tiles(&mut tiles, 4);
    
    Ok(tiles)
}

/// 构建中国官方麻将(MCR)使用的牌组(含花牌)
fn build_mcr_tiles() -> Result<Vec<Tile>> {
    let mut tiles = Vec::with_capacity(144);
    
    // 添加4副数牌: 万子、筒子、索子
    add_suit_tiles(&mut tiles, 4);
    
    // 添加4副字牌: 风牌(东南西北)和三元牌(中发白)
    add_honor_tiles(&mut tiles, 4);
    
    // 添加8张花牌: 春夏秋冬梅兰竹菊
    add_flower_tiles(&mut tiles);
    
    Ok(tiles)
}

/// 构建上海麻将使用的牌组(含花牌，可选百搭)
fn build_shanghai_tiles(with_joker: bool) -> Result<Vec<Tile>> {
    let mut tiles = Vec::with_capacity(if with_joker { 145 } else { 144 });
    
    // 添加4副数牌: 万子、筒子、索子
    add_suit_tiles(&mut tiles, 4);
    
    // 添加4副字牌: 风牌(东南西北)和三元牌(中发白)
    add_honor_tiles(&mut tiles, 4);
    
    // 添加8张花牌: 春夏秋冬梅兰竹菊
    add_flower_tiles(&mut tiles);
    
    // 如果需要，添加百搭牌
    if with_joker {
        tiles.push(Tile::Joker);
    }
    
    Ok(tiles)
}

/// 构建自定义配置的牌组
fn build_custom_tiles(flowers: u8, with_joker: bool, suit_sets: u8, honor_sets: u8) -> Result<Vec<Tile>> {
    if flowers > 8 {
        return Err(MajiangError::InvalidTileCount("花牌数量不能超过8张".to_string()));
    }
    
    // 计算容量: 基本牌 + 花牌 + 可能的百搭
    let capacity = (suit_sets as usize * 3 * 9) + (honor_sets as usize * 7) + (flowers as usize) + (with_joker as usize);
    let mut tiles = Vec::with_capacity(capacity);
    
    // 添加数牌
    add_suit_tiles(&mut tiles, suit_sets);
    
    // 添加字牌
    add_honor_tiles(&mut tiles, honor_sets);
    
    // 添加花牌
    add_partial_flower_tiles(&mut tiles, flowers);
    
    // 如果需要，添加百搭牌
    if with_joker {
        tiles.push(Tile::Joker);
    }
    
    Ok(tiles)
}

/// 添加指定副数的数牌(万子、筒子、索子)到牌集
fn add_suit_tiles(tiles: &mut Vec<Tile>, sets: u8) {
    let suits = [Suit::Character, Suit::Dot, Suit::Bamboo];
    
    for &suit in &suits {
        for _ in 0..sets {
            for n in 1..=9 {
                if let Some(tile) = Tile::new_suit(suit, n) {
                    tiles.push(tile);
                }
            }
        }
    }
}

/// 添加指定副数的字牌(风牌和三元牌)到牌集
fn add_honor_tiles(tiles: &mut Vec<Tile>, sets: u8) {
    // 添加风牌: 东南西北
    let winds = [Wind::East, Wind::South, Wind::West, Wind::North];
    for &wind in &winds {
        for _ in 0..sets {
            tiles.push(Tile::Wind(wind));
        }
    }
    
    // 添加三元牌: 中发白
    let dragons = [Dragon::Red, Dragon::Green, Dragon::White];
    for &dragon in &dragons {
        for _ in 0..sets {
            tiles.push(Tile::Dragon(dragon));
        }
    }
}

/// 添加所有8张花牌到牌集
fn add_flower_tiles(tiles: &mut Vec<Tile>) {
    // 季节牌: 春夏秋冬
    tiles.push(Tile::Flower(Flower::Spring));
    tiles.push(Tile::Flower(Flower::Summer));
    tiles.push(Tile::Flower(Flower::Autumn));
    tiles.push(Tile::Flower(Flower::Winter));
    
    // 花牌: 梅兰竹菊
    tiles.push(Tile::Flower(Flower::Plum));
    tiles.push(Tile::Flower(Flower::Orchid));
    tiles.push(Tile::Flower(Flower::Bamboo));
    tiles.push(Tile::Flower(Flower::Chrysanthemum));
}

/// 添加指定数量的花牌到牌集
fn add_partial_flower_tiles(tiles: &mut Vec<Tile>, count: u8) {
    let all_flowers = [
        Flower::Spring, Flower::Summer, Flower::Autumn, Flower::Winter,
        Flower::Plum, Flower::Orchid, Flower::Bamboo, Flower::Chrysanthemum
    ];
    
    for i in 0..(count as usize) {
        if i < all_flowers.len() {
            tiles.push(Tile::Flower(all_flowers[i]));
        }
    }
}

/// 洗牌函数，随机打乱牌的顺序
/// 
/// # 参数
/// * `tiles` - 要洗牌的牌集合
/// * `rng` - 随机数生成器
pub fn shuffle_tiles<R: Rng>(tiles: &mut [Tile], rng: &mut R) {
    tiles.shuffle(rng);
}

/// 创建一个洗好牌的牌墙
/// 
/// # 参数
/// * `config` - 牌墙配置
/// * `seed` - 可选的随机数种子，用于测试或复现
/// 
/// # 返回值
/// * `Result<Vec<Tile>>` - 洗好牌的麻将牌墙
pub fn create_shuffled_tiles(config: WallConfig, seed: Option<u64>) -> Result<Vec<Tile>> {
    let mut tiles = build_tiles(config)?;
    
    match seed {
        Some(seed_value) => {
            // 使用提供的种子创建随机数生成器
            let mut rng = StdRng::seed_from_u64(seed_value);
            shuffle_tiles(&mut tiles, &mut rng);
        }
        None => {
            // 使用系统随机源
            let mut rng = rand::thread_rng();
            shuffle_tiles(&mut tiles, &mut rng);
        }
    }
    
    Ok(tiles)
}