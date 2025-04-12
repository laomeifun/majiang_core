// src/meld/mod.rs
//
// 副露(吃、碰、杠)模块的入口文件，负责导出类型和功能函数
// 副露是指玩家通过他人打出的牌或自己摸到的牌形成的面子组合，在麻将中是一种重要的战术元素

mod types;
mod utils;

// 导出所有副露相关的类型和函数，使其可以从外部模块访问
pub use types::{Meld, MeldType, KanType, MeldSource};
pub use utils::{
    validate_chi, validate_pon, validate_kan, 
    sort_tiles_in_meld, can_form_chi, can_form_pon, can_form_kan,
    get_possible_chi_combinations, get_kan_tile_count, get_kan_tiles,
};