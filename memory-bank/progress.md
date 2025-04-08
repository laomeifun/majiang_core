# 项目进展

## 已完成功能

*   项目结构初始化 (`cargo new majiang_core --lib`)。
*   Git 仓库初始化。
*   记忆库 (`memory-bank/`) 初始化，包含所有核心文件：
    *   `projectbrief.md`
    *   `productContext.md`
    *   `activeContext.md`
    *   `systemPatterns.md`
    *   `techContext.md`
    *   `progress.md` (当前文件)
*   基本的模块结构已在 `src/` 目录下创建。
*   **模块化重构:**
    *   将 `action` 功能拆分到 `src/action/` 目录 (`mod.rs`, `types.rs`, `validation.rs`, `serialization.rs`)。
    *   将 `meld` 功能拆分到 `src/meld/` 目录 (`mod.rs`, `types.rs`, `utils.rs`)。
    *   将 `hand` 功能拆分到 `src/hand/` 目录 (`mod.rs`, `representation.rs`, `analysis.rs`, `efficiency.rs`, `parser.rs`)。
    *   删除了旧的 `src/action.rs`, `src/meld.rs`, `src/hand.rs` 文件。
*   **文档更新:**
    *   更新 `README.md` 以反映新的文件结构。
    *   为 `README.md` 中的文件结构添加了详细的作用说明。
*   **测试结构:**
    *   创建了新的测试目录结构 (`tests/tile/`, `tests/meld/`, `tests/hand/`, `tests/wall/`, `tests/action/`, `tests/player/`, `tests/game/`, `tests/rules/riichi/`, `tests/rules/shanghai/`)。
    *   删除了误创建的 `tests/common/` 目录。
    *   更新了 `README.md` 以反映最终的测试结构。

## 待构建功能

*   **基础数据结构:**
    *   `Tile` (牌) 的完整定义、常量和辅助函数 (`src/tile/`)。
    *   `Meld` (面子) 的定义和实现 (`src/meld/`)。
*   **核心机制:**
    *   `Wall` (牌墙) 的生成、洗牌、发牌、补花（如果规则需要）等逻辑 (`src/wall/`)。
    *   `Hand` (手牌) 的管理和分析逻辑实现 (`src/hand/`)。
*   **玩家动作:**
    *   定义和实现 `Action` 相关逻辑 (`src/action/`)。
    *   实现各种动作的验证逻辑。
*   **游戏流程:**
    *   定义 `GameState` (`src/game/state.rs`)。
    *   实现游戏主循环/状态机 (`src/game/flow.rs`)。
    *   管理游戏上下文信息 (`src/game/context.rs`)。
*   **规则实现:**
    *   定义通用的 `RuleSet` trait (`src/rules/mod.rs`)。
    *   实现至少一种具体规则集（如 MCR 或 Riichi） (`src/rules/mcr/` 或 `src/rules/riichi/`)：
        *   和牌检查 (`win_check.rs`)。
        *   计分/番种计算 (`scoring.rs` / `yaku.rs`)。
        *   特定规则逻辑（如立直判断、花牌处理）。
*   **错误处理:**
    *   完善 `Error` 类型 (`src/errors.rs`)。
*   **测试:**
    *   为所有核心逻辑编写单元测试和集成测试。
*   **文档:**
    *   编写 API 文档注释 (`///`)。
    *   完善记忆库文档。

## 当前状态

*   完成了基础设置、文档初始化以及 `action`, `meld`, `hand` 模块的结构化拆分。
*   创建了详细的测试目录结构。
*   `README.md` 已更新以反映当前的源代码和测试结构，并包含说明。
*   核心的游戏逻辑和模块内部实现（如向听数计算、牌效率、动作验证等）仍处于占位符或待实现状态。
*   存在大量编译错误，主要由模块拆分后的路径引用、类型/方法未找到等问题引起，需要在后续实现中解决。

## 已知问题

*   大量编译错误，需要在填充模块内容时修复。
*   (已删除 `tests/common/` 目录及其内容)

## 项目决策演变

*   决定将核心功能 (`action`, `meld`, `hand`) 拆分为独立的子模块，以提高代码组织性和可维护性。
*   决定为每个核心模块创建独立的测试目录，而不是使用一个通用的 `tests/common`。
