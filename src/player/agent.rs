// src/player/agent.rs

use crate::action::{Action, ActionEvent};
use crate::hand::Hand;
use crate::player::model::Player; // 引入 Player 结构体
use crate::player::ai; // 引入 AI 模块
use crate::tile::Tile; // 导入 Tile 类型
// use crate::game::GameContext; // 可能需要游戏上下文

/// 玩家代理类型，可以是真人、AI 或远程玩家
#[derive(Debug, Clone)] // 添加 Debug 和 Clone
pub enum PlayerAgent {
    Human, // 需要外部输入
    Ai(AiType), // 使用 AI 逻辑
    Remote, // 通过网络或其他方式控制
}

/// AI 类型（可以有不同难度的 AI）
#[derive(Debug, Clone, Copy)]
pub enum AiType {
    Simple, // 简单的 AI
    // Intermediate,
    // Advanced,
}

impl PlayerAgent {
    /// 根据代理类型决定玩家的下一个动作
    /// 对于 AI，调用 AI 决策函数
    /// 对于 Human/Remote，需要等待外部输入（这里可以返回 None 或特定状态）
    pub fn decide_action(
        &self,
        player: &Player, // 传入当前玩家状态
        possible_actions: &[Action],
        // context: &GameContext, // 可能需要游戏上下文
    ) -> Option<Action> {
        match self {
            PlayerAgent::Human => {
                // 等待人类玩家输入，这里暂时返回 None
                // 实际应用中，这里会与 UI 或命令行交互
                println!("Player {} (Human), choose action: {:?}", player.id, possible_actions);
                None // 表示需要外部输入
            }
            PlayerAgent::Ai(ai_type) => {
                // 根据 AI 类型选择决策逻辑
                match ai_type {
                    AiType::Simple => {
                        // 调用简单的 AI 决策
                        ai::choose_action(&player.hand, possible_actions /*, context */)
                    }
                    // 其他 AI 类型...
                }
            }
            PlayerAgent::Remote => {
                // 等待远程玩家输入，这里暂时返回 None
                println!("Player {} (Remote), waiting for action...", player.id);
                None // 表示需要外部输入
            }
        }
    }

    /// 处理来自其他玩家的动作事件（例如决定是否吃碰杠和）
    /// 返回自己想要执行的动作，或者 Pass
    pub fn respond_to_action(
        &self,
        player: &Player,
        event: &ActionEvent, // 其他玩家的动作事件
        possible_responses: &[Action], // 自己可以做的响应动作 (Chi, Pon, Kan, Ron, Pass)
        // context: &GameContext,
    ) -> Option<Action> {
         match self {
            PlayerAgent::Human => {
                println!("Player {} (Human), respond to {:?}, possible: {:?}", player.id, event.action, possible_responses);
                None // 等待输入
            }
            PlayerAgent::Ai(_ai_type) => {
                 // 简单 AI：如果不和牌，就 Pass
                 if possible_responses.contains(&Action::Ron(Tile::man(1).unwrap())) { // 简化示例，实际应检查具体的 Ron 动作
                     // 找到 Ron 动作并返回
                     possible_responses.iter().find(|a| matches!(a, Action::Ron(_))).cloned()
                 } else {
                     // 否则总是 Pass (如果 Pass 在列表中)
                     possible_responses.iter().find(|a| matches!(a, Action::Pass)).cloned()
                 }
            }
            PlayerAgent::Remote => {
                 println!("Player {} (Remote), waiting for response to {:?}...", player.id, event.action);
                 None // 等待输入
            }
        }
    }
}
