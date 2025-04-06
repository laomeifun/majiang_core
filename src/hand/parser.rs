// src/hand/parser.rs
// 手牌字符串解析

use super::representation::Hand;
use crate::tile::Tile;
use crate::meld::{Meld, MeldType, KanType}; // 假设 Meld 定义在 crate::meld
use regex::Regex; // 可能需要正则表达式库

/// 解析手牌字符串，生成 Hand 对象
///
/// 支持的格式示例:
/// - "123m456p789sEESS" (暗牌)
/// - "111m P:222p K:333s C:456m D:7z" (暗牌 + 副露 + 摸牌)
///   - P: Pon, K: Kan (Open/Added), C: Chi, D: Drawn
///   - 暗杠可以用 Kk: 表示 (Closed Kan)
///   - 加杠可以用 Ka: 表示 (Added Kan)
///
/// # Arguments
///
/// * `s` - 要解析的手牌字符串
///
/// # Returns
///
/// Result<Hand, String> - 成功则返回 Hand 对象，失败则返回错误信息
pub fn parse_hand_string(s: &str) -> Result<Hand, String> {
    // TODO: 实现更健壮的解析逻辑
    let mut hand = Hand::new();
    let mut current_part = String::new();
    let mut parsing_melds = false;
    let mut drawn_tile_str: Option<String> = None;

    for c in s.chars() {
        if c.is_whitespace() { // 跳过空格
            continue;
        }
        if c.is_alphabetic() && c.is_uppercase() && current_part.is_empty() {
            // 开始解析副露或摸牌部分 (例如 P:, K:, C:, D:)
            parsing_melds = true;
            current_part.push(c);
        } else if parsing_melds && c == ':' {
            // 解析副露/摸牌类型
            match current_part.as_str() {
                "P" | "PON" => { /* 准备解析碰 */ }
                "K" | "KAN" | "Kk" | "Ka" => { /* 准备解析杠 */ }
                "C" | "CHI" => { /* 准备解析吃 */ }
                "D" | "DRAWN" => { /* 准备解析摸牌 */ }
                _ => return Err(format!("Unknown meld/drawn type: {}", current_part)),
            }
            // 清空 current_part 以便接收牌数据
            current_part.clear();
        } else if parsing_melds {
            // 收集副露/摸牌的牌数据
            current_part.push(c);
            // TODO: 在遇到下一个大写字母或字符串末尾时处理 current_part
            //      并根据类型 (P/K/C/D) 创建 Meld 或设置 drawn_tile
            //      需要调用 parse_tile_string 等辅助函数
        } else {
            // 解析暗牌部分
            current_part.push(c);
            if c.is_alphabetic() && c != 'z' { // 假设牌字符串以 m, p, s, z 结尾
                // 解析一组牌，例如 "123m"
                match parse_tiles_from_part(&current_part) {
                    Ok(tiles) => {
                        for tile in tiles {
                            hand.add_tile(tile);
                        }
                    }
                    Err(e) => return Err(e),
                }
                current_part.clear();
            }
        }
    }

    // 处理字符串末尾可能剩余的暗牌部分
    if !parsing_melds && !current_part.is_empty() {
         return Err(format!("Invalid trailing characters in hand string: {}", current_part));
         // 或者尝试解析最后一部分？
         // match parse_tiles_from_part(&current_part) { ... }
    }

    // TODO: 处理解析出的副露和摸牌字符串

    Ok(hand)
}

/// 从字符串片段解析牌列表 (例如 "123m", "EEz")
fn parse_tiles_from_part(part: &str) -> Result<Vec<Tile>, String> {
    // TODO: 实现从 "123m" 或 "EEz" 这样的字符串解析 Tile 列表
    // 需要处理数字、花色字符 (m, p, s) 和字牌字符 (E, S, W, N, Z, B, F)
    // 简单的占位符实现
    let mut tiles = Vec::new();
    let re_tile = Regex::new(r"(\d+)([mpsz])|([ESWNPBFZ]+)z").unwrap(); // 简化正则
    if let Some(caps) = re_tile.captures(part) {
        if let Some(nums) = caps.get(1) {
            let suit_char = caps.get(2).unwrap().as_str().chars().next().unwrap();
            let suit = crate::tile::parse_suit(suit_char)?;
            for num_char in nums.as_str().chars() {
                if let Some(num) = num_char.to_digit(10) {
                     if num >= 1 && num <= 9 {
                         tiles.push(Tile::new_numbered(suit, num as u8));
                     } else {
                         return Err(format!("Invalid number in part: {}", part));
                     }
                } else {
                     return Err(format!("Invalid character in number part: {}", part));
                }
            }
        } else if let Some(honors) = caps.get(3) {
            for honor_char in honors.as_str().chars() {
                 tiles.push(crate::tile::parse_honor(honor_char)?);
            }
        } else {
             return Err(format!("Invalid tile part format: {}", part));
        }
    } else {
         return Err(format!("Could not parse tile part: {}", part));
    }

    Ok(tiles)
}

// TODO: 添加 parse_tile_string, parse_meld_string 等辅助函数

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::{Tile, TileSuit, WindSuit};

    #[test]
    fn test_parse_simple_hand() {
        // TODO: 完善 parse_tiles_from_part 后启用
        // let result = parse_hand_string("123m45p789sEEz");
        // assert!(result.is_ok());
        // let hand = result.unwrap();
        // assert_eq!(hand.get_closed_tiles().len(), 11);
        // assert!(hand.get_melds().is_empty());
        // assert!(hand.get_drawn_tile().is_none());
        // // 检查具体牌张
        // assert!(hand.get_closed_tiles().contains(&Tile::new_numbered(TileSuit::Characters, 1)));
        // assert!(hand.get_closed_tiles().contains(&Tile::new_wind(WindSuit::East)));
    }

    #[test]
    fn test_parse_with_meld_and_drawn() {
        // TODO: 实现完整的解析逻辑后启用
        // let result = parse_hand_string("11m P:222p D:3s");
        // assert!(result.is_ok());
        // let hand = result.unwrap();
        // assert_eq!(hand.get_closed_tiles().len(), 2); // 11m
        // assert_eq!(hand.get_melds().len(), 1); // P:222p
        // assert_eq!(hand.get_drawn_tile(), Some(Tile::new_numbered(TileSuit::Bamboos, 3)));
        // assert_eq!(hand.get_melds()[0].meld_type(), &MeldType::Pon);
    }

    #[test]
    fn test_parse_invalid_string() {
        assert!(parse_hand_string("123m45pXE").is_err()); // 无效字符
        assert!(parse_hand_string("123m P:22p").is_err()); // 副露格式错误
        assert!(parse_hand_string("123m D:").is_err()); // 摸牌缺少数据
    }
}
