// src/action/mod.rs
// 这个文件将导出 action 模块的公共接口

pub mod types;
pub mod validation;
pub mod serialization;

// 重新导出重要的类型，方便外部使用
pub use types::{Action, ActionResult};
