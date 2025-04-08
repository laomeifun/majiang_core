# 系统模式

## 系统架构

*   **核心库:** `majiang_core` 作为主要的 Rust crate。
*   **模块化设计:**
    *   `src/tile/`: 定义麻将牌 (`tile.rs`, `types.rs`) 及其显示 (`display.rs`)。
    *   `src/wall/`: 管理牌墙 (`mod.rs`), 构建 (`builder.rs`), 死墙 (`dead_wall.rs`)。
    *   `src/hand/`: 管理玩家手牌 (`representation.rs`), 分析 (`analysis.rs`), 效率计算 (`efficiency.rs`), 解析 (`parser.rs`)。
    *   `src/meld/`: 定义面子 (`types.rs`) 和相关工具 (`utils.rs`)。
    *   `src/action/`: 定义玩家动作 (`types.rs`), 验证 (`validation.rs`), 序列化 (`serialization.rs`)。
    *   `src/game/`: 包含游戏状态管理 (`state.rs`), 流程控制 (`flow.rs`, `turn.rs`), 上下文 (`context.rs`)。
    *   `src/player/`: 定义玩家模型 (`model.rs`), 动作 (`actions.rs`), AI (`ai.rs`), 代理 (`agent.rs`)。
    *   `src/rules/`: 包含不同麻将规则的实现 (`mod.rs` 定义 trait)。
        *   `src/rules/common/`: 通用规则逻辑或工具 (`win_patterns.rs`)。
        *   `src/rules/mcr/`: 国标麻将 (MCR) 特定规则 (`win_check.rs`, `scoring.rs`, `flower.rs`)。
        *   `src/rules/riichi/`: 日本麻将 (Riichi) 特定规则 (`win_check.rs`, `yaku.rs`, `score.rs`, `riichi_specific.rs`)。
        *   `src/rules/shanghai/`: 上海麻将特定规则 (`win_check.rs`, `scoring.rs`, `flower.rs`)。
    *   `src/errors.rs`: 定义库特定的错误类型。
    *   `src/lib.rs`: Crate 的入口点，声明和导出公共模块。
    *   `tests/`: 包含单元测试和集成测试。
        *   `tests/tile/`, `tests/meld/`, `tests/hand/`, `tests/wall/`, `tests/action/`, `tests/player/`, `tests/game/`: 各模块对应的测试目录。
        *   `tests/rules/`: 按规则组织的测试目录 (`riichi/`, `shanghai/` 等)。
    *   **测试组织原则:** 测试模块应尽可能小，按功能细分到不同的文件中，以提高可维护性和清晰度。例如，`tile` 模块的测试被拆分为 `creation_tests.rs`, `identification_tests.rs` 等。

```mermaid
graph TD
    subgraph majiang_core
        direction LR
        Lib[lib.rs] --> ActionMod[action/mod.rs]
        Lib --> TileMod[tile/mod.rs]
        Lib --> WallMod[wall/mod.rs]
        Lib --> HandMod[hand/mod.rs]
        Lib --> MeldMod[meld/mod.rs]
        Lib --> GameMod[game/mod.rs]
        Lib --> RulesMod[rules/mod.rs]
        Lib --> Errors[errors.rs]
        Lib --> PlayerMod[player/mod.rs] # 添加 Player 模块链接

        GameMod --> GameState[game/state.rs]
        GameMod --> GameFlow[game/flow.rs]
        GameMod --> GameContext[game/context.rs]
        GameMod --> GameTurn[game/turn.rs] # 添加 Turn 链接

        RulesMod --> RulesCommon[rules/common/mod.rs]
        RulesMod --> RulesMCR[rules/mcr/mod.rs]
        RulesMod --> RulesRiichi[rules/riichi/mod.rs]
        RulesMod --> RulesShanghai[rules/shanghai/mod.rs]

        HandMod --> TileMod
        HandMod --> MeldMod # Hand representation uses Meld
        WallMod --> TileMod
        MeldMod --> TileMod
        ActionMod --> TileMod
        ActionMod --> MeldMod # Actions like Kan/Pon/Chi involve Melds
        PlayerMod --> HandMod
        PlayerMod --> ActionMod
        GameMod --> PlayerMod
        GameMod --> WallMod # Game uses Wall

        ActionMod --> ActionTypes[action/types.rs]
        ActionMod --> ActionValidation[action/validation.rs]
        ActionMod --> ActionSerialization[action/serialization.rs]

        MeldMod --> MeldTypes[meld/types.rs]
        MeldMod --> MeldUtils[meld/utils.rs]

        HandMod --> HandRepresentation[hand/representation.rs]
        HandMod --> HandAnalysis[hand/analysis.rs]
        HandMod --> HandEfficiency[hand/efficiency.rs]
        HandMod --> HandParser[hand/parser.rs]

        TileMod --> TileTypes[tile/types.rs]
        TileMod --> TileDisplay[tile/display.rs]
        TileMod --> TileTile[tile/tile.rs]

        WallMod --> WallBuilder[wall/builder.rs]
        WallMod --> WallDeadWall[wall/dead_wall.rs]

        PlayerMod --> PlayerModel[player/model.rs]
        PlayerMod --> PlayerActions[player/actions.rs]
        PlayerMod --> PlayerAgent[player/agent.rs]
        PlayerMod --> PlayerAI[player/ai.rs]

    end

    ExternalApp --> Lib
```

## 关键技术决策

*   **状态机:** 游戏流程 (`src/game/flow.rs`) 将可能使用状态机模式来管理不同的游戏阶段（如发牌阶段、行牌阶段、结算阶段）。
*   **策略模式 / Trait Objects:** 规则的实现 (`src/rules/`) 将利用 Rust 的 trait 系统。定义一个通用的 `RuleSet` trait，不同的麻将规则（MCR, Riichi）实现这个 trait。游戏引擎根据配置选择具体的规则实现。
*   **数据结构:** 谨慎选择合适的数据结构以优化性能，例如使用 `Vec` 或固定大小数组表示手牌，使用哈希表或 BTreeMap 优化查找。
*   **错误处理:** 使用自定义的 `Error` enum (`src/errors.rs`) 和 `Result` 类型来处理可预见的错误情况。

## 设计模式应用

*   **模块化:** 将代码库分解为高内聚、低耦合的模块。
*   **封装:** 隐藏内部实现细节，通过公共 API 暴露功能。
*   **组合优于继承:** 利用 Rust 的 trait 和泛型实现代码复用和多态。

## 组件关系

*   `Game` 模块是核心协调者，它使用 `Wall`, `Hand`, `Action`, `Rules` 等模块来驱动游戏进行。
*   `Rules` 模块定义了游戏的具体行为和约束。
*   `Tile`, `Meld` 是基础数据结构，被多个模块使用。

## 关键实现路径

1.  **基础数据结构:** `Tile`, `Meld`。
2.  **核心机制:** `Wall` (发牌), `Hand` (手牌管理)。
3.  **规则接口:** 定义 `RuleSet` trait。
4.  **游戏流程:** 实现基本的游戏循环和状态转换。
5.  **动作处理:** 实现玩家动作（打牌、摸牌）的逻辑。
6.  **规则实现:** 实现至少一种具体规则（如 MCR）的检查逻辑（吃碰杠胡）。
7.  **计分:** 实现相应规则的计分逻辑。
