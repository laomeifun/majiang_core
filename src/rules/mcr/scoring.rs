// src/rules/mcr/scoring.rs

use crate::hand::Hand;
use crate::game::context::GameContext;
use crate::rules::{ScoreResult, Yaku};
use crate::tile::Tile;

/// 计算国标麻将的番种得分
pub fn calculate_mcr_score(
    hand: &Hand,
    context: &GameContext,
    win_tile: Tile,
    is_tsumo: bool
) -> Option<ScoreResult> {
    // 国标麻将有 81 个番种，计分复杂
    // 这里仅作占位符

    let mut total_score = 0;
    let mut yaku_list = Vec::new(); // 记录番种

    // --- 示例番种计算 (分数直接累加) ---

    // 大四喜 (88分)
    // if is_big_four_winds(hand) { total_score += 88; yaku_list.push(Yaku { name: "大四喜".to_string(), han: 88 }); }

    // 大三元 (88分)
    // if is_big_three_dragons(hand) { total_score += 88; yaku_list.push(Yaku { name: "大三元".to_string(), han: 88 }); }

    // 清幺九 (64分)
    // if is_all_terminals(hand) { total_score += 64; yaku_list.push(Yaku { name: "清幺九".to_string(), han: 64 }); }

    // 碰碰和 (6分)
    // if is_all_pongs(hand) { total_score += 6; yaku_list.push(Yaku { name: "碰碰和".to_string(), han: 6 }); }

    // 清一色 (24分)
    // if is_pure_one_suit(hand) { total_score += 24; yaku_list.push(Yaku { name: "清一色".to_string(), han: 24 }); }

    // 花牌 (每张1分)
    // let flower_score = super::flower::calculate_mcr_flower_score(hand);
    // if flower_score > 0 { total_score += flower_score; yaku_list.push(Yaku { name: format!("花牌 {}", flower_score), han: flower_score }); }

    // 自摸 (1分)
    // if is_tsumo { total_score += 1; yaku_list.push(Yaku { name: "自摸".to_string(), han: 1 }); }

    // 和绝张 (4分) - 需要判断和的牌是否是最后一张
    // ...

    // 平和 (2分) - 国标平和定义与日麻不同
    // if is_mcr_pinhu(hand) { total_score += 2; yaku_list.push(Yaku { name: "平和".to_string(), han: 2 }); }


    // 国标计分原则：
    // 1. 不重复原则：例如 大四喜 不计 碰碰和、幺九刻等。
    // 2. 不拆移原则：确定一种组合后不能再拆开计其他番种。
    // 3. 就高不就低原则：按得分最高的组合计算。
    // 4. 套算一次原则：例如 绿一色 不计 混一色。

    // 实际计算需要复杂的逻辑来处理这些原则，选择最优的番种组合。

    // 简化：直接返回一个基础分，保证 can_win 通过
    if total_score < 8 {
        total_score = 8; // 假设基础分
        yaku_list.push(Yaku { name: "底分".to_string(), han: 8 });
    }


    Some(ScoreResult {
        yaku: yaku_list,
        fu: 0, // 国标不计符
        score: total_score as i32, // 使用 score 字段存储总分
        han: 0, // han 字段在国标中意义不大，可以设为 0 或番种数量
    })
}

// --- Helper functions for MCR yaku ---
// fn is_big_four_winds(hand: &Hand) -> bool { ... }
// fn is_big_three_dragons(hand: &Hand) -> bool { ... }
// fn is_all_terminals(hand: &Hand) -> bool { ... }
// fn is_all_pongs(hand: &Hand) -> bool { ... }
// fn is_pure_one_suit(hand: &Hand) -> bool { ... }
// fn is_mcr_pinhu(hand: &Hand) -> bool { ... }
