// src/rules/mod.rs

use crate::hand::Hand;
use crate::meld::Meld;
use crate::game::context::GameContext;
use crate::tile::Tile;

/// 定义规则集需要实现的 Trait
pub trait RuleSet {
    /// 获取规则名称
    fn name(&self) -> &str;

    /// 判断手牌是否和牌 (包括检查番缚等)
    fn can_win(&self, hand: &Hand, context: &GameContext, is_tsumo: bool) -> bool;

    /// 计算和牌的点数和役种
    fn calculate_score(&self, hand: &Hand, context: &GameContext, win_tile: Tile, is_tsumo: bool) -> Option<ScoreResult>;

    /// 判断是否可以立直
    fn can_riichi(&self, hand: &Hand, context: &GameContext) -> bool;

    /// 判断是否可以吃牌
    fn can_chi(&self, hand: &Hand, discarded_tile: Tile, context: &GameContext) -> Vec<(Tile, Tile, Tile)>; // 返回所有可能的吃牌组合

    /// 判断是否可以碰牌
    fn can_pon(&self, hand: &Hand, discarded_tile: Tile, context: &GameContext) -> bool;

    /// 判断是否可以杠牌 (包括明杠、暗杠、加杠)
    fn can_kan(&self, hand: &Hand, tile: Tile, from_discard: bool, context: &GameContext) -> bool;

    // 可以添加其他规则相关的判断，例如流局条件、特殊役种等
}

/// 计分结果
#[derive(Debug, Clone)]
pub struct ScoreResult {
    pub yaku: Vec<Yaku>, // 役种列表
    pub fu: u32,         // 符数
    pub score: i32,      // 总点数 (支付给和牌者)
    pub han: u32,        // 番数
}

/// 役种定义 (示例)
#[derive(Debug, Clone)]
pub struct Yaku {
    pub name: String,
    pub han: u32,
}

// 导出子模块
pub mod common;
pub mod riichi;
pub mod shanghai;
pub mod mcr;

// 可以提供一个函数来根据配置获取具体的规则集实例
// pub fn get_rule_set(config: &RuleConfig) -> Box<dyn RuleSet> { ... }

// #[derive(Debug, Clone)]
// pub enum RuleConfig {
//     Riichi,
//     Shanghai,
//     Mcr,
// }
