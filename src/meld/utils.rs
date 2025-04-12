// src/meld/utils.rs
//
// 提供与副露相关的工具函数，包括验证、检查和组合生成等功能
// 这些函数是麻将游戏中处理副露操作的核心逻辑

use crate::errors::{MajiangError, MajiangResult};
use crate::meld::{Meld, MeldType, KanType, MeldSource};
use crate::tile::Tile;
use std::collections::HashSet;

/// 验证吃的有效性
/// 
/// 吃必须满足以下条件：
/// 1. 三张牌必须是同一花色的数牌
/// 2. 三张牌必须连续(如2-3-4筒)
/// 
/// # 参数
/// * `tiles` - 要验证的三张牌
/// 
/// # 返回
/// * `MajiangResult<()>` - 验证成功返回Ok(())，否则返回具体错误
pub fn validate_chi(tiles: &[Tile]) -> MajiangResult<()> {
    if tiles.len() != 3 {
        return Err(MajiangError::InvalidMeld("吃必须由三张牌组成".to_string()));
    }

    // 先确保所有牌都是数牌
    if !tiles.iter().all(|t| t.is_suit()) {
        return Err(MajiangError::InvalidMeld("吃只能由数牌组成".to_string()));
    }

    // 提取花色和点数
    let mut suits = Vec::new();
    let mut numbers = Vec::new();
    
    for tile in tiles {
        if let Tile::Suit(suit, num) = *tile {
            suits.push(suit);
            numbers.push(num);
        }
    }

    // 检查花色是否一致
    if suits.iter().any(|&s| s != suits[0]) {
        return Err(MajiangError::InvalidMeld("吃必须是同一花色".to_string()));
    }

    // 排序点数，然后检查是否连续
    numbers.sort_unstable();
    for i in 1..numbers.len() {
        if numbers[i] != numbers[i-1] + 1 {
            return Err(MajiangError::InvalidMeld("吃的牌必须是连续的".to_string()));
        }
    }

    Ok(())
}

/// 验证碰的有效性
/// 
/// 碰必须满足以下条件：
/// 1. 三张相同的牌
/// 
/// # 参数
/// * `tiles` - 要验证的三张牌
/// 
/// # 返回
/// * `MajiangResult<()>` - 验证成功返回Ok(())，否则返回具体错误
pub fn validate_pon(tiles: &[Tile]) -> MajiangResult<()> {
    if tiles.len() != 3 {
        return Err(MajiangError::InvalidMeld("碰必须由三张牌组成".to_string()));
    }

    // 检查所有牌是否相同
    let first_tile = tiles[0];
    if !tiles.iter().all(|&t| t == first_tile) {
        return Err(MajiangError::InvalidMeld("碰必须由三张相同的牌组成".to_string()));
    }

    Ok(())
}

/// 验证杠的有效性
/// 
/// 杠必须满足以下条件：
/// 1. 四张相同的牌
/// 2. 杠的类型必须与来源匹配
/// 
/// # 参数
/// * `tiles` - 要验证的四张牌
/// * `kan_type` - 杠的类型(明杠、暗杠、加杠)
/// * `sources` - 牌的来源
/// 
/// # 返回
/// * `MajiangResult<()>` - 验证成功返回Ok(())，否则返回具体错误
pub fn validate_kan(tiles: &[Tile], kan_type: KanType, sources: &[MeldSource]) -> MajiangResult<()> {
    if tiles.len() != 4 {
        return Err(MajiangError::InvalidMeld("杠必须由四张牌组成".to_string()));
    }

    // 检查所有牌是否相同
    let first_tile = tiles[0];
    if !tiles.iter().all(|&t| t == first_tile) {
        return Err(MajiangError::InvalidMeld("杠必须由四张相同的牌组成".to_string()));
    }

    // 检查杠的类型与来源是否匹配
    match kan_type {
        KanType::Open => {
            // 明杠需要有一张来自其他玩家
            if !sources.iter().any(|&s| matches!(s, MeldSource::Player(_))) {
                return Err(MajiangError::InvalidMeld("明杠必须有一张牌来自其他玩家".to_string()));
            }
        },
        KanType::Closed => {
            // 暗杠所有牌必须来自自己
            if !sources.iter().all(|&s| s == MeldSource::SelfDrawn) {
                return Err(MajiangError::InvalidMeld("暗杠的所有牌必须来自自己".to_string()));
            }
        },
        KanType::Added => {
            // 加杠通常不在这里验证，因为需要知道之前的碰的信息
            // 这里只做简单检查
            if sources.iter().filter(|&&s| s == MeldSource::SelfDrawn).count() != 1 {
                return Err(MajiangError::InvalidMeld("加杠应该由之前的碰加上自己新摸到的一张牌构成".to_string()));
            }
        },
    }

    Ok(())
}

/// 将副露中的牌排序
/// 
/// 对于吃，按照点数顺序排序
/// 对于碰和杠，保持原有顺序(因为所有牌都相同)
/// 
/// # 参数
/// * `meld` - 要排序的副露
/// 
/// # 返回
/// * `Meld` - 排序后的副露
pub fn sort_tiles_in_meld(meld: &Meld) -> Meld {
    match meld.meld_type {
        MeldType::Chi => {
            // 为吃按照点数排序
            let mut sorted_pairs: Vec<(Tile, MeldSource)> = meld.tiles.iter()
                .zip(meld.sources.iter())
                .map(|(&t, &s)| (t, s))
                .collect();

            // 按点数排序
            sorted_pairs.sort_by(|(a, _), (b, _)| {
                if let (Tile::Suit(_, a_num), Tile::Suit(_, b_num)) = (*a, *b) {
                    a_num.cmp(&b_num)
                } else {
                    std::cmp::Ordering::Equal // 理论上不会走到这里，因为吃只能由数牌组成
                }
            });

            // 拆分回tiles和sources
            let tiles = sorted_pairs.iter().map(|(t, _)| *t).collect();
            let sources = sorted_pairs.iter().map(|(_, s)| *s).collect();

            Meld {
                tiles,
                meld_type: MeldType::Chi,
                sources,
            }
        },
        _ => meld.clone(), // 对于碰和杠，不需要排序因为所有牌都一样
    }
}

/// 检查是否可以形成吃
/// 
/// # 参数
/// * `hand_tiles` - 手中的牌
/// * `discarded_tile` - 要吃的牌(通常是别人打出的牌)
/// 
/// # 返回
/// * `bool` - 是否可以吃
pub fn can_form_chi(hand_tiles: &[Tile], discarded_tile: Tile) -> bool {
    // 只有数牌能吃
    if !discarded_tile.is_suit() {
        return false;
    }

    // 提取discarded_tile的花色和点数
    if let Tile::Suit(suit, number) = discarded_tile {
        let hand_set: HashSet<_> = hand_tiles.iter().copied().collect();
        
        // 检查三种可能的吃牌组合
        // 例: 如果打出3筒，检查是否有(1筒,2筒)、(2筒,4筒)或(4筒,5筒)
        
        // 组合1: n-2, n-1, n
        if number >= 3 {
            let need_1 = Tile::new_suit(suit, number - 2);
            let need_2 = Tile::new_suit(suit, number - 1);
            if let (Some(n1), Some(n2)) = (need_1, need_2) {
                if hand_set.contains(&n1) && hand_set.contains(&n2) {
                    return true;
                }
            }
        }
        
        // 组合2: n-1, n, n+1
        if number >= 2 && number <= 8 {
            let need_1 = Tile::new_suit(suit, number - 1);
            let need_2 = Tile::new_suit(suit, number + 1);
            if let (Some(n1), Some(n2)) = (need_1, need_2) {
                if hand_set.contains(&n1) && hand_set.contains(&n2) {
                    return true;
                }
            }
        }
        
        // 组合3: n, n+1, n+2
        if number <= 7 {
            let need_1 = Tile::new_suit(suit, number + 1);
            let need_2 = Tile::new_suit(suit, number + 2);
            if let (Some(n1), Some(n2)) = (need_1, need_2) {
                if hand_set.contains(&n1) && hand_set.contains(&n2) {
                    return true;
                }
            }
        }
    }
    
    false
}

/// 检查是否可以形成碰
/// 
/// # 参数
/// * `hand_tiles` - 手中的牌
/// * `discarded_tile` - 要碰的牌(通常是别人打出的牌)
/// 
/// # 返回
/// * `bool` - 是否可以碰
pub fn can_form_pon(hand_tiles: &[Tile], discarded_tile: Tile) -> bool {
    // 计算手中有多少张与discarded_tile相同的牌
    hand_tiles.iter().filter(|&&t| t == discarded_tile).count() >= 2
}

/// 检查是否可以形成杠
/// 
/// # 参数
/// * `hand_tiles` - 手中的牌
/// * `discarded_tile` - 要杠的牌(可能是别人打出的，也可能是自己摸到的)
/// * `is_self_drawn` - 是否为自摸杠
/// 
/// # 返回
/// * `bool` - 是否可以杠
pub fn can_form_kan(hand_tiles: &[Tile], discarded_tile: Tile, is_self_drawn: bool) -> bool {
    let same_count = hand_tiles.iter().filter(|&&t| t == discarded_tile).count();
    
    if is_self_drawn {
        // 自摸杠，需要手里有4张相同的牌(包括刚摸到的那张)
        same_count >= 3
    } else {
        // 明杠，需要手里有3张相同的牌
        same_count >= 3
    }
}

/// 获取可能的吃组合
/// 
/// # 参数
/// * `hand_tiles` - 手中的牌
/// * `discarded_tile` - 要吃的牌(通常是别人打出的牌)
/// 
/// # 返回
/// * `Vec<Vec<Tile>>` - 所有可能的吃组合，每个内部Vec包含组成一个吃的三张牌
pub fn get_possible_chi_combinations(hand_tiles: &[Tile], discarded_tile: Tile) -> Vec<Vec<Tile>> {
    let mut combinations = Vec::new();

    if !discarded_tile.is_suit() {
        return combinations;
    }

    if let Tile::Suit(suit, number) = discarded_tile {
        let hand_set: HashSet<_> = hand_tiles.iter().copied().collect();
        
        // 组合1: n-2, n-1, n
        if number >= 3 {
            let need_1 = Tile::new_suit(suit, number - 2);
            let need_2 = Tile::new_suit(suit, number - 1);
            if let (Some(n1), Some(n2)) = (need_1, need_2) {
                if hand_set.contains(&n1) && hand_set.contains(&n2) {
                    combinations.push(vec![n1, n2, discarded_tile]);
                }
            }
        }
        
        // 组合2: n-1, n, n+1
        if number >= 2 && number <= 8 {
            let need_1 = Tile::new_suit(suit, number - 1);
            let need_2 = Tile::new_suit(suit, number + 1);
            if let (Some(n1), Some(n2)) = (need_1, need_2) {
                if hand_set.contains(&n1) && hand_set.contains(&n2) {
                    combinations.push(vec![n1, discarded_tile, n2]);
                }
            }
        }
        
        // 组合3: n, n+1, n+2
        if number <= 7 {
            let need_1 = Tile::new_suit(suit, number + 1);
            let need_2 = Tile::new_suit(suit, number + 2);
            if let (Some(n1), Some(n2)) = (need_1, need_2) {
                if hand_set.contains(&n1) && hand_set.contains(&n2) {
                    combinations.push(vec![discarded_tile, n1, n2]);
                }
            }
        }
    }
    
    combinations
}

/// 获取杠中包含多少张指定的牌
/// 
/// # 参数
/// * `meld` - 杠副露
/// * `tile` - 要检查的牌
/// 
/// # 返回
/// * `usize` - 杠中包含的指定牌的数量
pub fn get_kan_tile_count(meld: &Meld, tile: Tile) -> usize {
    if let MeldType::Kan(_) = meld.meld_type {
        meld.tiles.iter().filter(|&&t| t == tile).count()
    } else {
        0
    }
}

/// 获取杠中的所有牌
/// 
/// # 参数
/// * `meld` - 杠副露
/// 
/// # 返回
/// * `Option<Vec<Tile>>` - 杠中的牌，如果不是杠则返回None
pub fn get_kan_tiles(meld: &Meld) -> Option<Vec<Tile>> {
    match meld.meld_type {
        MeldType::Kan(_) => Some(meld.tiles.clone()),
        _ => None,
    }
}