// tests/tile/test_serialization.rs
//
// 麻将牌序列化功能的测试用例
// 本文件测试serialization.rs中的核心方法，确保牌的序列化和反序列化功能正确无误

use majiang_core::tile::{
    Tile, Suit, Wind, Dragon, Flower,
    to_id, from_id, to_data, from_data, tiles_to_ids, ids_to_tiles
};

/// 测试单张牌的序列化ID转换
#[test]
fn test_single_tile_id_conversion() {
    // 测试各种类型牌的ID转换
    let test_cases = [
        Tile::Suit(Suit::Character, 1),  // 一万
        Tile::Suit(Suit::Dot, 5),        // 五筒
        Tile::Suit(Suit::Bamboo, 9),     // 九条
        Tile::Wind(Wind::East),          // 东风
        Tile::Dragon(Dragon::Red),       // 红中
        Tile::Flower(Flower::Spring),    // 春
        Tile::Joker,                     // 百搭
    ];
    
    for tile in &test_cases {
        let id = to_id(tile);
        let recovered = from_id(id);
        
        assert!(recovered.is_some());
        assert_eq!(recovered.unwrap(), *tile);
    }
    
    // 测试无效ID
    assert_eq!(from_id(255), None);
}

/// 测试TileData结构的转换
#[test]
fn test_tile_data_conversion() {
    // 测试数牌
    let man5 = Tile::Suit(Suit::Character, 5);
    let data = to_data(&man5);
    
    assert_eq!(data.id, 4);
    assert_eq!(data.kind, "万子");
    assert_eq!(data.value, "5");
    assert!(data.is_red); // 五万是红牌
    
    let recovered = from_data(&data);
    assert!(recovered.is_some());
    assert_eq!(recovered.unwrap(), man5);
    
    // 测试风牌
    let east = Tile::Wind(Wind::East);
    let data = to_data(&east);
    
    assert_eq!(data.id, 27);
    assert_eq!(data.kind, "风牌");
    assert_eq!(data.value, "东");
    assert!(!data.is_red);
    
    let recovered = from_data(&data);
    assert!(recovered.is_some());
    assert_eq!(recovered.unwrap(), east);
    
    // 测试三元牌
    let white = Tile::Dragon(Dragon::White);
    let data = to_data(&white);
    
    assert_eq!(data.id, 31);
    assert_eq!(data.kind, "三元牌");
    assert_eq!(data.value, "白");
    assert!(!data.is_red);
    
    let recovered = from_data(&data);
    assert!(recovered.is_some());
    assert_eq!(recovered.unwrap(), white);
    
    // 测试花牌
    let plum = Tile::Flower(Flower::Plum);
    let data = to_data(&plum);
    
    assert_eq!(data.id, 38);
    assert_eq!(data.kind, "花牌");
    assert_eq!(data.value, "梅");
    assert!(!data.is_red);
    
    let recovered = from_data(&data);
    assert!(recovered.is_some());
    assert_eq!(recovered.unwrap(), plum);
    
    // 测试百搭牌
    let joker = Tile::Joker;
    let data = to_data(&joker);
    
    assert_eq!(data.id, 42);
    assert_eq!(data.kind, "百搭");
    assert_eq!(data.value, "百搭");
    assert!(!data.is_red);
    
    let recovered = from_data(&data);
    assert!(recovered.is_some());
    assert_eq!(recovered.unwrap(), joker);
}

/// 测试牌组的批量序列化转换
#[test]
fn test_tiles_batch_conversion() {
    // 创建测试牌组
    let tiles = vec![
        Tile::Suit(Suit::Character, 1),
        Tile::Suit(Suit::Character, 2),
        Tile::Suit(Suit::Character, 3),
        Tile::Wind(Wind::East),
        Tile::Dragon(Dragon::Red),
    ];
    
    // 批量转换为ID
    let ids = tiles_to_ids(&tiles);
    
    // 验证ID
    assert_eq!(ids, vec![0, 1, 2, 27, 33]);
    
    // 从ID恢复牌组
    let recovered = ids_to_tiles(&ids);
    
    // 验证恢复的牌组
    assert_eq!(recovered.len(), tiles.len());
    
    for (i, tile_opt) in recovered.iter().enumerate() {
        assert!(tile_opt.is_some());
        assert_eq!(tile_opt.unwrap(), tiles[i]);
    }
    
    // 测试包含无效ID的情况
    let invalid_ids = vec![0, 255, 1];
    let recovered = ids_to_tiles(&invalid_ids);
    
    assert_eq!(recovered.len(), 3);
    assert!(recovered[0].is_some());
    assert_eq!(recovered[0].unwrap(), Tile::Suit(Suit::Character, 1));
    assert!(recovered[1].is_none());
    assert!(recovered[2].is_some());
    assert_eq!(recovered[2].unwrap(), Tile::Suit(Suit::Character, 2));
}