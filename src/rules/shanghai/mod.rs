// src/rules/shanghai/mod.rs

use crate::hand::Hand;
use crate::game::context::GameContext;
use crate::rules::{RuleSet, ScoreResult};
use crate::tile::Tile;

// 导出 shanghai 模块下的子模块
pub mod win_check;
pub mod scoring;
pub mod flower;

/// 上海麻将规则集实现 (占位符)
#[derive(Debug, Default)]
pub struct ShanghaiRuleSet;

impl RuleSet for ShanghaiRuleSet {
    fn name(&self) -> &str {
        "Shanghai Mahjong"
    }

    fn can_win(&self, hand: &Hand, context: &GameContext, is_tsumo: bool) -> bool {
        // 实现上海麻将的和牌判断逻辑
        win_check::can_win_shanghai(hand, context)
    }

    fn calculate_score(&self, hand: &Hand, context: &GameContext, win_tile: Tile, is_tsumo: bool) -> Option<ScoreResult> {
        // 实现上海麻将的计分逻辑
        scoring::calculate_shanghai_score(hand, context, win_tile, is_tsumo)
    }

    // 上海麻将通常没有立直
    fn can_riichi(&self, _hand: &Hand, _context: &GameContext) -> bool {
        false
    }

    fn can_chi(&self, hand: &Hand, discarded_tile: Tile, context: &GameContext) -> Vec<(Tile, Tile, Tile)> {
        // 实现吃牌判断逻辑 (可能与日麻不同)
        vec![] // 暂不实现
    }

    fn can_pon(&self, hand: &Hand, discarded_tile: Tile, context: &GameContext) -> bool {
        // 实现碰牌判断逻辑
        hand.get_closed_tiles().iter().filter(|&t| *t == discarded_tile).count() >= 2
    }

    fn can_kan(&self, hand: &Hand, tile: Tile, from_discard: bool, context: &GameContext) -> bool {
        // 实现杠牌判断逻辑
        false // 暂不实现
    }
}
