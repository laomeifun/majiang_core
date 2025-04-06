// src/rules/shanghai/scoring.rs

use crate::hand::Hand;
use crate::game::context::GameContext;
use crate::rules::{ScoreResult, Yaku};
use crate::tile::Tile;

/// 计算上海麻将的得分 (番数、辣子等)
pub fn calculate_shanghai_score(
    hand: &Hand,
    context: &GameContext,
    win_tile: Tile,
    is_tsumo: bool
) -> Option<ScoreResult> {
    // 上海麻将计分规则多样，通常基于“番”和“辣子”
    // 这里仅作占位符

    let mut total_fan = 0;
    let mut yaku_list = Vec::new(); // 上海麻将可能不叫“役”，但可以类似记录番种

    // --- 示例番种计算 ---

    // 碰碰和
    // if is_pong_pong_hu(hand) { total_fan += 2; yaku_list.push(...) }

    // 清一色
    // if is_qing_yi_se(hand) { total_fan += 4; yaku_list.push(...) }

    // 混一色
    // if is_hun_yi_se(hand) { total_fan += 2; yaku_list.push(...) }

    // 大吊车 (单钓将)
    // if is_single_wait_pair(hand, win_tile) { total_fan += 1; yaku_list.push(...) }

    // 门清 (可能加番)
    // if hand.get_melds().is_empty() { total_fan += 1; yaku_list.push(...) }

    // 自摸 (可能加番)
    // if is_tsumo { total_fan += 1; yaku_list.push(...) }

    // 花牌计算 (需要 flower.rs 逻辑)
    // let flower_fan = super::flower::calculate_flower_fan(hand, context);
    // total_fan += flower_fan;
    // if flower_fan > 0 { yaku_list.push(...) }


    // 基础和牌番 (例如 平和 可能有 1 番底番)
    if total_fan == 0 {
        total_fan = 1; // 假设最少 1 番
        yaku_list.push(Yaku { name: "底和".to_string(), han: 1 });
    }


    // 上海麻将通常不计算符数
    // 得分计算可能基于番数封顶 (例如 8 番或 10 番封顶)
    let final_score = calculate_final_shanghai_points(total_fan, is_tsumo /*, is_dealer */);


    Some(ScoreResult {
        yaku: yaku_list,
        fu: 0, // 上海麻将通常不计符
        score: final_score,
        han: total_fan, // 使用 han 字段存储总番数
    })
}

/// 计算最终支付点数 (简化示例)
fn calculate_final_shanghai_points(total_fan: u32, is_tsumo: bool) -> i32 {
    // 假设 8 番封顶，每番对应一个支付基数
    let capped_fan = total_fan.min(8); // 封顶
    let base_pay = match capped_fan {
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 6,
        5 => 8,
        6 => 10,
        7 => 12,
        8 => 15, // 假设的辣子数或支付单位
        _ => 0, // 不应该发生
    };

    if is_tsumo {
        base_pay * 3 // 自摸三家付
    } else {
        base_pay // 荣和一家付
    }
    // 实际规则可能更复杂，涉及庄家闲家、包牌等
}

// --- Helper functions ---
// fn is_pong_pong_hu(hand: &Hand) -> bool { ... }
// fn is_qing_yi_se(hand: &Hand) -> bool { ... }
// fn is_hun_yi_se(hand: &Hand) -> bool { ... }
// fn is_single_wait_pair(hand: &Hand, win_tile: Tile) -> bool { ... }
