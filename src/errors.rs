// src/errors.rs

use thiserror::Error; // 可以使用 thiserror 库来简化错误定义

/// 全局错误类型
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum MahjongError {
    #[error("Invalid action: {0}")]
    InvalidAction(String), // 无效的操作

    #[error("Invalid game state: {0}")]
    InvalidState(String), // 无效的游戏状态

    #[error("Rule violation: {0}")]
    RuleViolation(String), // 违反规则

    #[error("Tile not found in hand")]
    TileNotFound, // 手牌中找不到指定的牌

    #[error("Cannot perform action: {0}")]
    ActionNotAllowed(String), // 当前状态不允许该操作

    #[error("Internal error: {0}")]
    InternalError(String), // 内部逻辑错误

    // 可以根据需要添加更多具体的错误类型
}

// 可以定义一个统一的 Result 类型
pub type MahjongResult<T> = Result<T, MahjongError>;
