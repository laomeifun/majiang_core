// tests/wall/test_basic.rs
//
// 牌墙模块基础功能测试
// 包括初始化、洗牌、发牌等基本操作

use majiang_core::wall::{Wall, WallConfig, DeadWallConfig};
use majiang_core::errors::MajiangError;

/// 测试基本的牌墙创建
#[test]
fn test_wall_creation() {
    // 创建一个标准的中式麻将牌墙(MCR规则，含花牌)
    let wall = Wall::new(
        WallConfig::MCR, 
        Some(DeadWallConfig::MCR { replacement_count: 8 }),
        None
    ).expect("创建牌墙失败");
    
    // 验证牌墙初始状态
    assert_eq!(wall.config(), WallConfig::MCR);
    assert_eq!(wall.drawn_count(), 0);
    assert!(!wall.is_empty());
    
    // 标准MCR有144张牌，减去岭上牌区的16张，应该剩余128张
    assert_eq!(wall.remaining_tiles(), 128);
    
    // 验证岭上牌区存在
    assert!(wall.dead_wall().is_some());
}

/// 测试不同规则下的牌墙创建
#[test]
fn test_different_rule_walls() {
    // 1. 日本麻将牌墙(无花牌)
    let riichi_wall = Wall::new(
        WallConfig::Riichi,
        Some(DeadWallConfig::Riichi { dora_indicators: 1, uradora_indicators: 1 }),
        None
    ).expect("创建日麻牌墙失败");
    
    // 日麻有136张牌，减去岭上牌区的14张，应该剩余122张
    assert_eq!(riichi_wall.remaining_tiles(), 122);
    
    // 2. 上海麻将牌墙(含花牌和百搭)
    let shanghai_wall = Wall::new(
        WallConfig::Shanghai { with_joker: true },
        Some(DeadWallConfig::Shanghai { replacement_count: 8 }),
        None
    ).expect("创建上海麻将牌墙失败");
    
    // 上海麻将有144张牌+1张百搭，减去岭上牌区的16张，应该剩余129张
    assert_eq!(shanghai_wall.remaining_tiles(), 129);
    
    // 3. 自定义配置牌墙
    let custom_wall = Wall::new(
        WallConfig::Custom { 
            flowers: 4,       // 只用四季花
            with_joker: false,
            suit_sets: 3,     // 每种数牌3副
            honor_sets: 2     // 字牌2副
        },
        None, // 无岭上牌区
        None
    ).expect("创建自定义牌墙失败");
    
    // 计算牌数: (数牌3副*3种*9张) + (字牌2副*7种) + 4张花 = 81 + 14 + 4 = 99
    assert_eq!(custom_wall.remaining_tiles(), 99);
    
    // 无岭上牌区
    assert!(custom_wall.dead_wall().is_none());
}

/// 测试牌墙发牌功能
#[test]
fn test_dealing_tiles() {
    // 创建一个标准的中式麻将牌墙
    let mut wall = Wall::new(
        WallConfig::MCR, 
        Some(DeadWallConfig::MCR { replacement_count: 8 }),
        None
    ).expect("创建牌墙失败");
    
    // 发13张牌作为初始手牌
    let hand = wall.deal_initial_hand(13).expect("发牌失败");
    
    // 验证手牌数量
    assert_eq!(hand.len(), 13);
    
    // 验证牌墙状态变化
    assert_eq!(wall.drawn_count(), 13);
    assert_eq!(wall.remaining_tiles(), 115); // 128 - 13 = 115
    
    // 游戏开始
    wall.start_game();
    
    // 测试摸牌
    let _tile = wall.draw_tile().expect("摸牌失败");
    assert_eq!(wall.drawn_count(), 14);
    assert_eq!(wall.remaining_tiles(), 114); // 115 - 1 = 114
    
    // 再摸4张牌
    for _ in 0..4 {
        let _ = wall.draw_tile().expect("摸牌失败");
    }
    assert_eq!(wall.drawn_count(), 18);
    assert_eq!(wall.remaining_tiles(), 110); // 114 - 4 = 110
}

/// 测试牌墙种子确定性
#[test]
fn test_wall_seed_determinism() {
    // 使用固定种子创建两个牌墙
    let seed = 12345;
    let mut wall1 = Wall::new(
        WallConfig::MCR,
        None,
        Some(seed)
    ).expect("创建第一个牌墙失败");
    
    let mut wall2 = Wall::new(
        WallConfig::MCR,
        None,
        Some(seed)
    ).expect("创建第二个牌墙失败");
    
    // 连续摸10张牌并比较
    for _ in 0..10 {
        let tile1 = wall1.deal_initial_hand(1).expect("摸牌失败")[0];
        let tile2 = wall2.deal_initial_hand(1).expect("摸牌失败")[0];
        
        // 使用相同种子，摸出的牌应该完全相同
        assert_eq!(tile1, tile2);
    }
}

/// 测试牌墙错误处理
#[test]
fn test_wall_error_handling() {
    // 创建一个小牌墙
    let mut wall = Wall::new(
        WallConfig::Custom {
            flowers: 0,
            with_joker: false,
            suit_sets: 1, // 只有一副数牌
            honor_sets: 0  // 无字牌
        },
        None,
        None
    ).expect("创建牌墙失败");
    
    // 只有27张牌，尝试发28张
    let result = wall.deal_initial_hand(28);
    assert!(result.is_err());
    
    if let Err(e) = result {
        assert!(matches!(e, MajiangError::NotEnoughTiles));
    }
    
    // 正常发完26张牌
    let _ = wall.deal_initial_hand(26).expect("发牌失败");
    assert_eq!(wall.remaining_tiles(), 1); // 只剩1张牌
    
    // 开始游戏
    wall.start_game();
    
    // 摸最后一张牌
    let _ = wall.draw_tile().expect("摸最后一张牌失败");
    assert_eq!(wall.remaining_tiles(), 0);
    assert!(wall.is_empty());
    
    // 牌墙已空，继续摸牌应该出错
    let result = wall.draw_tile();
    assert!(result.is_err());
    
    if let Err(e) = result {
        assert!(matches!(e, MajiangError::NotEnoughTiles));
    }
}

/// 测试游戏状态约束
#[test]
fn test_game_state_constraints() {
    let mut wall = Wall::new(
        WallConfig::MCR,
        None,
        None
    ).expect("创建牌墙失败");
    
    // 发初始手牌
    let _ = wall.deal_initial_hand(13).expect("发牌失败");
    
    // 标记游戏开始
    wall.start_game();
    
    // 游戏开始后，不能再发初始手牌
    let result = wall.deal_initial_hand(13);
    assert!(result.is_err());
    
    if let Err(e) = result {
        assert!(matches!(e, MajiangError::InvalidOperation(_)));
    }
    
    // 游戏开始前，不能摸牌
    let mut wall2 = Wall::new(
        WallConfig::MCR,
        None,
        None
    ).expect("创建第二个牌墙失败");
    
    // 未标记游戏开始就摸牌应该失败
    let result = wall2.draw_tile();
    assert!(result.is_err());
    
    if let Err(e) = result {
        assert!(matches!(e, MajiangError::InvalidOperation(_)));
    }
}