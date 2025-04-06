// src/game/utils.rs

// 可以在这里放置游戏相关的辅助函数
// 例如：计算玩家索引的下一个、判断是否庄家等

/// 获取下一个玩家的索引 (循环)
pub fn next_player_index(current_index: usize) -> usize {
    (current_index + 1) % 4
}

/// 获取上一个玩家的索引 (循环)
pub fn prev_player_index(current_index: usize) -> usize {
    (current_index + 3) % 4 // (current_index - 1 + 4) % 4
}

// 示例函数
pub fn example_game_util() -> bool {
    true
}
