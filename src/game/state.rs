// src/game/state.rs

use crate::player::model::{Player, WindDirection};
use crate::player::agent::PlayerAgent; // 引入 PlayerAgent
use crate::wall::Wall;
use crate::tile::Tile;
// use crate::rules::RuleSet; // 需要引入规则集

/// 游戏的主状态结构体
#[derive(Debug)]
pub struct GameState {
    pub players: Vec<Player>,       // 游戏中的所有玩家
    pub agents: Vec<PlayerAgent>,   // 每个玩家对应的代理 (Human/AI/Remote)
    pub wall: Wall,                 // 当前牌墙
    pub current_turn: usize,        // 当前轮到哪个玩家 (索引 0-3)
    pub dealer_index: usize,        // 当前庄家索引
    pub round_wind: WindDirection,  // 当前场风
    pub round_number: u8,           // 当前局数 (例如 东1局, 南2局)
    pub honba: u8,                  // 本场数
    pub riichi_sticks: u8,          // 立直棒数量
    pub dora_indicators: Vec<Tile>, // 当前宝牌指示牌
    pub last_discard: Option<Tile>, // 上一个被打出的牌 (用于判断荣和、吃碰杠)
    // pub rule_set: Box<dyn RuleSet>, // 使用的规则集
    // 可以添加更多状态，例如游戏阶段 (发牌、行牌、结束等)
    // pub phase: GamePhase,
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum GamePhase {
//     Dealing, // 发牌阶段
//     Playing, // 行牌阶段
//     RoundOver, // 一局结束
//     GameOver, // 整个游戏结束
// }

impl GameState {
    /// 创建一个新的游戏状态 (示例，需要更完整的初始化逻辑)
    pub fn new(/* rule_set: Box<dyn RuleSet>, agent_types: [PlayerAgent; 4] */) -> Self {
        let initial_score = 25000; // 初始分数示例
        let players = vec![
            Player::new(0, initial_score, WindDirection::East),
            Player::new(1, initial_score, WindDirection::South),
            Player::new(2, initial_score, WindDirection::West),
            Player::new(3, initial_score, WindDirection::North),
        ];
        // let agents = agent_types.into(); // 从输入参数创建 agents
        let agents = vec![PlayerAgent::Ai(crate::player::agent::AiType::Simple); 4]; // 示例：4个简单AI

        GameState {
            players,
            agents,
            wall: Wall::new(),
            current_turn: 0, // 东家开始
            dealer_index: 0,
            round_wind: WindDirection::East,
            round_number: 1,
            honba: 0,
            riichi_sticks: 0,
            dora_indicators: Vec::new(), // 需要在发牌后设置
            last_discard: None,
            // rule_set,
            // phase: GamePhase::Dealing,
        }
    }

    /// 获取当前轮次的玩家引用
    pub fn current_player(&self) -> &Player {
        &self.players[self.current_turn]
    }

    /// 获取当前轮次的玩家可变引用
    pub fn current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.current_turn]
    }

     /// 获取当前轮次的玩家代理
    pub fn current_agent(&self) -> &PlayerAgent {
        &self.agents[self.current_turn]
    }

    // 其他游戏状态相关的逻辑，例如发牌、处理动作、进入下一回合等...
}
