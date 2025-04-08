// src/errors.rs

/// 麻将游戏中的所有错误类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MajiangError {
    /// 牌墙中没有足够的牌
    NotEnoughTiles,
    
    /// 无效的操作
    /// 例如：在不允许的时机进行操作，执行了当前状态下不合法的动作等
    InvalidOperation(String),
    
    /// 无效的牌
    /// 例如：使用了不存在的牌，或者尝试创建不合法的牌
    InvalidTile(String),
    
    /// 规则冲突
    /// 例如：某些动作违反了特定规则，或者组合了不兼容的规则设置
    RuleViolation(String),
    
    /// 无效的游戏状态
    /// 例如：游戏状态转换错误，或处于不应该的状态
    InvalidState(String),
    
    /// 无效的玩家操作
    /// 例如：玩家尝试使用不在手中的牌，或者执行了不被允许的动作
    InvalidAction(String),
    
    /// 牌不在手牌中
    /// 当尝试打出或使用不在手中的牌时
    TileNotFound,
    
    /// 无效的副露操作
    /// 例如：尝试碰/杠不存在的牌，或不符合规则的杠/碰
    InvalidMeld(String),
    
    /// 无效的和牌组合
    /// 例如：牌型不符合和牌要求，或缺少必要的役种
    InvalidWinningHand(String),
    
    /// 内部逻辑错误
    /// 程序内部的不一致状态，通常表示代码存在bug
    InternalError(String),
    
    /// 配置错误
    /// 例如：规则配置冲突，或者初始化参数错误
    ConfigurationError(String),
    
    /// 序列化/反序列化错误
    /// 在数据转换过程中出现问题
    SerializationError(String),
    
    /// 资源不足
    /// 例如：点数不足以支付某个操作
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
/// 在整个代码库中使用此类型表示可能失败的操作
pub type MajiangResult<T> = Result<T, MajiangError>;
