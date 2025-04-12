// tests/meld/test_meld_operations.rs
//
// 测试副露的操作功能，包括排序、判断可能的组合等

use majiang_core::meld::{Meld, MeldType, MeldSource, KanType};
use majiang_core::meld::utils::{
    sort_tiles_in_meld, can_form_chi, can_form_pon, can_form_kan,
    get_possible_chi_combinations, get_kan_tile_count, get_kan_tiles
};
use majiang_core::tile::{Tile, Suit};

/// 测试副露排序功能 - 吃的牌按照点数排序
#[test]
fn test_sort_chi_tiles() {
    // 创建一个顺序不对的吃副露（4筒、3筒、2筒）
    let tiles = vec![
        Tile::new_suit(Suit::Dot, 4).unwrap(),
        Tile::new_suit(Suit::Dot, 3).unwrap(),
        Tile::new_suit(Suit::Dot, 2).unwrap(),
    ];
    
    let sources = vec![
        MeldSource::Player(1),
        MeldSource::SelfDrawn,
        MeldSource::SelfDrawn,
    ];
    
    let meld = Meld::new(tiles, MeldType::Chi, sources).unwrap();
    
    // 排序后应该是2筒、3筒、4筒
    let sorted = sort_tiles_in_meld(&meld);
    
    assert_eq!(sorted.tiles.len(), 3);
    
    // 验证排序结果
    if let Tile::Suit(_, number) = sorted.tiles[0] {
        assert_eq!(number, 2);
    } else {
        panic!("期望是数牌");
    }
    
    if let Tile::Suit(_, number) = sorted.tiles[1] {
        assert_eq!(number, 3);
    } else {
        panic!("期望是数牌");
    }
    
    if let Tile::Suit(_, number) = sorted.tiles[2] {
        assert_eq!(number, 4);
    } else {
        panic!("期望是数牌");
    }
    
    // 验证来源也随之排序
    assert_eq!(sorted.sources[0], MeldSource::SelfDrawn);
    assert_eq!(sorted.sources[1], MeldSource::SelfDrawn);
    assert_eq!(sorted.sources[2], MeldSource::Player(1));
}

/// 测试副露排序功能 - 碰的牌保持原样
#[test]
fn test_sort_pon_unchanged() {
    // 碰的牌都一样，排序前后应该一致
    let tile = Tile::new_suit(Suit::Character, 7).unwrap();
    let tiles = vec![tile, tile, tile];
    
    let sources = vec![
        MeldSource::SelfDrawn,
        MeldSource::SelfDrawn,
        MeldSource::Player(2),
    ];
    
    let meld = Meld::new(tiles.clone(), MeldType::Pon, sources.clone()).unwrap();
    let sorted = sort_tiles_in_meld(&meld);
    
    // 验证排序前后一致
    assert_eq!(sorted.tiles, meld.tiles);
    assert_eq!(sorted.sources, meld.sources);
}

/// 测试判断是否可以吃
#[test]
fn test_can_form_chi() {
    // 测试手牌
    let hand_tiles = vec![
        Tile::new_suit(Suit::Bamboo, 1).unwrap(),
        Tile::new_suit(Suit::Bamboo, 2).unwrap(),
        Tile::new_suit(Suit::Character, 3).unwrap(),
        Tile::new_suit(Suit::Character, 4).unwrap(),
        Tile::new_suit(Suit::Character, 6).unwrap(),
    ];
    
    // 可以吃的牌：3条（和手上的1条、2条组成顺子）
    let can_chi_tile = Tile::new_suit(Suit::Bamboo, 3).unwrap();
    assert!(can_form_chi(&hand_tiles, can_chi_tile));
    
    // 可以吃的牌：5万（和手上的3万、4万组成顺子）
    let can_chi_tile2 = Tile::new_suit(Suit::Character, 5).unwrap();
    assert!(can_form_chi(&hand_tiles, can_chi_tile2));
    
    // 不能吃的牌：7万（手上没有相邻的牌）
    let cannot_chi_tile = Tile::new_suit(Suit::Character, 7).unwrap();
    assert!(!can_form_chi(&hand_tiles, cannot_chi_tile));
    
    // 不能吃的牌：字牌
    let wind_tile = Tile::Wind(majiang_core::tile::Wind::East);
    assert!(!can_form_chi(&hand_tiles, wind_tile));
}

/// 测试判断是否可以碰
#[test]
fn test_can_form_pon() {
    // 测试手牌
    let hand_tiles = vec![
        Tile::new_suit(Suit::Dot, 5).unwrap(),
        Tile::new_suit(Suit::Dot, 5).unwrap(),
        Tile::new_suit(Suit::Bamboo, 8).unwrap(),
        Tile::Wind(majiang_core::tile::Wind::South),
        Tile::Wind(majiang_core::tile::Wind::South),
    ];
    
    // 可以碰的牌：5筒（手上有两张）
    let can_pon_tile = Tile::new_suit(Suit::Dot, 5).unwrap();
    assert!(can_form_pon(&hand_tiles, can_pon_tile));
    
    // 可以碰的牌：南风（手上有两张）
    let can_pon_tile2 = Tile::Wind(majiang_core::tile::Wind::South);
    assert!(can_form_pon(&hand_tiles, can_pon_tile2));
    
    // 不能碰的牌：8条（手上只有一张）
    let cannot_pon_tile = Tile::new_suit(Suit::Bamboo, 8).unwrap();
    assert!(!can_form_pon(&hand_tiles, cannot_pon_tile));
    
    // 不能碰的牌：东风（手上没有）
    let cannot_pon_tile2 = Tile::Wind(majiang_core::tile::Wind::East);
    assert!(!can_form_pon(&hand_tiles, cannot_pon_tile2));
}

/// 测试判断是否可以杠
#[test]
fn test_can_form_kan() {
    // 测试手牌
    let hand_tiles = vec![
        Tile::new_suit(Suit::Character, 1).unwrap(),
        Tile::new_suit(Suit::Character, 1).unwrap(),
        Tile::new_suit(Suit::Character, 1).unwrap(),
        Tile::Dragon(majiang_core::tile::Dragon::Red),
        Tile::Dragon(majiang_core::tile::Dragon::Red),
    ];
    
    // 可以自摸杠：1万（手上已有三张）
    let can_self_kan_tile = Tile::new_suit(Suit::Character, 1).unwrap();
    assert!(can_form_kan(&hand_tiles, can_self_kan_tile, true));
    
    // 可以明杠：1万
    assert!(can_form_kan(&hand_tiles, can_self_kan_tile, false));
    
    // 不能杠：红中（手上只有两张）
    let cannot_kan_tile = Tile::Dragon(majiang_core::tile::Dragon::Red);
    assert!(!can_form_kan(&hand_tiles, cannot_kan_tile, true));
    assert!(!can_form_kan(&hand_tiles, cannot_kan_tile, false));
}

/// 测试获取可能的吃组合
#[test]
fn test_get_possible_chi_combinations() {
    // 测试手牌
    let hand_tiles = vec![
        Tile::new_suit(Suit::Character, 1).unwrap(),
        Tile::new_suit(Suit::Character, 2).unwrap(),
        Tile::new_suit(Suit::Character, 4).unwrap(),
        Tile::new_suit(Suit::Character, 5).unwrap(),
        Tile::new_suit(Suit::Bamboo, 3).unwrap(),
        Tile::new_suit(Suit::Bamboo, 4).unwrap(),
    ];
    
    // 3万可以组成两种吃：(1万,2万,3万) 或 (3万,4万,5万)
    let tile_3man = Tile::new_suit(Suit::Character, 3).unwrap();
    let combinations = get_possible_chi_combinations(&hand_tiles, tile_3man);
    
    assert_eq!(combinations.len(), 2);
    
    // 5条只能组成一种吃：(3条,4条,5条)
    let tile_5sou = Tile::new_suit(Suit::Bamboo, 5).unwrap();
    let combinations2 = get_possible_chi_combinations(&hand_tiles, tile_5sou);
    
    assert_eq!(combinations2.len(), 1);
    
    // 9万不能组成吃
    let tile_9man = Tile::new_suit(Suit::Character, 9).unwrap();
    let combinations3 = get_possible_chi_combinations(&hand_tiles, tile_9man);
    
    assert_eq!(combinations3.len(), 0);
}

/// 测试获取杠中的牌数量
#[test]
fn test_get_kan_tile_count() {
    // 创建一个杠副露
    let tile = Tile::new_suit(Suit::Dot, 6).unwrap();
    let tiles = vec![tile, tile, tile, tile];
    let sources = vec![MeldSource::SelfDrawn; 4];
    
    let kan = Meld::new(tiles, MeldType::Kan(KanType::Closed), sources).unwrap();
    
    // 测试计数
    assert_eq!(get_kan_tile_count(&kan, tile), 4);
    
    // 测试不同的牌
    let other_tile = Tile::new_suit(Suit::Dot, 7).unwrap();
    assert_eq!(get_kan_tile_count(&kan, other_tile), 0);
    
    // 测试非杠副露
    let pon_tiles = vec![tile, tile, tile];
    let pon_sources = vec![MeldSource::SelfDrawn; 3];
    let pon = Meld::new(pon_tiles, MeldType::Pon, pon_sources).unwrap();
    
    assert_eq!(get_kan_tile_count(&pon, tile), 0);
}

/// 测试获取杠中的所有牌
#[test]
fn test_get_kan_tiles() {
    // 创建杠副露
    let tile = Tile::Wind(majiang_core::tile::Wind::West);
    let tiles = vec![tile; 4];
    let sources = vec![MeldSource::SelfDrawn; 4];
    
    let kan = Meld::new(tiles.clone(), MeldType::Kan(KanType::Closed), sources).unwrap();
    
    // 获取杠中的牌
    let kan_tiles = get_kan_tiles(&kan);
    assert!(kan_tiles.is_some());
    assert_eq!(kan_tiles.unwrap(), tiles);
    
    // 测试非杠副露
    let chi_tiles = vec![
        Tile::new_suit(Suit::Character, 2).unwrap(),
        Tile::new_suit(Suit::Character, 3).unwrap(),
        Tile::new_suit(Suit::Character, 4).unwrap(),
    ];
    let chi_sources = vec![MeldSource::SelfDrawn; 3];
    let chi = Meld::new(chi_tiles, MeldType::Chi, chi_sources).unwrap();
    
    assert!(get_kan_tiles(&chi).is_none());
}

/// 测试获取副露中的关键牌
#[test]
fn test_get_key_tile() {
    // 对于吃，关键牌是中间的牌
    let chi_tiles = vec![
        Tile::new_suit(Suit::Bamboo, 3).unwrap(),
        Tile::new_suit(Suit::Bamboo, 4).unwrap(),
        Tile::new_suit(Suit::Bamboo, 5).unwrap(),
    ];
    let chi_sources = vec![MeldSource::SelfDrawn; 3];
    let chi = Meld::new(chi_tiles, MeldType::Chi, chi_sources).unwrap();
    
    assert_eq!(chi.get_key_tile(), Tile::new_suit(Suit::Bamboo, 4).unwrap());
    
    // 对于碰，关键牌是其中一张牌（因为都一样）
    let pon_tile = Tile::Dragon(majiang_core::tile::Dragon::White);
    let pon_tiles = vec![pon_tile; 3];
    let pon_sources = vec![MeldSource::SelfDrawn; 3];
    let pon = Meld::new(pon_tiles, MeldType::Pon, pon_sources).unwrap();
    
    assert_eq!(pon.get_key_tile(), pon_tile);
    
    // 对于杠，关键牌也是其中一张牌
    let kan_tile = Tile::new_suit(Suit::Dot, 9).unwrap();
    let kan_tiles = vec![kan_tile; 4];
    let kan_sources = vec![MeldSource::SelfDrawn; 4];
    let kan = Meld::new(kan_tiles, MeldType::Kan(KanType::Closed), kan_sources).unwrap();
    
    assert_eq!(kan.get_key_tile(), kan_tile);
}