// src/game/turn.rs

use crate::game::state::GameState;
use crate::action::{Action, ActionEvent};
use crate::errors::{MahjongError, MahjongResult};
use crate::player::actions as player_actions; // 引入玩家动作相关函数

/// 处理单个玩家的回合
/// 返回下一个状态或错误
pub fn process_player_turn(game_state: &mut GameState) -> MahjongResult<()> {
    let current_player_index = game_state.current_turn;

    // 1. 玩家摸牌
    let drawn_tile = match game_state.wall.draw_tile() {
        Some(tile) => tile,
        None => {
            // 牌墙摸完，流局处理
            println!("Wall empty, round draw!");
            // handle_round_draw(game_state);
            return Ok(()); // 或者返回特定状态表示流局
        }
    };
    game_state.players[current_player_index].hand.set_drawn_tile(Some(drawn_tile));
    println!("Player {} draws {:?}", current_player_index, drawn_tile);

    // 2. 检查自摸、暗杠等动作
    // let possible_actions_after_draw = get_actions_after_draw(game_state);
    // ...

    // 3. 获取玩家决策 (打牌、立直、自摸、杠)
    let agent = &game_state.agents[current_player_index].clone(); // 克隆 agent 以避免可变借用冲突
    let player = &game_state.players[current_player_index];
    // let context = GameContext::from_game_state(game_state, current_player_index); // 创建上下文

    // 获取所有合法动作 (需要更完善的逻辑)
    let possible_actions = player_actions::get_possible_actions(&player.hand /*, &context */);
    if possible_actions.is_empty() {
         // 理论上至少可以打牌，除非手牌为空？这是异常情况
         return Err(MahjongError::InternalError("No possible actions for player".to_string()));
    }


    let chosen_action = match agent.decide_action(player, &possible_actions /*, &context */) {
        Some(action) => action,
        None => {
            // 如果是 Human/Remote 且没有输入，可能需要等待
            // 这里暂时返回错误或特定状态表示等待输入
            return Err(MahjongError::InvalidAction("Waiting for player input".to_string()));
        }
    };

    println!("Player {} chooses action: {:?}", current_player_index, chosen_action);

    // 4. 应用玩家选择的动作
    apply_action(game_state, current_player_index, chosen_action)?;

    // 5. 如果是打牌动作，触发其他玩家的响应检查 (吃碰杠荣和)
    // if let Action::Discard(discarded_tile) = chosen_action {
    //     process_discard_responses(game_state, current_player_index, discarded_tile)?;
    // }

    // 6. 切换到下一个玩家 (如果游戏没有结束)
    // advance_turn(game_state);

    Ok(())
}

/// 应用玩家选择的动作到游戏状态
fn apply_action(game_state: &mut GameState, player_index: usize, action: Action) -> MahjongResult<()> {
    let player = &mut game_state.players[player_index];

    match action {
        Action::Discard(tile) => {
            player.discard_tile(tile)
                .map_err(|e| MahjongError::InvalidAction(format!("Discard failed: {}", e)))?;
            game_state.last_discard = Some(tile); // 记录最后打出的牌
            // 清空玩家的摸牌状态 (已经在 discard_tile 中处理)
            // player.hand.set_drawn_tile(None);
        }
        Action::Riichi(tile) => {
            // 检查是否满足立直条件 (需要规则)
            // if !can_riichi(...) { return Err(...) }
            player.discard_tile(tile)
                 .map_err(|e| MahjongError::InvalidAction(format!("Riichi discard failed: {}", e)))?;
            player.is_riichi = true;
            game_state.riichi_sticks += 1; // 增加立直棒
            game_state.last_discard = Some(tile);
            // player.hand.set_drawn_tile(None);
            println!("Player {} declares Riichi!", player_index);
        }
        Action::Tsumo => {
            // 处理自摸和牌 (需要规则)
            println!("Player {} wins by Tsumo!", player_index);
            // calculate_score(...);
            // end_round(...);
            unimplemented!("Tsumo handling not implemented");
        }
        Action::Kan(tile) => {
            // 处理杠牌 (需要区分暗杠/加杠/明杠，并从牌山摸牌)
            println!("Player {} declares Kan with {:?}!", player_index, tile);
            // add_kan_meld(...);
            // draw_replacement_tile(...); // 摸岭上牌
            // reveal_new_dora(...); // 翻新的宝牌指示牌
            unimplemented!("Kan handling not implemented");
        }
        // 处理其他动作 (Chi, Pon, Ron, Pass 在响应阶段处理)
        _ => {
            return Err(MahjongError::InvalidAction(format!("Action {:?} cannot be actively chosen in turn", action)));
        }
    }
    Ok(())
}

// --- 其他辅助函数 ---
// fn get_actions_after_draw(...) -> Vec<Action> { ... }
// fn process_discard_responses(...) -> MahjongResult<()> { ... }
// fn advance_turn(...) { ... }
// fn handle_round_draw(...) { ... }
