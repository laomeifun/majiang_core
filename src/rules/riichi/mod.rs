// src/rules/riichi/mod.rs

use crate::hand::Hand;
use crate::game::context::GameContext;
use crate::rules::{RuleSet, ScoreResult, Yaku};
use crate::tile::Tile;
use crate::rules::common::win_patterns; // 引入通用和牌型判断

// 导出 riichi 模块下的子模块
pub mod win_check;
pub mod yaku;
pub mod score;
pub mod riichi_specific;

/// 日本麻将规则集实现
#[derive(Debug, Default)]
pub struct RiichiRuleSet;

impl RuleSet for RiichiRuleSet {
    fn name(&self) -> &str {
        "Riichi Mahjong"
    }

    fn can_win(&self, hand: &Hand, context: &GameContext, is_tsumo: bool) -> bool {
        // 1. 检查基本和牌型 (4面子+1雀头, 七对子, 国士无双等)
        if !win_check::can_form_winning_hand(hand) {
            return false;
        }

        // 2. 检查是否有役 (至少一番)
        let score_result = self.calculate_score(hand, context, Tile::man(1).unwrap(), is_tsumo); // 临时传入一个 win_tile
        match score_result {
            Some(result) => result.han >= 1, // 至少需要一番才能和牌
            None => false, // 无法计算分数，说明无役或牌型错误
        }
        // 实际检查会更复杂，例如振听判断等
    }

    fn calculate_score(&self, hand: &Hand, context: &GameContext, win_tile: Tile, is_tsumo: bool) -> Option<ScoreResult> {
        // 调用 yaku.rs 计算役种和番数
        let yaku_list = yaku::calculate_yaku(hand, context, win_tile, is_tsumo);
        if yaku_list.is_empty() {
            return None; // 无役
        }

        // 调用 score.rs 计算符数和最终点数
        score::calculate_riichi_score(hand, context, win_tile, is_tsumo, yaku_list)
    }

    fn can_riichi(&self, hand: &Hand, context: &GameContext) -> bool {
        riichi_specific::check_riichi_conditions(hand, context)
    }

    fn can_chi(&self, hand: &Hand, discarded_tile: Tile, context: &GameContext) -> Vec<(Tile, Tile, Tile)> {
        // 实现吃牌判断逻辑
        // 需要检查是否是上家打出的牌，手牌中是否有合适的组合等
        vec![] // 暂不实现
    }

    fn can_pon(&self, hand: &Hand, discarded_tile: Tile, context: &GameContext) -> bool {
        // 实现碰牌判断逻辑
        // 需要检查手牌中是否有两张相同的牌
        hand.get_closed_tiles().iter().filter(|&t| *t == discarded_tile).count() >= 2
        // 实际还需要考虑不能碰自己打出的牌等细节
    }

    fn can_kan(&self, hand: &Hand, tile: Tile, from_discard: bool, context: &GameContext) -> bool {
        // 实现杠牌判断逻辑 (复杂，需要区分不同类型的杠)
        false // 暂不实现
    }
}
