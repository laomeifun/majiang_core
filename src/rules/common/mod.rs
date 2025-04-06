// src/rules/common/mod.rs

// 导出 common 模块下的文件
pub mod win_patterns;

// 可以重新导出常用的函数或结构体
pub use win_patterns::is_standard_win_shape;
