// src/game/context.rs

use crate::player::model::WindDirection;
use crate::tile::Tile;
use crate::game::state::GameState; // 引入 GameState

/// 包含规则判断等需要的上下文信息
/// 通常是 GameState 的一部分或者从 GameState 派生
#[derive(Debug, Clone)]
pub struct GameContext<'a> {
    // 使用生命周期 'a 将 context 与 GameState 关联起来，避免复制大量数据
    pub game_state: &'a GameState,
    // 可以添加一些计算好的或常用的信息，方便规则判断
    // 例如：当前玩家的风牌、场风、宝牌列表等
    pub player_wind: WindDirection,
    pub round_wind: WindDirection,
    pub dora: Vec<Tile>, // 包括指示牌和里宝牌指示牌对应的实际宝牌
    // pub ura_dora: Vec<Tile>, // 如果需要区分表里宝牌
}

impl<'a> GameContext<'a> {
    /// 从 GameState 创建 GameContext
    pub fn from_game_state(game_state: &'a GameState, player_index: usize) -> Self {
        let player_wind = game_state.players[player_index].wind;
        let round_wind = game_state.round_wind;
        // 计算实际的宝牌 (需要规则逻辑)
        let dora = calculate_dora(&game_state.dora_indicators);

        GameContext {
            game_state,
            player_wind,
            round_wind,
            dora,
        }
    }

    // 提供访问 GameState 信息的便捷方法
    pub fn current_turn(&self) -> usize {
        self.game_state.current_turn
    }

    pub fn dealer_index(&self) -> usize {
        self.game_state.dealer_index
    }

    // ... 其他便捷访问方法
}

/// 计算实际宝牌的函数 (示例，需要具体规则实现)
fn calculate_dora(indicators: &[Tile]) -> Vec<Tile> {
    // 根据指示牌计算下一张牌作为宝牌
    // 需要处理边界情况（例如 9->1, 风牌循环, 箭牌循环）
    indicators.iter().map(|_indicator| {
        // 实际的宝牌计算逻辑
        Tile::man(1).unwrap() // 临时返回一个固定值
    }).collect()
}
