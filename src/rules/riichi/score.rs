// src/rules/riichi/score.rs

use crate::hand::Hand;
use crate::game::context::GameContext;
use crate::rules::{ScoreResult, Yaku};
use crate::tile::Tile;

/// 计算日麻的符数和最终得分
pub fn calculate_riichi_score(
    hand: &Hand,
    context: &GameContext,
    win_tile: Tile,
    is_tsumo: bool,
    yaku_list: Vec<Yaku> // 传入计算好的役种列表
) -> Option<ScoreResult> {

    // 1. 计算总番数
    let total_han: u32 = yaku_list.iter().map(|y| y.han).sum();
    if total_han == 0 { return None; } // 无役不能和

    // 2. 特殊处理役满
    if is_yakuman(&yaku_list) {
        let score = calculate_yakuman_score(context.game_state.dealer_index == context.current_turn(), is_tsumo, total_han);
        return Some(ScoreResult {
            yaku: yaku_list,
            fu: 0, // 役满不计符数
            score,
            han: total_han, // 可以是复合役满
        });
    }

    // 3. 计算符数 (fu)
    let fu = calculate_fu(hand, context, win_tile, is_tsumo);

    // 4. 根据番数和符数计算基本点 (base_points)
    let base_points = calculate_base_points(total_han, fu);

    // 5. 根据是否庄家、自摸/荣和计算最终支付点数
    let final_score = calculate_final_score(base_points, context.game_state.dealer_index == context.current_turn(), is_tsumo);

    Some(ScoreResult {
        yaku: yaku_list,
        fu,
        score: final_score,
        han: total_han,
    })
}

/// 计算符数 (fu) - 简化版
fn calculate_fu(hand: &Hand, context: &GameContext, win_tile: Tile, is_tsumo: bool) -> u32 {
    let mut fu = 20; // 底符

    // 和牌方式符
    if is_tsumo {
        if !is_pinfu_shape(hand) { // 平和自摸不加符
             fu += 2;
        }
    } else { // 荣和
        if hand.get_melds().is_empty() { // 门清荣和
            fu += 10;
        }
    }

    // 面子符 (刻子/杠子)
    // for meld in hand.get_melds() { fu += fu_for_meld(meld, context); }
    // for pon in find_pons_in_closed_hand(hand) { fu += fu_for_pon(pon, context); }
    // for kan in find_kans_in_closed_hand(hand) { fu += fu_for_kan(kan, context); }

    // 雀头符 (役牌雀头)
    // if is_yakuhai_pair(hand.get_pair(), context) { fu += 2; }

    // 听牌型符 (坎张、边张、单骑)
    // fu += fu_for_wait_pattern(hand, win_tile);

    // 七对子固定 25 符 (特殊规则)
    if crate::rules::riichi::win_check::is_seven_pairs(hand) {
        return 25;
    }

    // 平和型 (Pinfu) 固定 20符(自摸) 或 30符(荣和) (特殊规则)
    // if is_pinfu(...) { return if is_tsumo { 20 } else { 30 }; }


    // 向上取整到 10 的倍数
    (fu + 9) / 10 * 10
}

/// 根据番数和符数计算基本点
fn calculate_base_points(han: u32, fu: u32) -> i32 {
    if han >= 13 { // 役满
        return 8000; // 单倍役满基本点
    }
    if han >= 11 { return 6000; } // 三倍满
    if han >= 8 { return 4000; }  // 倍满
    if han >= 6 { return 3000; }  // 跳满

    let base = (fu as i32) * (1 << (han + 2)); // fu * 2^(han+2)
    if base > 2000 { 2000 } else { base } // 满贯封顶 (基本点不超过 2000)
}

/// 计算最终支付点数
fn calculate_final_score(base_points: i32, is_dealer: bool, is_tsumo: bool) -> i32 {
    if is_tsumo {
        if is_dealer {
            // 庄家自摸：每家支付 base_points * 2 (向上取整到百位)
            (base_points * 2 + 99) / 100 * 100 * 3 // 总支付点数
        } else {
            // 闲家自摸：庄家支付 base_points * 2, 闲家支付 base_points (向上取整到百位)
            let dealer_pay = (base_points * 2 + 99) / 100 * 100;
            let non_dealer_pay = (base_points + 99) / 100 * 100;
            dealer_pay + non_dealer_pay * 2 // 总支付点数
        }
    } else { // 荣和
        if is_dealer {
            // 庄家荣和：放铳者支付 base_points * 6 (向上取整到百位)
            (base_points * 6 + 99) / 100 * 100
        } else {
            // 闲家荣和：放铳者支付 base_points * 4 (向上取整到百位)
            (base_points * 4 + 99) / 100 * 100
        }
    }
    // TODO: 加上本场和立直棒的点数
}

/// 计算役满得分
fn calculate_yakuman_score(is_dealer: bool, is_tsumo: bool, han: u32) -> i32 {
    let multiplier = han / 13; // 几倍役满
    let base_yakuman = if is_dealer { 48000 } else { 32000 };
    base_yakuman * multiplier as i32
    // TODO: 加上本场和立直棒的点数
}

/// 检查是否包含役满役种
fn is_yakuman(yaku_list: &[Yaku]) -> bool {
    // 需要根据役种名称或特定标记判断
    // yaku_list.iter().any(|y| y.is_yakuman_flag)
    false // 暂不实现
}

/// 检查是否是平和型 (简化)
fn is_pinfu_shape(hand: &Hand) -> bool {
    // 4顺子+非役牌雀头+两面听
    false // 暂不实现
}

// 其他符数计算辅助函数...
// fn fu_for_meld(...) -> u32 { ... }
// fn fu_for_pon(...) -> u32 { ... }
// fn fu_for_kan(...) -> u32 { ... }
// fn is_yakuhai_pair(...) -> bool { ... }
// fn fu_for_wait_pattern(...) -> u32 { ... }
