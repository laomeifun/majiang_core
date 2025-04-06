majiang-core/
├── .gitignore              # Git 忽略文件配置
├── Cargo.lock              # 锁定项目依赖的精确版本
├── Cargo.toml              # Rust 项目配置文件 (元数据, 依赖等)
├── README.md               # 项目说明文档 (本文件)
├── clinerules/             # (推测) Cline 工具的自定义规则目录
├── memory-bank/            # (推测) Cline 工具的上下文记忆库目录
├── src/                    # 项目源代码根目录
│   ├── errors.rs           # 定义项目自定义的错误类型
│   ├── lib.rs              # Rust 库的入口点, 声明和导出模块
│   ├── action/             # 玩家动作模块 (吃, 碰, 杠, 立直, 和牌等)
│   │   ├── mod.rs          # action 模块的入口和导出
│   │   ├── serialization.rs# 动作的序列化/反序列化逻辑 (可选)
│   │   ├── types.rs        # 定义 Action 枚举, ActionResult 等核心类型
│   │   └── validation.rs   # 动作合法性验证逻辑
│   ├── game/               # 游戏流程和状态管理模块
│   │   ├── context.rs      # 游戏上下文信息 (场风, 自风, 宝牌指示牌等)
│   │   ├── flow.rs         # 游戏整体流程控制 (开局, 换庄, 结束等)
│   │   ├── mod.rs          # game 模块的入口和导出
│   │   ├── state.rs        # 定义游戏主状态结构 (包含玩家, 牌墙, 当前轮次等)
│   │   ├── turn.rs         # 单个玩家回合的流程管理
│   │   └── utils.rs        # 游戏相关的辅助函数
│   ├── hand/               # 手牌表示和分析模块
│   │   ├── analysis.rs     # 核心手牌分析 (向听数计算, 听牌判断, 牌型分解)
│   │   ├── efficiency.rs   # 牌效率计算 (计算打哪张牌最优, 进张分析)
│   │   ├── mod.rs          # hand 模块的入口和导出
│   │   ├── parser.rs       # 手牌字符串表示的解析逻辑
│   │   └── representation.rs # 定义 Hand 结构体及基础操作 (加牌, 减牌, 副露管理)
│   ├── meld/               # 副露 (吃, 碰, 杠) 表示模块
│   │   ├── mod.rs          # meld 模块的入口和导出
│   │   ├── types.rs        # 定义 Meld 结构体, MeldType, KanType 等
│   │   └── utils.rs        # 副露相关的辅助函数
│   ├── player/             # 玩家相关逻辑模块
│   │   ├── actions.rs      # 玩家可执行动作的封装 (结合手牌和游戏状态)
│   │   ├── agent.rs        # 玩家代理接口 (区分人, AI, 网络玩家等)
│   │   ├── ai.rs           # AI 玩家的决策逻辑
│   │   ├── mod.rs          # player 模块的入口和导出
│   │   ├── model.rs        # 玩家数据模型 (手牌, 副露, 河牌, 分数等)
│   │   └── utils.rs        # 玩家相关的辅助函数
│   ├── rules/              # 不同麻将规则的实现模块
│   │   ├── mod.rs          # rules 模块入口, 定义 RuleSet trait 接口
│   │   ├── common/         # 多种规则通用的逻辑
│   │   │   ├── mod.rs      # common 规则模块入口
│   │   │   └── win_patterns.rs # 通用和牌型判断 (如标准 4 面子 1 雀头)
│   │   ├── mcr/            # 国标麻将 (Mahjong Competition Rules) 规则实现
│   │   │   ├── flower.rs   # 国标花牌处理
│   │   │   ├── mod.rs      # mcr 规则模块入口
│   │   │   ├── scoring.rs  # 国标计分逻辑
│   │   │   └── win_check.rs# 国标和牌检查
│   │   ├── riichi/         # 日本立直麻将规则实现
│   │   │   ├── mod.rs      # riichi 规则模块入口
│   │   │   ├── riichi_specific.rs # 立直麻将特有逻辑 (立直判断, 一发, 里宝牌等)
│   │   │   ├── score.rs    # 立直麻将计分逻辑 (符数, 番数计算)
│   │   │   ├── win_check.rs# 立直麻将和牌检查
│   │   │   └── yaku.rs     # 立直麻将役种定义和判断
│   │   ├── shanghai/       # 上海麻将规则实现
│   │   │   ├── flower.rs   # 上海麻将花牌处理
│   │   │   ├── mod.rs      # shanghai 规则模块入口
│   │   │   ├── scoring.rs  # 上海麻将计分逻辑
│   │   │   └── win_check.rs# 上海麻将和牌检查
│   ├── tile/               # 麻将牌表示模块
│   │   ├── display.rs      # Tile 的显示逻辑 (如转为字符串 "1m", "東")
│   │   ├── mod.rs          # tile 模块的入口和导出
│   │   ├── tile.rs         # 定义 Tile 结构体
│   │   └── types.rs        # 定义牌的类型枚举 (花色 Suit, 风 Wind, 箭牌 Dragon)
│   └── wall/               # 牌墙管理模块
│       ├── builder.rs      # 构建初始牌墙的逻辑 (生成牌, 洗牌)
│       ├── dead_wall.rs    # 岭上牌(杠后摸牌)、宝牌指示牌等逻辑
│       └── mod.rs          # wall 模块入口, 定义 Wall 结构体及发牌逻辑
├── target/                 # 编译输出目录 (通常在 .gitignore 中忽略)
└── tests/                  # 测试代码目录
    ├── test_tile.rs        # tile 模块的单元/集成测试
    └── common/             # 测试用的通用工具或数据
        ├── ruel.rs         # (可能是 rule.rs 的拼写错误?) 测试规则相关的通用帮助函数
        └── test_tile.rs    # 通用测试中与 tile 相关的部分

# 说明
这里展示了项目的主要结构及各文件/目录的作用。
- `src/` 包含核心库代码。
- 各子目录 (action, game, hand, meld, player, rules, tile, wall) 对应不同的功能模块。
- `rules/` 下按不同麻将规则分子目录。
- `tests/` 包含集成测试和单元测试。
- `target/` 是编译输出目录。
- `memory-bank/` 和 `clinerules/` 可能是辅助目录。
