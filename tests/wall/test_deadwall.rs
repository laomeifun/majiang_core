// tests/wall/test_deadwall.rs
//
// 岭上牌区功能测试
// 主要测试不同规则下岭上牌的管理、宝牌指示牌和杠后补牌等功能

use majiang_core::wall::{Wall, WallConfig, DeadWallConfig};
use majiang_core::errors::MajiangError;

/// 测试日麻规则下的岭上牌区创建和基本操作
#[test]
fn test_riichi_deadwall() {
    // 创建一个日麻牌墙，带有岭上牌区
    let mut wall = Wall::new(
        WallConfig::Riichi,
        Some(DeadWallConfig::Riichi { 
            dora_indicators: 2,    // 2个表宝牌
            uradora_indicators: 1  // 1个里宝牌
        }),
        None  // 随机种子
    ).expect("创建日麻牌墙失败");
    
    // 游戏开始
    wall.start_game();
    
    // 获取宝牌指示牌，初始只有第一个宝牌指示牌可见
    let dora_indicators = wall.get_dora_indicators().expect("获取宝牌指示牌失败");
    assert_eq!(dora_indicators.len(), 1);
    
    // 进行一次杠操作，翻开一个新的宝牌指示牌
    let _new_dora = wall.reveal_next_dora_indicator().expect("翻新宝牌失败");
    
    // 现在应该有两个宝牌指示牌了（初始的一个 + 新翻的一个）
    let dora_indicators = wall.get_dora_indicators().expect("获取宝牌指示牌失败");
    assert_eq!(dora_indicators.len(), 2);
    
    // 进行杠后补牌
    let _replacement_tile = wall.draw_replacement_tile().expect("杠后补牌失败");
    
    // 验证是否正确计数
    assert_eq!(wall.drawn_count(), 1);  // 杠后补牌也计入已摸牌数
    
    // 再次翻开宝牌指示牌应该失败，因为已经翻开了所有配置允许的宝牌
    let result = wall.reveal_next_dora_indicator();
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e, MajiangError::InvalidOperation(_)));
    }
}

/// 测试日麻规则下的宝牌和里宝牌
#[test]
fn test_riichi_dora_and_uradora() {
    // 使用固定种子，便于测试
    let seed = 42;
    let mut wall = Wall::new(
        WallConfig::Riichi,
        Some(DeadWallConfig::Riichi { 
            dora_indicators: 3,    // 3个表宝牌
            uradora_indicators: 3  // 3个里宝牌
        }),
        Some(seed)
    ).expect("创建日麻牌墙失败");
    
    wall.start_game();
    
    // 初始只有第一个宝牌指示牌可见
    let dora_indicators = wall.get_dora_indicators().expect("获取宝牌指示牌失败");
    assert_eq!(dora_indicators.len(), 1);
    
    // 获取里宝牌指示牌（通常游戏结束时才会公开）
    let uradora_indicators = wall.get_uradora_indicators().expect("获取里宝牌指示牌失败");
    assert_eq!(uradora_indicators.len(), 3); // 配置了3个里宝牌
    
    // 连续进行杠操作，应该翻开所有宝牌指示牌
    for i in 0..2 {
        let _new_dora = wall.reveal_next_dora_indicator().expect("翻新宝牌失败");
        let dora_indicators = wall.get_dora_indicators().expect("获取宝牌指示牌失败");
        assert_eq!(dora_indicators.len(), i + 2); // 初始1个 + 新翻的数量
    }
    
    // 所有宝牌指示牌都已翻开，再翻应该出错
    let result = wall.reveal_next_dora_indicator();
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e, MajiangError::InvalidOperation(_)));
    }
}

/// 测试中式麻将规则下的岭上牌区
#[test]
fn test_mcr_deadwall() {
    // 创建一个中式麻将牌墙(MCR规则)，带有岭上牌区
    let mut wall = Wall::new(
        WallConfig::MCR,
        Some(DeadWallConfig::MCR { replacement_count: 8 }),
        None
    ).expect("创建中式麻将牌墙失败");
    
    wall.start_game();
    
    // 中式麻将没有宝牌指示牌，尝试获取应该出错
    let result = wall.get_dora_indicators();
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e, MajiangError::InvalidOperation(_)));
    }
    
    // 测试杠后补牌功能
    for i in 0..8 {
        let _tile = wall.draw_replacement_tile().expect("杠后补牌失败");
        assert_eq!(wall.drawn_count(), i + 1);
    }
    
    // 查看岭上牌区的剩余补牌数量
    let dead_wall = wall.dead_wall().expect("获取岭上牌区失败");
    assert_eq!(dead_wall.remaining_replacement_tiles(), 0);
}

/// 测试没有岭上牌区的情况
#[test]
fn test_no_deadwall() {
    // 创建一个没有岭上牌区的牌墙
    let mut wall = Wall::new(
        WallConfig::MCR,
        None, // 无岭上牌区
        None
    ).expect("创建牌墙失败");
    
    wall.start_game();
    
    // 没有岭上牌区，尝试杠后补牌应该出错
    let result = wall.draw_replacement_tile();
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e, MajiangError::InvalidOperation(_)));
    }
    
    // 没有岭上牌区，尝试获取宝牌指示牌应该出错
    let result = wall.get_dora_indicators();
    assert!(result.is_err());
}

/// 测试花牌补充
#[test]
fn test_flower_replacement() {
    // 创建带有花牌的牌墙
    let mut wall = Wall::new(
        WallConfig::MCR,
        Some(DeadWallConfig::MCR { replacement_count: 8 }),
        Some(12345) // 固定种子便于测试
    ).expect("创建牌墙失败");
    
    // 发初始手牌（假设其中包含了花牌，需要补牌）
    let initial_hand = wall.deal_initial_hand(13).expect("发初始手牌失败");
    wall.start_game();
    
    // 模拟检测到花牌并进行补牌
    let mut final_hand = initial_hand;
    
    // 假设我们检测到了2张花牌需要补充
    for _ in 0..2 {
        let replacement = wall.draw_replacement_tile().expect("补花失败");
        final_hand.push(replacement);
    }
    
    // 验证最终手牌数量
    assert_eq!(final_hand.len(), 15); // 原13张 + 2张补花
    
    // 验证已经摸出的牌数统计正确
    assert_eq!(wall.drawn_count(), 15); // 13张初始手牌 + 2张补花
}