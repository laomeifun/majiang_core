# 技术背景

## 使用的技术

*   **编程语言:** Rust (Stable toolchain)
*   **构建工具:** Cargo (Rust 的包管理器和构建系统)
*   **版本控制:** Git
*   **核心依赖:** (目前主要是 Rust 标准库，后续可能添加)
    *   `rand`: 用于洗牌等随机操作。
    *   `serde`: (可选) 用于序列化/反序列化游戏状态。
    *   `thiserror`: (可选) 简化错误类型的定义。
    *   `log`: (可选) 用于日志记录。

## 开发环境设置

*   **安装 Rust:** 通过 `rustup` (https://rustup.rs/) 安装 Rust 工具链。
*   **代码编辑器:** 推荐使用支持 Rust Language Server (RLS) 或 `rust-analyzer` 的编辑器，如 VS Code。
    *   **VS Code 扩展:**
        *   `rust-analyzer`
        *   `crates` (管理 Cargo.toml 依赖)
        *   `Even Better TOML` (TOML 文件语法高亮)
*   **构建项目:** 在项目根目录运行 `cargo build`。
*   **运行测试:** 运行 `cargo test`。
*   **代码格式化:** 使用 `cargo fmt`。
*   **代码检查:** 使用 `cargo clippy`。

## 技术约束

*   **内存安全:** 充分利用 Rust 的所有权和借用检查机制，避免内存安全问题。
*   **性能:** 在关键路径（如和牌检查、AI 计算）需要关注性能，但优先保证代码的正确性和可读性。
*   **跨平台:** 核心库应能在主流操作系统（Windows, macOS, Linux）上编译和运行。
*   **依赖管理:** 谨慎添加外部依赖，保持核心库的轻量级。

## 依赖项

*   当前依赖项定义在 `Cargo.toml` 文件中。
*   使用 `cargo update` 更新依赖项。
*   使用 `cargo tree` 查看依赖关系树。

## 工具使用模式

*   **Cargo:** 用于构建、测试、运行、格式化、检查、依赖管理等所有与 Rust 项目相关的操作。
*   **Git:** 用于版本控制，遵循常规的 Git 工作流（如 feature branches, pull requests）。
*   **调试器:** 可以使用 GDB/LLDB 或编辑器集成的调试器进行调试。
