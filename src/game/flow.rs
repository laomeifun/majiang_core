// src/game/flow.rs

use crate::game::state::GameState;
use crate::game::turn::process_player_turn;
use crate::errors::MahjongResult;

/// 运行整个游戏直到结束
pub fn run_game(game_state: &mut GameState) -> MahjongResult<()> {
    println!("Starting game!");

    // 游戏主循环 (可以按局、按轮进行)
    loop {
        // 使用 Display trait 直接打印风向
        println!("\n--- Round: {}{}局 {}本场 ---",
                 game_state.round_wind, // 直接使用 Display
                 game_state.round_number,
                 game_state.honba);
        println!("Dealer: Player {}", game_state.dealer_index);
        println!("Riichi sticks: {}", game_state.riichi_sticks);

        // 初始化一局 (洗牌、发牌、设置宝牌等)
        initialize_round(game_state)?;

        // 回合循环
        loop {
            // 处理当前玩家的回合
            let turn_result = process_player_turn(game_state);

            match turn_result {
                Ok(_) => {
                    // 检查是否有人和牌或流局，如果结束则跳出内层循环
                    // if is_round_over(game_state) { break; }
                    // 否则，切换到下一个玩家 (需要实现 advance_turn)
                    // advance_turn(game_state);
                    println!("Turn processed, advancing (logic pending)...");
                    // 临时跳出，避免无限循环
                     break; // TODO: Remove this break when round end logic is added
                }
                Err(e) => {
                    // 处理错误，例如等待玩家输入或游戏逻辑错误
                    eprintln!("Error during turn: {:?}", e);
                    // 根据错误类型决定是否继续或终止游戏
                    return Err(e); // 暂时直接返回错误
                }
            }
             // 临时跳出外层循环
             break; // TODO: Remove this break when game end logic is added
        }

        // 处理一局结束 (计分、连庄判断等)
        // process_round_end(game_state)?;

        // 检查整个游戏是否结束 (例如有人被飞)
        // if is_game_over(game_state) {
        //     println!("Game over!");
        //     break;
        // }

        // 准备下一局 (切换庄家、场风等)
        // prepare_next_round(game_state)?;

         // 临时跳出外层循环
         break; // TODO: Remove this break when game end logic is added
    }

    // 游戏结束后的处理 (最终排名等)
    // finalize_game(game_state);

    Ok(())
}

/// 初始化一局 (洗牌、发牌等) - 示例
fn initialize_round(game_state: &mut GameState) -> MahjongResult<()> {
    println!("Initializing round...");
    game_state.wall = crate::wall::Wall::new(); // 创建新牌墙并洗牌
    game_state.current_turn = game_state.dealer_index; // 庄家先开始
    game_state.last_discard = None;
    // 清空玩家手牌、弃牌堆、立直状态等
    for player in &mut game_state.players {
        player.hand = crate::hand::Hand::new();
        player.discards.clear();
        player.is_riichi = false;
    }

    // 发牌 (每人 13 张)
    println!("Dealing tiles...");
    for _ in 0..13 {
        for i in 0..4 {
            if let Some(tile) = game_state.wall.draw_tile() {
                game_state.players[i].hand.add_closed_tile(tile);
            } else {
                return Err(crate::errors::MahjongError::InternalError("Wall empty during initial deal".to_string()));
            }
        } // <-- 这里是内层循环的结束
    } // <-- 添加外层循环的结束括号 '}'
    // 翻开宝牌指示牌 (需要从牌山特定位置拿)
    // game_state.dora_indicators = reveal_dora_indicators(&mut game_state.wall)?;
    println!("Round initialized.");
    Ok(()) // <-- 确保 Ok(()) 存在
}

// --- 辅助函数 ---
// wind_to_char 函数不再需要，因为 WindDirection 实现了 Display
// fn wind_to_char(wind: crate::player::model::WindDirection) -> char { ... }

// fn is_round_over(game_state: &GameState) -> bool { ... }
// fn advance_turn(game_state: &mut GameState) { ... }
// fn process_round_end(game_state: &mut GameState) -> MahjongResult<()> { ... }
// fn is_game_over(game_state: &GameState) -> bool { ... }
// fn prepare_next_round(game_state: &mut GameState) -> MahjongResult<()> { ... }
// fn finalize_game(game_state: &GameState) { ... }
// fn reveal_dora_indicators(wall: &mut crate::wall::Wall) -> MahjongResult<Vec<crate::tile::Tile>> { ... }
