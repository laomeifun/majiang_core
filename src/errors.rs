// src/errors.rs
//
// 本文件定义了麻将游戏核心库中使用的所有错误类型
// 错误处理是麻将引擎健壮性的重要保障，所有可能失败的操作都应返回 MajiangResult
// 而不是使用 panic 或 unwrap

/// 麻将游戏中的所有错误类型
///
/// 这个枚举定义了麻将游戏中可能出现的各种错误情况，分为以下几类：
/// 
/// # 资源类错误
/// - `NotEnoughTiles`: 牌墙资源不足
/// - `InsufficientResources`: 其他资源（如点数）不足
/// 
/// # 规则类错误
/// - `RuleViolation`: 违反游戏规则的操作
/// - `InvalidWinningHand`: 不符合和牌条件
/// 
/// # 操作类错误
/// - `InvalidOperation`: 在不适当时机的操作
/// - `InvalidAction`: 玩家不合法的动作
/// - `InvalidMeld`: 不符合规则的副露操作
/// - `TileNotFound`: 使用不在手中的牌
/// 
/// # 系统类错误
/// - `InvalidState`: 游戏状态异常
/// - `InternalError`: 内部逻辑错误
/// - `ConfigurationError`: 配置参数错误
/// - `SerializationError`: 数据转换错误
/// 
/// # 数据类错误
/// - `InvalidTile`: 无效的牌或牌数据
///
/// # 示例
/// 
/// ```
/// use majiang_core::errors::{MajiangError, MajiangResult};
/// 
/// // 创建一个可能返回错误的函数
/// fn check_tile_in_hand(hand: &[u8], tile: u8) -> MajiangResult<()> {
///     if !hand.contains(&tile) {
///         return Err(MajiangError::TileNotFound);
///     }
///     Ok(())
/// }
/// 
/// // 处理错误结果
/// fn discard_tile(hand: &[u8], tile: u8) -> MajiangResult<()> {
///     match check_tile_in_hand(hand, tile) {
///         Ok(_) => {
///             // 执行打牌逻辑
///             Ok(())
///         },
///         Err(e) => {
///             // 可以进一步包装错误信息
///             Err(MajiangError::InvalidAction(format!(
///                 "无法打出不存在的牌: {}", e
///             )))
///         }
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MajiangError {
    /// 牌墙中没有足够的牌
    /// 
    /// 在摸牌、开杠等需要从牌墙摸牌的操作中，牌墙剩余牌数不足时返回此错误
    NotEnoughTiles,
    
    /// 无效的操作
    /// 
    /// 在不适当的时机或状态下尝试执行操作时返回此错误，例如：
    /// - 在非自己回合尝试进行操作
    /// - 在已经碰牌后尝试杠牌
    /// - 在游戏未开始时尝试打牌
    InvalidOperation(String),
    
    /// 无效的牌
    /// 
    /// 当尝试使用或创建不符合规则的牌时返回此错误，例如：
    /// - 创建点数超出范围的数牌（如 "10万"）
    /// - 使用不存在的牌种类
    /// - 访问无效的牌索引
    InvalidTile(String),
    
    /// 规则冲突
    /// 
    /// 当操作违反特定规则或者组合了不兼容的规则设置时返回此错误，例如：
    /// - 在某些规则下不允许的特定和牌形式（如国士无双）
    /// - 同时启用了相互冲突的规则选项
    RuleViolation(String),
    
    /// 无效的游戏状态
    /// 
    /// 当游戏处于不一致状态或试图进行不适当的状态转换时返回此错误，例如：
    /// - 在一局游戏结束后尝试继续操作
    /// - 状态机转换到不合法的下一状态
    InvalidState(String),
    
    /// 无效的玩家操作
    /// 
    /// 当玩家尝试执行不被当前规则或状态允许的动作时返回此错误，例如：
    /// - 尝试吃不能吃的牌
    /// - 在立直后改变打牌策略
    InvalidAction(String),
    
    /// 牌不在手牌中
    /// 
    /// 当尝试使用、打出或组合不在玩家手中的牌时返回此错误
    TileNotFound,
    
    /// 无效的副露操作
    /// 
    /// 当尝试进行不符合规则的副露（吃/碰/杠）操作时返回此错误，例如：
    /// - 尝试碰不同的牌
    /// - 尝试吃不连续的牌
    /// - 暗杠不足四张相同的牌
    InvalidMeld(String),
    
    /// 无效的和牌组合
    /// 
    /// 当牌型不符合和牌要求，或缺少必要的役种时返回此错误，例如：
    /// - 没有达成任何役种的和牌尝试
    /// - 听牌不正确
    /// - 不符合和牌牌型要求（如未形成四组面子+一对雀头）
    InvalidWinningHand(String),
    
    /// 内部逻辑错误
    /// 
    /// 当程序内部状态不一致，通常表示代码存在bug时返回此错误
    /// 这类错误通常不应该被终端用户看到，而应在开发阶段被修复
    InternalError(String),
    
    /// 配置错误
    /// 
    /// 当游戏配置参数有误或配置选项之间存在冲突时返回此错误，例如：
    /// - 玩家数量不符合规则要求
    /// - 点数设置不合理
    ConfigurationError(String),
    
    /// 序列化/反序列化错误
    /// 
    /// 在游戏状态、动作或其他数据的转换过程中出现问题时返回此错误
    SerializationError(String),
    
    /// 资源不足
    /// 
    /// 当玩家所拥有的资源不足以支付某个操作的成本时返回此错误，例如：
    /// - 点数不足以支付立直条件
    /// - 无法满足最低点数要求
    InsufficientResources(String),
}

impl std::fmt::Display for MajiangError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotEnoughTiles => write!(f, "牌墙中没有足够的牌"),
            Self::InvalidOperation(msg) => write!(f, "无效的操作: {}", msg),
            Self::InvalidTile(msg) => write!(f, "无效的牌: {}", msg),
            Self::RuleViolation(msg) => write!(f, "规则冲突: {}", msg),
            Self::InvalidState(msg) => write!(f, "无效的游戏状态: {}", msg),
            Self::InvalidAction(msg) => write!(f, "无效的玩家操作: {}", msg),
            Self::TileNotFound => write!(f, "牌不在手牌中"),
            Self::InvalidMeld(msg) => write!(f, "无效的副露操作: {}", msg),
            Self::InvalidWinningHand(msg) => write!(f, "无效的和牌组合: {}", msg),
            Self::InternalError(msg) => write!(f, "内部逻辑错误: {}", msg),
            Self::ConfigurationError(msg) => write!(f, "配置错误: {}", msg),
            Self::SerializationError(msg) => write!(f, "序列化/反序列化错误: {}", msg),
            Self::InsufficientResources(msg) => write!(f, "资源不足: {}", msg),
        }
    }
}

impl std::error::Error for MajiangError {}

/// 麻将游戏结果类型
/// 
/// 在整个代码库中使用此类型表示可能失败的操作。使用 Result 模式可以：
/// - 避免使用 panic 或 unwrap，提高程序健壮性
/// - 通过类型系统强制错误处理
/// - 利用 ? 运算符简化错误传播
/// 
/// # 示例
/// 
/// ```
/// use majiang_core::errors::{MajiangError, MajiangResult};
/// 
/// fn draw_tile() -> MajiangResult<u8> {
///     // 实际实现会从牌墙中摸牌
///     Ok(1) // 假设摸到了1万
/// }
/// 
/// fn player_turn() -> MajiangResult<()> {
///     // 使用 ? 运算符处理可能的错误
///     let tile = draw_tile()?;
///     println!("摸到了牌: {}", tile);
///     Ok(())
/// }
/// ```
pub type MajiangResult<T> = Result<T, MajiangError>;
