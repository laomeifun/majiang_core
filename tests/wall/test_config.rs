// tests/wall/test_config.rs
//
// 牌墙配置测试
// 展示不同牌墙配置的用法和特性

use majiang_core::tile::Tile;
use majiang_core::wall::{Wall, WallConfig, DeadWallConfig, builder};

/// 测试并展示各种牌墙配置
#[test]
fn test_wall_configurations() {
    // 1. 日本麻将配置
    let riichi_config = WallConfig::Riichi;
    let riichi_tiles = builder::build_tiles(riichi_config).unwrap();
    
    // 日麻共136张牌 = 数牌(3种*9点*4副) + 字牌(7种*4副)
    assert_eq!(riichi_tiles.len(), 136);
    
    // 验证日麻没有花牌
    let has_flowers = riichi_tiles.iter().any(|tile| matches!(tile, Tile::Flower(_)));
    assert!(!has_flowers);
    
    // 2. 中国官方规则(MCR)配置
    let mcr_config = WallConfig::MCR;
    let mcr_tiles = builder::build_tiles(mcr_config).unwrap();
    
    // MCR共144张牌 = 数牌(3种*9点*4副) + 字牌(7种*4副) + 花牌(8张)
    assert_eq!(mcr_tiles.len(), 144);
    
    // 验证MCR有8张花牌
    let flower_count = mcr_tiles.iter()
        .filter(|tile| matches!(tile, Tile::Flower(_)))
        .count();
    assert_eq!(flower_count, 8);
    
    // 3. 上海麻将配置(带百搭)
    let shanghai_config = WallConfig::Shanghai { with_joker: true };
    let shanghai_tiles = builder::build_tiles(shanghai_config).unwrap();
    
    // 上海麻将带百搭共145张牌 = 数牌(3种*9点*4副) + 字牌(7种*4副) + 花牌(8张) + 百搭(1张)
    assert_eq!(shanghai_tiles.len(), 145);
    
    // 验证上海麻将有百搭牌
    let has_joker = shanghai_tiles.iter().any(|tile| matches!(tile, Tile::Joker));
    assert!(has_joker);
    
    // 4. 自定义配置示例
    let custom_config = WallConfig::Custom {
        flowers: 4,         // 只用四季花
        with_joker: true,   // 带百搭
        suit_sets: 2,       // 每种数牌2副
        honor_sets: 1       // 字牌1副
    };
    let custom_tiles = builder::build_tiles(custom_config).unwrap();
    
    // 计算牌数: (数牌2副*3种*9张) + (字牌1副*7种) + (花牌4张) + (百搭1张) = 54 + 7 + 4 + 1 = 66
    assert_eq!(custom_tiles.len(), 66);
}

/// 展示如何使用固定种子创建可复现的牌墙
#[test]
fn test_reproducible_wall() {
    // 创建一个使用固定种子的牌墙
    let seed = 42;
    let shuffled_tiles1 = builder::create_shuffled_tiles(WallConfig::MCR, Some(seed)).unwrap();
    
    // 使用相同种子再创建一个
    let shuffled_tiles2 = builder::create_shuffled_tiles(WallConfig::MCR, Some(seed)).unwrap();
    
    // 两次洗牌结果应该完全相同（可预测、可复现）
    assert_eq!(shuffled_tiles1, shuffled_tiles2);
    
    // 这对测试和调试非常有用:
    // 1. 在发现bug时，可以记录种子并重现问题
    // 2. 在单元测试中，可以确保测试结果稳定
    // 3. 可以用来研究特定牌型和牌局
}

/// 展示如何在不同麻将规则中使用花牌
#[test]
fn test_flower_tiles_usage() {
    // 1. 创建MCR规则牌墙(有花牌)
    let mut mcr_wall = Wall::new(
        WallConfig::MCR,
        Some(DeadWallConfig::MCR { replacement_count: 8 }),
        Some(123) // 固定种子
    ).unwrap();
    
    // 发牌并检查花牌
    let hand = mcr_wall.deal_initial_hand(13).unwrap();
    mcr_wall.start_game();
    
    // 检查手牌中是否有花牌
    let flower_tiles: Vec<&Tile> = hand.iter()
        .filter(|&tile| matches!(tile, Tile::Flower(_)))
        .collect();
    
    // 如果有花牌，需要补牌
    let mut final_hand = hand.clone();
    for _ in 0..flower_tiles.len() {
        if let Ok(replacement) = mcr_wall.draw_replacement_tile() {
            final_hand.push(replacement);
        }
    }
    
    // 打印演示信息
    println!("MCR规则中的花牌处理:");
    println!("初始手牌有 {} 张花牌", flower_tiles.len());
    println!("补牌后手牌共 {} 张", final_hand.len());
    
    // 2. 创建日麻规则牌墙(无花牌)
    let mut riichi_wall = Wall::new(
        WallConfig::Riichi,
        Some(DeadWallConfig::Riichi { dora_indicators: 1, uradora_indicators: 1 }),
        Some(123) // 固定种子
    ).unwrap();
    
    // 日麻发牌
    let riichi_hand = riichi_wall.deal_initial_hand(13).unwrap();
    
    // 日麻不应该有花牌
    let has_flowers = riichi_hand.iter().any(|tile| matches!(tile, Tile::Flower(_)));
    assert!(!has_flowers);
}

/// 演示宝牌系统的使用方法
#[test]
fn test_dora_system_usage() {
    // 创建日麻牌墙
    let mut wall = Wall::new(
        WallConfig::Riichi,
        Some(DeadWallConfig::Riichi { dora_indicators: 3, uradora_indicators: 3 }),
        Some(42) // 固定种子
    ).unwrap();
    
    wall.start_game();
    
    // 获取初始宝牌指示牌
    let initial_dora = wall.get_dora_indicators().unwrap()[0];
    println!("初始宝牌指示牌: {}", initial_dora);
    
    // 在游戏中，宝牌指示牌指向的下一张牌是宝牌
    // 例如，如果指示牌是2万，宝牌是3万
    // 如果指示牌是9万，宝牌是1万
    // 如果指示牌是北，宝牌是东
    // 这个逻辑通常由规则模块实现
    
    // 模拟第一次杠后翻新宝牌
    let second_dora = wall.reveal_next_dora_indicator().unwrap();
    println!("第一次杠后的新宝牌指示牌: {}", second_dora);
    
    // 模拟第二次杠后翻新宝牌
    let third_dora = wall.reveal_next_dora_indicator().unwrap();
    println!("第二次杠后的新宝牌指示牌: {}", third_dora);
    
    // 游戏结束时，可以查看里宝牌指示牌(对立直者有效)
    let uradora = wall.get_uradora_indicators().unwrap();
    println!("里宝牌指示牌:");
    for (i, tile) in uradora.iter().enumerate() {
        println!("  第{}张: {}", i+1, tile);
    }
}