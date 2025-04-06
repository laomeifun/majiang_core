mahjong-core/
├── Cargo.toml
└── src/
    ├── lib.rs              # 库入口，统一导出各大模块

    ├── tile.rs             # 基础牌定义（牌，花色，风牌，箭牌等）
    ├── meld.rs             # 副露相关结构和功能
    ├── hand.rs             # 手牌结构和基础分析逻辑
    ├── wall.rs             # 牌墙、洗牌、发牌逻辑
    ├── action.rs           # 定义动作类型（吃碰杠立直和荣等）
    ├── errors.rs           # 全局错误类型定义

    ├── player/             # **玩家相关代码分模块化**
    │   ├── mod.rs          # 玩家模块统一出口
    │   ├── model.rs        # 玩家状态与数据结构
    │   ├── actions.rs      # 玩家能执行的行为逻辑
    │   ├── ai.rs           # AI 玩家逻辑
    │   ├── agent.rs        # 控制接口，封装真人、AI或远程玩家
    │   └── utils.rs        # 玩家相关辅助函数

    ├── game/               # **游戏整体状态和流程管理相关**
    │   ├── mod.rs          # game子模块统一出口
    │   ├── state.rs        # 游戏的主状态结构体，持有全局状态信息
    │   ├── context.rs      # 规则判断等需用到的上下文信息（场风、宝牌等）
    │   ├── turn.rs         # 单个回合的流程控制
    │   ├── flow.rs         # 整个一局/一轮的流程控制
    │   └── utils.rs        # 游戏相关辅助方法

    ├── rules/              # **多规则支持的规则实现模块**
    │   ├── mod.rs          # 定义规则接口(RuleSet Trait)、全局配置
    │   ├── common/         # 规则共享的通用基础代码
    │   │   └── win_patterns.rs    # 标准和牌型识别等
    │   ├── riichi/         # 日本麻将规则实现
    │   │   ├── mod.rs
    │   │   ├── win_check.rs
    │   │   ├── yaku.rs
    │   │   ├── score.rs
    │   │   └── riichi_specific.rs
    │   ├── shanghai/       # 上海麻将规则实现
    │   │   ├── mod.rs
    │   │   ├── win_check.rs
    │   │   ├── scoring.rs
    │   │   └── flower.rs
    │   └── mcr/            # 国际规则/国标麻将实现
    │       ├── mod.rs
    │       ├── win_check.rs
    │       ├── scoring.rs
    │       └── flower.rs

    ├── tests/              # 测试代码
    │   ├── common/         # tile、hand、meld 等基础功能测试
    │   ├── player/         # 玩家相关测试
    │   ├── game/           # 游戏状态与流程相关测试
    │   └── rules/          # 不同规则的判定与计分测试
    │       ├── riichi/
    │       ├── shanghai/
    │       └── mcr/