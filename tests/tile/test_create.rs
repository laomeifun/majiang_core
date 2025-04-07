// tests/tile/test_create.rs
//
// 麻将牌基本操作的测试用例
// 本文件测试tile.rs中的核心方法，确保创建、转换和判断牌类型的功能正确无误

use majiang_core::tile::{Tile, Suit, Wind, Dragon, Flower};

/// 测试创建有效的数牌
#[test]
fn test_create_valid_suit_tiles() {
    // 测试所有有效的万子
    for i in 1..=9 {
        let tile = Tile::new_suit(Suit::Character, i);
        assert!(tile.is_some());
        assert_eq!(tile.unwrap(), Tile::Suit(Suit::Character, i));
    }
    
    // 测试所有有效的筒子
    for i in 1..=9 {
        let tile = Tile::new_suit(Suit::Dot, i);
        assert!(tile.is_some());
        assert_eq!(tile.unwrap(), Tile::Suit(Suit::Dot, i));
    }
    
    // 测试所有有效的索子
    for i in 1..=9 {
        let tile = Tile::new_suit(Suit::Bamboo, i);
        assert!(tile.is_some());
        assert_eq!(tile.unwrap(), Tile::Suit(Suit::Bamboo, i));
    }
}

/// 测试创建无效的数牌(超出1-9范围)
#[test]
fn test_create_invalid_suit_tiles() {
    // 测试无效的点数:0
    let tile = Tile::new_suit(Suit::Character, 0);
    assert!(tile.is_none());
    
    // 测试无效的点数:10
    let tile = Tile::new_suit(Suit::Dot, 10);
    assert!(tile.is_none());
    
    // 测试无效的点数:255
    let tile = Tile::new_suit(Suit::Bamboo, 255);
    assert!(tile.is_none());
}

/// 测试牌ID和牌类型之间的相互转换
#[test]
fn test_tile_id_conversion() {
    // 测试数牌的转换（以部分典型值为例）
    let test_cases = [
        (Tile::Suit(Suit::Character, 1), 0), // 一万
        (Tile::Suit(Suit::Character, 5), 4), // 五万
        (Tile::Suit(Suit::Character, 9), 8), // 九万
        (Tile::Suit(Suit::Dot, 1), 9),       // 一筒
        (Tile::Suit(Suit::Dot, 5), 13),      // 五筒
        (Tile::Suit(Suit::Dot, 9), 17),      // 九筒
        (Tile::Suit(Suit::Bamboo, 1), 18),   // 一条
        (Tile::Suit(Suit::Bamboo, 5), 22),   // 五条
        (Tile::Suit(Suit::Bamboo, 9), 26),   // 九条
    ];
    
    for (tile, id) in test_cases {
        // 从牌到ID
        assert_eq!(tile.to_id(), id);
        // 从ID到牌
        assert_eq!(Tile::from_id(id), Some(tile));
    }
    
    // 测试风牌
    let winds = [
        (Tile::Wind(Wind::East), 27),
        (Tile::Wind(Wind::South), 28),
        (Tile::Wind(Wind::West), 29),
        (Tile::Wind(Wind::North), 30),
    ];
    
    for (tile, id) in winds {
        assert_eq!(tile.to_id(), id);
        assert_eq!(Tile::from_id(id), Some(tile));
    }
    
    // 测试三元牌
    let dragons = [
        (Tile::Dragon(Dragon::White), 31),
        (Tile::Dragon(Dragon::Green), 32),
        (Tile::Dragon(Dragon::Red), 33),
    ];
    
    for (tile, id) in dragons {
        assert_eq!(tile.to_id(), id);
        assert_eq!(Tile::from_id(id), Some(tile));
    }
    
    // 测试花牌
    let flowers = [
        (Tile::Flower(Flower::Spring), 34),
        (Tile::Flower(Flower::Summer), 35),
        (Tile::Flower(Flower::Autumn), 36),
        (Tile::Flower(Flower::Winter), 37),
        (Tile::Flower(Flower::Plum), 38),
        (Tile::Flower(Flower::Orchid), 39),
        (Tile::Flower(Flower::Bamboo), 40),
        (Tile::Flower(Flower::Chrysanthemum), 41),
    ];
    
    for (tile, id) in flowers {
        assert_eq!(tile.to_id(), id);
        assert_eq!(Tile::from_id(id), Some(tile));
    }
    
    // 测试百搭牌
    let joker = Tile::Joker;
    assert_eq!(joker.to_id(), 42);
    assert_eq!(Tile::from_id(42), Some(joker));
    
    // 测试无效ID
    assert_eq!(Tile::from_id(43), None);
    assert_eq!(Tile::from_id(255), None);
}

/// 测试牌类型判断方法
#[test]
fn test_tile_type_checks() {
    // 测试is_suit方法
    assert!(Tile::Suit(Suit::Character, 1).is_suit());
    assert!(Tile::Suit(Suit::Dot, 5).is_suit());
    assert!(Tile::Suit(Suit::Bamboo, 9).is_suit());
    assert!(!Tile::Wind(Wind::East).is_suit());
    assert!(!Tile::Dragon(Dragon::Red).is_suit());
    assert!(!Tile::Flower(Flower::Spring).is_suit());
    assert!(!Tile::Joker.is_suit());
    
    // 测试is_wind方法
    assert!(Tile::Wind(Wind::East).is_wind());
    assert!(Tile::Wind(Wind::South).is_wind());
    assert!(Tile::Wind(Wind::West).is_wind());
    assert!(Tile::Wind(Wind::North).is_wind());
    assert!(!Tile::Suit(Suit::Character, 1).is_wind());
    assert!(!Tile::Dragon(Dragon::Red).is_wind());
    
    // 测试is_dragon方法
    assert!(Tile::Dragon(Dragon::White).is_dragon());
    assert!(Tile::Dragon(Dragon::Green).is_dragon());
    assert!(Tile::Dragon(Dragon::Red).is_dragon());
    assert!(!Tile::Suit(Suit::Character, 1).is_dragon());
    assert!(!Tile::Wind(Wind::East).is_dragon());
    
    // 测试is_flower方法
    assert!(Tile::Flower(Flower::Spring).is_flower());
    assert!(Tile::Flower(Flower::Plum).is_flower());
    assert!(!Tile::Suit(Suit::Character, 1).is_flower());
    assert!(!Tile::Wind(Wind::East).is_flower());
    
    // 测试is_honor方法(风牌或三元牌)
    assert!(Tile::Wind(Wind::East).is_honor());
    assert!(Tile::Dragon(Dragon::Red).is_honor());
    assert!(!Tile::Suit(Suit::Character, 1).is_honor());
    assert!(!Tile::Flower(Flower::Spring).is_honor());
    assert!(!Tile::Joker.is_honor());
    
    // 测试is_joker方法
    assert!(Tile::Joker.is_joker());
    assert!(!Tile::Suit(Suit::Character, 1).is_joker());
    assert!(!Tile::Wind(Wind::East).is_joker());
    assert!(!Tile::Dragon(Dragon::Red).is_joker());
    assert!(!Tile::Flower(Flower::Spring).is_joker());
}

/// 测试红宝牌(红五)判断
#[test]
fn test_red_tiles() {
    // 五万是红五
    let man5 = Tile::Suit(Suit::Character, 5);
    assert!(man5.is_red());
    
    // 五筒是红五
    let pin5 = Tile::Suit(Suit::Dot, 5);
    assert!(pin5.is_red());
    
    // 五索是红五
    let sou5 = Tile::Suit(Suit::Bamboo, 5);
    assert!(sou5.is_red());
    
    // 其他五以外的数牌不是红牌
    assert!(!Tile::Suit(Suit::Character, 1).is_red());
    assert!(!Tile::Suit(Suit::Character, 9).is_red());
    assert!(!Tile::Suit(Suit::Dot, 1).is_red());
    assert!(!Tile::Suit(Suit::Dot, 9).is_red());
    assert!(!Tile::Suit(Suit::Bamboo, 1).is_red());
    assert!(!Tile::Suit(Suit::Bamboo, 9).is_red());
    
    // 字牌不是红牌
    assert!(!Tile::Wind(Wind::East).is_red());
    assert!(!Tile::Dragon(Dragon::Red).is_red());
    
    // 花牌不是红牌
    assert!(!Tile::Flower(Flower::Spring).is_red());
    
    // 百搭不是红牌
    assert!(!Tile::Joker.is_red());
}

/// 测试Display trait实现(字符串表示)
#[test]
fn test_tile_display() {
    // 数牌显示测试
    assert_eq!(Tile::Suit(Suit::Character, 1).to_string(), "1万");
    assert_eq!(Tile::Suit(Suit::Dot, 5).to_string(), "5筒");
    assert_eq!(Tile::Suit(Suit::Bamboo, 9).to_string(), "9条");
    
    // 风牌显示测试
    assert_eq!(Tile::Wind(Wind::East).to_string(), "东");
    assert_eq!(Tile::Wind(Wind::South).to_string(), "南");
    assert_eq!(Tile::Wind(Wind::West).to_string(), "西");
    assert_eq!(Tile::Wind(Wind::North).to_string(), "北");
    
    // 三元牌显示测试
    assert_eq!(Tile::Dragon(Dragon::White).to_string(), "白");
    assert_eq!(Tile::Dragon(Dragon::Green).to_string(), "发");
    assert_eq!(Tile::Dragon(Dragon::Red).to_string(), "中");
    
    // 花牌显示测试
    assert_eq!(Tile::Flower(Flower::Spring).to_string(), "春");
    assert_eq!(Tile::Flower(Flower::Summer).to_string(), "夏");
    assert_eq!(Tile::Flower(Flower::Autumn).to_string(), "秋");
    assert_eq!(Tile::Flower(Flower::Winter).to_string(), "冬");
    assert_eq!(Tile::Flower(Flower::Plum).to_string(), "梅");
    assert_eq!(Tile::Flower(Flower::Orchid).to_string(), "兰");
    assert_eq!(Tile::Flower(Flower::Bamboo).to_string(), "竹");
    assert_eq!(Tile::Flower(Flower::Chrysanthemum).to_string(), "菊");
    
    // 百搭牌显示测试
    assert_eq!(Tile::Joker.to_string(), "百搭");
}