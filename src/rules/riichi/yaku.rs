// src/rules/riichi/yaku.rs

use crate::hand::Hand;
use crate::game::context::GameContext;
use crate::rules::Yaku;
use crate::tile::Tile;

/// 计算手牌的所有役种
pub fn calculate_yaku(hand: &Hand, context: &GameContext, win_tile: Tile, is_tsumo: bool) -> Vec<Yaku> {
    let mut yaku_list = Vec::new();

    // --- 示例役种判断 ---

    // 立直 (需要从 context 或 player 状态获取信息)
    // if context.game_state.players[context.current_turn()].is_riichi {
    //     yaku_list.push(Yaku { name: "立直".to_string(), han: 1 });
    // }

    // 门前清自摸和
    if is_tsumo && hand.get_melds().is_empty() {
        yaku_list.push(Yaku { name: "门前清自摸和".to_string(), han: 1 });
    }

    // 役牌：场风
    // if check_yakuhai_round_wind(hand, context) {
    //     yaku_list.push(Yaku { name: format!("役牌: 场风({})", wind_to_char(context.round_wind)), han: 1 });
    // }

    // 役牌：门风
    // if check_yakuhai_player_wind(hand, context) {
    //     yaku_list.push(Yaku { name: format!("役牌: 门风({})", wind_to_char(context.player_wind)), han: 1 });
    // }

    // 役牌：白发中
    // if check_yakuhai_sangen(hand, context, WhiteDragonTile) { yaku_list.push(...) }
    // if check_yakuhai_sangen(hand, context, GreenDragonTile) { yaku_list.push(...) }
    // if check_yakuhai_sangen(hand, context, RedDragonTile) { yaku_list.push(...) }


    // 断幺九
    // if is_tanyao(hand) {
    //     yaku_list.push(Yaku { name: "断幺九".to_string(), han: 1 });
    // }

    // 平和 (需要复杂的条件判断)
    // if is_pinfu(hand, context, win_tile, is_tsumo) {
    //     yaku_list.push(Yaku { name: "平和".to_string(), han: 1 });
    // }

    // 一杯口 / 二杯口 (需要判断)
    // ...

    // 混全带幺九 / 纯全带幺九
    // ...

    // 混一色 / 清一色
    // ...

    // 对对和
    // ...

    // 三暗刻 / 四暗刻
    // ...

    // 小三元 / 大三元
    // ...

    // 宝牌 (Dora) - 通常不计为役，但在算分时增加番数
    // let dora_count = count_dora(hand, context);
    // if dora_count > 0 {
    //     yaku_list.push(Yaku { name: format!("宝牌 {}", dora_count), han: dora_count });
    // }


    // --- 役满 ---
    // 国士无双
    // if crate::rules::riichi::win_check::is_kokushi_musou(hand) { ... }
    // 四暗刻
    // ...
    // 大三元
    // ...
    // 字一色
    // ...
    // 绿一色
    // ...
    // 清老头
    // ...
    // 九莲宝灯
    // ...
    // 四杠子
    // ...
    // 天和 / 地和
    // ...


    // 役种复合和叠加规则处理...

    // 临时返回一个示例役种，保证 can_win 能通过
    if yaku_list.is_empty() {
         // 如果没有其他役，检查断幺九 (简化版)
         if hand.all_tiles().iter().all(|t| !is_terminal_or_honor(*t)) {
              yaku_list.push(Yaku { name: "断幺九 (temp)".to_string(), han: 1 });
         } else {
             // 保证至少有一个役，避免 can_win 失败
             yaku_list.push(Yaku { name: "临时役".to_string(), han: 1 });
         }
    }


    yaku_list
}


// --- Helper functions ---

fn is_terminal_or_honor(tile: Tile) -> bool {
    // 需要 Tile 实现判断是否是幺九牌
    unimplemented!("is_terminal_or_honor needs Tile implementation");
    // let id = tile.id();
    // match id {
    //     0 | 8 | 9 | 17 | 18 | 26 => true, // 1,9 万筒索
    //     27..=33 => true, // 风牌 + 箭牌
    //     _ => false,
    // }
}

// fn wind_to_char(wind: crate::player::model::WindDirection) -> char { ... } // 已在 flow.rs 中定义
// fn check_yakuhai_... ( ... ) -> bool { ... }
// fn is_tanyao( ... ) -> bool { ... }
// fn is_pinfu( ... ) -> bool { ... }
// fn count_dora( ... ) -> u32 { ... }
