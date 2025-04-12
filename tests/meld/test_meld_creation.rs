// tests/meld/test_meld_creation.rs
//
// 测试副露的创建功能，包括吃、碰、杠的基本构造

use majiang_core::errors::MajiangError;
use majiang_core::meld::{Meld, MeldType, KanType, MeldSource};
use majiang_core::tile::{Tile, Suit};

/// 测试创建基本的吃副露
#[test]
fn test_create_chi() {
    // 准备三张连续的同花色牌：2筒、3筒、4筒
    let tiles = vec![
        Tile::new_suit(Suit::Dot, 2).unwrap(),
        Tile::new_suit(Suit::Dot, 3).unwrap(),
        Tile::new_suit(Suit::Dot, 4).unwrap(),
    ];
    
    // 假设3筒是从另一个玩家处获得的，其他是自己的牌
    let sources = vec![
        MeldSource::SelfDrawn,
        MeldSource::Player(1),
        MeldSource::SelfDrawn,
    ];
    
    // 创建吃副露
    let result = Meld::new(tiles, MeldType::Chi, sources);
    
    // 验证创建成功
    assert!(result.is_ok());
    
    // 进一步验证创建的副露内容
    let meld = result.unwrap();
    assert_eq!(meld.meld_type, MeldType::Chi);
    assert_eq!(meld.tiles.len(), 3);
    assert_eq!(meld.sources.len(), 3);
    assert_eq!(meld.sources[1], MeldSource::Player(1));
    assert!(meld.is_open()); // 吃是明露的副露
}

/// 测试创建基本的碰副露
#[test]
fn test_create_pon() {
    // 准备三张相同的牌：5万、5万、5万
    let tiles = vec![
        Tile::new_suit(Suit::Character, 5).unwrap(),
        Tile::new_suit(Suit::Character, 5).unwrap(),
        Tile::new_suit(Suit::Character, 5).unwrap(),
    ];
    
    // 假设其中一张是从玩家2处获得的
    let sources = vec![
        MeldSource::SelfDrawn,
        MeldSource::SelfDrawn,
        MeldSource::Player(2),
    ];
    
    // 创建碰副露
    let result = Meld::new(tiles, MeldType::Pon, sources);
    
    // 验证创建成功
    assert!(result.is_ok());
    
    // 进一步验证创建的副露内容
    let meld = result.unwrap();
    assert_eq!(meld.meld_type, MeldType::Pon);
    assert_eq!(meld.tiles.len(), 3);
    assert_eq!(meld.sources.len(), 3);
    assert_eq!(meld.sources[2], MeldSource::Player(2));
    assert!(meld.is_open()); // 碰是明露的副露
}

/// 测试创建明杠副露
#[test]
fn test_create_open_kan() {
    // 准备四张相同的牌：8条、8条、8条、8条
    let tiles = vec![
        Tile::new_suit(Suit::Bamboo, 8).unwrap(),
        Tile::new_suit(Suit::Bamboo, 8).unwrap(),
        Tile::new_suit(Suit::Bamboo, 8).unwrap(),
        Tile::new_suit(Suit::Bamboo, 8).unwrap(),
    ];
    
    // 假设其中一张是从玩家0处获得的
    let sources = vec![
        MeldSource::SelfDrawn,
        MeldSource::SelfDrawn,
        MeldSource::SelfDrawn,
        MeldSource::Player(0),
    ];
    
    // 创建明杠副露
    let result = Meld::new(tiles, MeldType::Kan(KanType::Open), sources);
    
    // 验证创建成功
    assert!(result.is_ok());
    
    // 进一步验证创建的副露内容
    let meld = result.unwrap();
    assert_eq!(meld.meld_type, MeldType::Kan(KanType::Open));
    assert_eq!(meld.tiles.len(), 4);
    assert_eq!(meld.sources.len(), 4);
    assert!(meld.is_open()); // 明杠是公开的
}

/// 测试创建暗杠副露
#[test]
fn test_create_closed_kan() {
    // 准备四张相同的牌：发、发、发、发
    let dragon_green = Tile::Dragon(majiang_core::tile::Dragon::Green);
    let tiles = vec![dragon_green; 4];
    
    // 所有牌都是自己摸到的
    let sources = vec![MeldSource::SelfDrawn; 4];
    
    // 创建暗杠副露
    let result = Meld::new(tiles, MeldType::Kan(KanType::Closed), sources);
    
    // 验证创建成功
    assert!(result.is_ok());
    
    // 进一步验证创建的副露内容
    let meld = result.unwrap();
    assert_eq!(meld.meld_type, MeldType::Kan(KanType::Closed));
    assert_eq!(meld.tiles.len(), 4);
    assert_eq!(meld.sources.len(), 4);
    assert!(!meld.is_open()); // 暗杠不是公开的
}

/// 测试创建加杠副露
#[test]
fn test_create_added_kan() {
    // 准备四张相同的牌：南、南、南、南
    let wind_south = Tile::Wind(majiang_core::tile::Wind::South);
    let tiles = vec![wind_south; 4];
    
    // 三张来自之前的碰，一张是新摸到的
    let sources = vec![
        MeldSource::Player(3),
        MeldSource::SelfDrawn,
        MeldSource::SelfDrawn,
        MeldSource::SelfDrawn, // 新摸到的牌
    ];
    
    // 创建加杠副露
    let result = Meld::new(tiles, MeldType::Kan(KanType::Added), sources);
    
    // 验证创建成功
    assert!(result.is_ok());
    
    // 进一步验证创建的副露内容
    let meld = result.unwrap();
    assert_eq!(meld.meld_type, MeldType::Kan(KanType::Added));
    assert_eq!(meld.tiles.len(), 4);
    assert_eq!(meld.sources.len(), 4);
    assert!(meld.is_open()); // 加杠是公开的
}

/// 测试创建无效副露（牌数量不匹配）
#[test]
fn test_create_invalid_meld_wrong_tile_count() {
    // 只有两张牌的碰（无效）
    let tiles = vec![
        Tile::new_suit(Suit::Character, 3).unwrap(),
        Tile::new_suit(Suit::Character, 3).unwrap(),
    ];
    let sources = vec![MeldSource::SelfDrawn, MeldSource::Player(1)];
    
    let result = Meld::new(tiles, MeldType::Pon, sources);
    assert!(result.is_err());
    
    // 检查错误类型
    match result {
        Err(MajiangError::InvalidMeld(_)) => assert!(true),
        _ => panic!("期望InvalidMeld错误，但得到了不同的结果"),
    }
}

/// 测试创建无效副露（来源数量不匹配）
#[test]
fn test_create_invalid_meld_wrong_source_count() {
    // 三张牌但只有两个来源（无效）
    let tiles = vec![
        Tile::new_suit(Suit::Dot, 7).unwrap(),
        Tile::new_suit(Suit::Dot, 7).unwrap(),
        Tile::new_suit(Suit::Dot, 7).unwrap(),
    ];
    let sources = vec![MeldSource::SelfDrawn, MeldSource::Player(2)]; // 少一个来源
    
    let result = Meld::new(tiles, MeldType::Pon, sources);
    assert!(result.is_err());
    
    // 检查错误类型
    match result {
        Err(MajiangError::InvalidMeld(_)) => assert!(true),
        _ => panic!("期望InvalidMeld错误，但得到了不同的结果"),
    }
}