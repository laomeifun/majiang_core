// src/rules/mcr/mod.rs

use crate::hand::Hand;
use crate::game::context::GameContext;
use crate::rules::{RuleSet, ScoreResult};
use crate::tile::Tile;

// 导出 mcr 模块下的子模块
pub mod win_check;
pub mod scoring;
pub mod flower; // 国标也有花牌

/// 国标麻将 (Mahjong Competition Rules) 规则集实现 (占位符)
#[derive(Debug, Default)]
pub struct McrRuleSet;

impl RuleSet for McrRuleSet {
    fn name(&self) -> &str {
        "Mahjong Competition Rules (MCR)"
    }

    fn can_win(&self, hand: &Hand, context: &GameContext, is_tsumo: bool) -> bool {
        // 实现国标麻将的和牌判断逻辑 (需要至少 8 分)
        win_check::can_win_mcr(hand, context, is_tsumo)
    }

    fn calculate_score(&self, hand: &Hand, context: &GameContext, win_tile: Tile, is_tsumo: bool) -> Option<ScoreResult> {
        // 实现国标麻将的计分逻辑 (计算番种得分)
        scoring::calculate_mcr_score(hand, context, win_tile, is_tsumo)
    }

    // 国标麻将没有立直
    fn can_riichi(&self, _hand: &Hand, _context: &GameContext) -> bool {
        false
    }

    fn can_chi(&self, hand: &Hand, discarded_tile: Tile, context: &GameContext) -> Vec<(Tile, Tile, Tile)> {
        // 实现吃牌判断逻辑
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
