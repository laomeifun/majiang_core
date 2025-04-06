// src/hand/mod.rs
// 导出 Hand 相关内容

pub mod representation;
pub mod analysis;
pub mod efficiency;
pub mod parser;

// 重新导出核心类型和功能
pub use representation::Hand;
pub use analysis::{analyze_hand, ShantenCalculator, HandAnalysisResult}; // 假设的类型
pub use efficiency::calculate_efficiency; // 假设的函数
pub use parser::parse_hand_string; // 假设的函数
