// src/lib.rs

// 导出基础模块
pub mod tile;
pub mod meld;
pub mod hand;
pub mod wall;
pub mod action;
pub mod errors;

// 导出玩家模块
pub mod player;

// 导出游戏逻辑模块
pub mod game;

// 导出规则模块
pub mod rules;

// 如果需要，可以在这里添加一些顶层函数或常量

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
