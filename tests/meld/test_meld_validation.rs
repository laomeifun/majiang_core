// tests/meld/test_meld_validation.rs
//
// 测试副露的验证功能，确保吃、碰、杠的规则被正确验证

use majiang_core::errors::MajiangResult;
use majiang_core::meld::{KanType, MeldSource, MeldType};
use majiang_core::meld::utils::{validate_chi, validate_pon, validate_kan};
use majiang_core::tile::{Suit, Tile};

/// 测试有效的吃验证
#[test]
fn test_validate_valid_chi() {
    // 有效的连续三张牌：1万、2万、3万
    let tiles = vec![
        Tile::new_suit(Suit::Character, 1).unwrap(),
        Tile::new_suit(Suit::Character, 2).unwrap(),
        Tile::new_suit(Suit::Character, 3).unwrap(),
    ];
    
    let result = validate_chi(&tiles);
    assert!(result.is_ok());
}

/// 测试无效的吃验证 - 不同花色
#[test]
fn test_validate_invalid_chi_different_suits() {
    // 无效的吃组合：1万、2筒、3万
    let tiles = vec![
        Tile::new_suit(Suit::Character, 1).unwrap(),
        Tile::new_suit(Suit::Dot, 2).unwrap(),       // 不同花色
        Tile::new_suit(Suit::Character, 3).unwrap(),
    ];
    
    let result = validate_chi(&tiles);
    assert!(result.is_err());
}

/// 测试无效的吃验证 - 不连续的牌
#[test]
fn test_validate_invalid_chi_non_sequential() {
    // 无效的吃组合：1万、2万、4万（跳跃了3万）
    let tiles = vec![
        Tile::new_suit(Suit::Character, 1).unwrap(),
        Tile::new_suit(Suit::Character, 2).unwrap(),
        Tile::new_suit(Suit::Character, 4).unwrap(), // 不连续
    ];
    
    let result = validate_chi(&tiles);
    assert!(result.is_err());
}

/// 测试无效的吃验证 - 包含字牌
#[test]
fn test_validate_invalid_chi_with_honor() {
    // 无效的吃组合：包含字牌
    let tiles = vec![
        Tile::new_suit(Suit::Character, 1).unwrap(),
        Tile::new_suit(Suit::Character, 2).unwrap(),
        Tile::Wind(majiang_core::tile::Wind::East),  // 字牌不能吃
    ];
    
    let result = validate_chi(&tiles);
    assert!(result.is_err());
}

/// 测试有效的碰验证
#[test]
fn test_validate_valid_pon() {
    // 有效的碰组合：三张相同的牌
    let tiles = vec![
        Tile::new_suit(Suit::Bamboo, 5).unwrap(),
        Tile::new_suit(Suit::Bamboo, 5).unwrap(),
        Tile::new_suit(Suit::Bamboo, 5).unwrap(),
    ];
    
    let result = validate_pon(&tiles);
    assert!(result.is_ok());
}

/// 测试有效的碰验证 - 字牌
#[test]
fn test_validate_valid_pon_honor() {
    // 有效的碰组合：三张相同的字牌
    let east = Tile::Wind(majiang_core::tile::Wind::East);
    let tiles = vec![east, east, east];
    
    let result = validate_pon(&tiles);
    assert!(result.is_ok());
}

/// 测试无效的碰验证 - 不同的牌
#[test]
fn test_validate_invalid_pon() {
    // 无效的碰组合：不完全相同的牌
    let tiles = vec![
        Tile::new_suit(Suit::Bamboo, 5).unwrap(),
        Tile::new_suit(Suit::Bamboo, 5).unwrap(),
        Tile::new_suit(Suit::Bamboo, 6).unwrap(), // 不同的点数
    ];
    
    let result = validate_pon(&tiles);
    assert!(result.is_err());
}

/// 测试有效的明杠验证
#[test]
fn test_validate_valid_open_kan() {
    // 四张相同的牌
    let tiles = vec![
        Tile::new_suit(Suit::Character, 9).unwrap(),
        Tile::new_suit(Suit::Character, 9).unwrap(),
        Tile::new_suit(Suit::Character, 9).unwrap(),
        Tile::new_suit(Suit::Character, 9).unwrap(),
    ];
    
    // 其中一张来自其他玩家
    let sources = vec![
        MeldSource::SelfDrawn,
        MeldSource::SelfDrawn,
        MeldSource::SelfDrawn,
        MeldSource::Player(2),
    ];
    
    let result = validate_kan(&tiles, KanType::Open, &sources);
    assert!(result.is_ok());
}

/// 测试有效的暗杠验证
#[test]
fn test_validate_valid_closed_kan() {
    // 四张相同的牌
    let white = Tile::Dragon(majiang_core::tile::Dragon::White);
    let tiles = vec![white; 4];
    
    // 所有牌都是自己的
    let sources = vec![MeldSource::SelfDrawn; 4];
    
    let result = validate_kan(&tiles, KanType::Closed, &sources);
    assert!(result.is_ok());
}

/// 测试无效的暗杠验证 - 来源错误
#[test]
fn test_validate_invalid_closed_kan() {
    // 四张相同的牌
    let white = Tile::Dragon(majiang_core::tile::Dragon::White);
    let tiles = vec![white; 4];
    
    // 其中一张来自其他玩家（暗杠应全部是自己的牌）
    let mut sources = vec![MeldSource::SelfDrawn; 3];
    sources.push(MeldSource::Player(1));
    
    let result = validate_kan(&tiles, KanType::Closed, &sources);
    assert!(result.is_err());
}

/// 测试无效的杠验证 - 牌不同
#[test]
fn test_validate_invalid_kan_different_tiles() {
    // 四张牌但不完全相同
    let tiles = vec![
        Tile::Wind(majiang_core::tile::Wind::East),
        Tile::Wind(majiang_core::tile::Wind::East),
        Tile::Wind(majiang_core::tile::Wind::East),
        Tile::Wind(majiang_core::tile::Wind::South), // 不同的风牌
    ];
    
    let sources = vec![MeldSource::SelfDrawn; 4];
    
    let result = validate_kan(&tiles, KanType::Closed, &sources);
    assert!(result.is_err());
}

/// 测试无效的杠验证 - 牌数量错误
#[test]
fn test_validate_invalid_kan_wrong_count() {
    // 只有三张牌（杠需要四张）
    let north = Tile::Wind(majiang_core::tile::Wind::North);
    let tiles = vec![north; 3];
    
    let sources = vec![MeldSource::SelfDrawn; 3];
    
    let result = validate_kan(&tiles, KanType::Closed, &sources);
    assert!(result.is_err());
}

/// 测试有效的加杠验证
#[test]
fn test_validate_valid_added_kan() {
    // 四张相同的牌
    let red = Tile::Dragon(majiang_core::tile::Dragon::Red);
    let tiles = vec![red; 4];
    
    // 三张来自之前的碰（其中一张来自其他玩家），一张是新摸到的
    let sources = vec![
        MeldSource::Player(0),
        MeldSource::SelfDrawn,
        MeldSource::SelfDrawn,
        MeldSource::SelfDrawn, // 新加的牌
    ];
    
    let result = validate_kan(&tiles, KanType::Added, &sources);
    assert!(result.is_ok());
}