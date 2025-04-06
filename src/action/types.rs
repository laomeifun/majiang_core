// src/action/types.rs
// 定义 Action 枚举, ActionResult 等

use crate::tile::Tile; // 引入 Tile 类型

/// 杠的类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KanType {
    /// 暗杠 (Closed Kan) - 手牌中有4张相同的牌
    Closed(Tile),
    /// 加杠 (Added Kan) - 碰牌后摸到第四张相同的牌
    Added(Tile),
    /// 大明杠 (Open Kan) - 其他玩家打出第四张相同的牌
    Open(Tile),
}

/// 玩家可以执行的动作
#[derive(Debug, Clone, PartialEq, Eq, Hash)] // 添加 derive
pub enum Action {
    /// 打出一张牌
    Discard(Tile),
    /// 吃牌 (参数：吃的牌，组成的顺子中的两张手牌)
    /// 例如：手牌有 2m, 3m，上家打出 1m，则 Chi(1m, 2m, 3m)
    /// 注意：参数顺序可能需要根据实际使用调整
    Chi(Tile, Tile, Tile),
    /// 碰牌 (参数：碰的牌)
    Pon(Tile),
    /// 杠牌
    Kan(KanType),
    /// 立直
    Riichi,
    /// 自摸和牌
    Tsumo,
    /// 荣和 (参数：荣和的牌)
    Ron(Tile),
    /// 无动作/跳过
    Pass,
    // Placeholder, // 移除占位符
}

/// 动作执行的结果
#[derive(Debug, Clone, PartialEq)] // 添加 derive
pub struct ActionResult {
    /// 动作是否成功执行
    pub success: bool,
    /// （可选）动作产生的效果或信息
    pub message: Option<String>,
    // TODO: 可以添加更多字段，例如状态变更、得分等
}

impl ActionResult {
    /// 创建一个成功的 ActionResult
    pub fn success(message: Option<String>) -> Self {
        ActionResult { success: true, message }
    }

    /// 创建一个失败的 ActionResult
    pub fn failure(message: String) -> Self {
        ActionResult { success: false, message: Some(message) }
    }
}
