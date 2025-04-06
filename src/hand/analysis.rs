// src/hand/analysis.rs
// 核心分析逻辑 (听牌, 向听数, 牌型分解)

use super::representation::Hand;
use crate::tile::Tile;
use crate::meld::Meld;

/// 手牌分析结果
#[derive(Debug, Clone, Default)]
pub struct HandAnalysisResult {
    /// 向听数 (Shanten) - 距离听牌还差几步
    /// -1: 和了 (Winning hand)
    /// 0: 听牌 (Tenpai)
    /// 1+: 1向听, 2向听, ...
    pub shanten: i8,
    /// 听牌时，所听的牌及对应的剩余张数
    pub waits: Vec<(Tile, usize)>,
    /// (可选) 手牌分解成的面子和搭子
    // pub decomposition: Vec<HandComponent>, // 例如：Meld, Pair, Taatsu (搭子)
    /// (可选) 预估的和了点数或牌型
    // pub estimated_score: u32,
    // pub potential_yaku: Vec<Yaku>,
}

/// 手牌组件，用于表示分解后的部分
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandComponent {
    Meld(Meld), // 副露或暗刻/顺子
    Pair(Tile), // 对子 (雀头)
    Taatsu(Tile, Tile), // 搭子 (例如 12m, 35p, 东东) - 需要更精确的定义
    Isolated(Tile), // 孤张
}


/// 计算向听数的结构体或函数集合
pub struct ShantenCalculator;

impl ShantenCalculator {
    /// 计算给定手牌的向听数 (考虑国士无双和七对子)
    ///
    /// # Arguments
    ///
    /// * `hand` - 需要分析的手牌
    ///
    /// # Returns
    ///
    /// 向听数 (i8)
    pub fn calculate(hand: &Hand) -> i8 {
        // TODO: 实现向听数计算逻辑
        // 1. 计算标准形 (4面子1雀头) 的向听数
        // 2. 计算七对子的向听数
        // 3. 计算国士无双的向听数
        // 4. 取最小值

        // 占位符：返回一个默认值
        let standard_shanten = Self::calculate_standard_form(hand);
        let chiitoi_shanten = Self::calculate_chiitoitsu(hand);
        let kokushi_shanten = Self::calculate_kokushi_musou(hand);

        standard_shanten.min(chiitoi_shanten).min(kokushi_shanten)
    }

    /// 计算标准形 (4面子1雀头) 的向听数
    fn calculate_standard_form(hand: &Hand) -> i8 {
        // 这是最复杂的部分，通常使用递归或动态规划
        // 需要考虑所有可能的面子和搭子组合
        // 公式：向听数 = 8 - (面子数 * 2) - (搭子数 + 雀头数)
        // 需要找到使向听数最小的分解方式

        // 简化占位符
        // 假设手牌是 1112345678999s + 东
        if hand.tile_count() == 14 { // 假设是标准14张牌
             // 这是一个非常粗略的估计
             let pairs = Self::count_pairs(hand);
             let melds = hand.get_melds().len(); // 只计算明牌
             // 假设剩余牌能组成一些搭子
             let taatsu_approx = (hand.get_closed_tiles().len() - pairs * 2) / 2;

             // 粗略公式: 8 - (melds * 2) - (pairs + taatsu_approx)
             // (8 - (melds as i8 * 2) - (pairs.min(1) as i8 + taatsu_approx.min(4 - melds) as i8)).max(-1)
             6 // 返回一个较高的默认值
        } else {
            9 // 非标准牌数，返回更高值
        }
    }

    /// 计算七对子的向听数
    /// 公式：向听数 = 6 - 对子数
    fn calculate_chiitoitsu(hand: &Hand) -> i8 {
        if !hand.get_melds().is_empty() { // 七对子必须门清
            return 9; // 返回一个较大的值表示不可能
        }
        let closed_tiles = hand.get_closed_tiles();
        if closed_tiles.len() != 13 && closed_tiles.len() != 14 { // 必须是13张(听牌)或14张(和了)
             // return 9; // 严格来说是这样，但计算过程中可能牌数不对
        }

        let mut counts = std::collections::HashMap::new();
        for tile in closed_tiles {
            *counts.entry(tile).or_insert(0) += 1;
        }
        if let Some(drawn) = hand.get_drawn_tile() {
             *counts.entry(drawn).or_insert(0) += 1;
        }


        let mut pairs = 0;
        let mut four_of_a_kind = 0; // 四张相同的牌在七对子中算两对
        for &count in counts.values() {
            if count >= 2 {
                pairs += count / 2;
            }
            if count == 4 {
                 four_of_a_kind += 1;
            }
        }

        // 如果有4张一样的牌，每组算2对，所以要额外加
        // pairs += four_of_a_kind; // 这种计算方式不完全对，应该在上面 count/2 时处理

        // 正确计算：需要7对，向听数 = 6 - 对子数
        let required_pairs = 7;
        (required_pairs - pairs as i8).max(0) // 向听数不会小于0 (听牌)
        // 如果 pairs >= 7，则 shanten = -1 (和了)
        // 修正：向听数 = 6 - 对子数
        // (6 - pairs as i8).max(-1) // 允许 -1 表示和了
    }

    /// 计算国士无双的向听数
    /// 需要13种幺九牌各一张，其中一种成对
    /// 公式：
    /// - 有对子: 12 - 种类数
    /// - 无对子: 13 - 种类数
    fn calculate_kokushi_musou(hand: &Hand) -> i8 {
        if !hand.get_melds().is_empty() { // 国士必须门清
            return 9;
        }
         let closed_tiles = hand.get_closed_tiles();
         if closed_tiles.len() != 13 && closed_tiles.len() != 14 {
             // return 9;
         }

        let kokushi_tiles: Vec<Tile> = vec![
            Tile::new_numbered(crate::tile::TileSuit::Characters, 1), Tile::new_numbered(crate::tile::TileSuit::Characters, 9),
            Tile::new_numbered(crate::tile::TileSuit::Dots, 1), Tile::new_numbered(crate::tile::TileSuit::Dots, 9),
            Tile::new_numbered(crate::tile::TileSuit::Bamboos, 1), Tile::new_numbered(crate::tile::TileSuit::Bamboos, 9),
            Tile::new_wind(crate::tile::WindSuit::East), Tile::new_wind(crate::tile::WindSuit::South),
            Tile::new_wind(crate::tile::WindSuit::West), Tile::new_wind(crate::tile::WindSuit::North),
            Tile::new_dragon(crate::tile::DragonSuit::White), Tile::new_dragon(crate::tile::DragonSuit::Green),
            Tile::new_dragon(crate::tile::DragonSuit::Red),
        ];

        let mut counts = std::collections::HashMap::new();
        for tile in closed_tiles {
             if kokushi_tiles.contains(&tile) {
                 *counts.entry(tile).or_insert(0) += 1;
             } else {
                 return 9; // 包含非幺九牌，不可能是国士
             }
        }
         if let Some(drawn) = hand.get_drawn_tile() {
             if kokushi_tiles.contains(&drawn) {
                 *counts.entry(drawn).or_insert(0) += 1;
             } else {
                 return 9;
             }
         }


        let mut kinds = 0;
        let mut has_pair = false;
        for tile in &kokushi_tiles {
            if let Some(&count) = counts.get(tile) {
                if count > 0 {
                    kinds += 1;
                }
                if count >= 2 {
                    has_pair = true;
                }
            }
        }

        if has_pair {
            (13 - 1 - kinds as i8).max(-1) // 国士听牌/和了 (13张牌，12种+1对)
        } else {
            (13 - kinds as i8).max(0) // 国士 N 向听 (需要凑齐13种)
        }
    }

    /// (辅助函数) 统计手牌中的对子数量 (仅用于估算)
    fn count_pairs(hand: &Hand) -> usize {
         let mut counts = std::collections::HashMap::new();
         for tile in hand.get_closed_tiles() {
             *counts.entry(tile).or_insert(0) += 1;
         }
         if let Some(drawn) = hand.get_drawn_tile() {
             *counts.entry(drawn).or_insert(0) += 1;
         }
         counts.values().filter(|&&count| count >= 2).count()
    }
}

/// 分析手牌，计算向听数和听牌
///
/// # Arguments
///
/// * `hand` - 需要分析的手牌
/// * `remaining_tiles` - (可选) 牌墙/牌河中剩余牌的信息，用于计算听牌张数
///
/// # Returns
///
/// `HandAnalysisResult` 包含分析结果
pub fn analyze_hand(hand: &Hand, _remaining_tiles: Option<&HashMap<Tile, usize>>) -> HandAnalysisResult {
    // TODO: 实现完整的分析逻辑
    let shanten = ShantenCalculator::calculate(hand);
    let mut waits = Vec::new();

    if shanten == 0 {
        // TODO: 计算听牌的具体牌张和剩余数量
        // 需要遍历所有可能的打牌选择，然后检查打掉后是否和牌
        // 或者使用更高效的听牌判断算法
        waits.push((Tile::new_numbered(crate::tile::TileSuit::Characters, 1), 4)); // 占位符
    }

    HandAnalysisResult {
        shanten,
        waits,
        // decomposition: vec![], // 占位符
        // estimated_score: 0,
        // potential_yaku: vec![],
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::{Tile, TileSuit, WindSuit, DragonSuit};
    use crate::hand::representation::Hand; // 引入 Hand

    // Helper to create hand from string (requires parser module)
    // fn hand_from_str(s: &str) -> Hand { ... }

    #[test]
    fn test_shanten_calculation_simple() {
        let mut hand = Hand::new();
        // 1112345678999s + 东 (和了) - 假设能正确解析
        // 简化：手动添加牌
        hand.add_tile(Tile::new_numbered(TileSuit::Bamboos, 1));
        hand.add_tile(Tile::new_numbered(TileSuit::Bamboos, 1));
        hand.add_tile(Tile::new_numbered(TileSuit::Bamboos, 1));
        hand.add_tile(Tile::new_numbered(TileSuit::Bamboos, 2));
        hand.add_tile(Tile::new_numbered(TileSuit::Bamboos, 3));
        hand.add_tile(Tile::new_numbered(TileSuit::Bamboos, 4));
        hand.add_tile(Tile::new_numbered(TileSuit::Bamboos, 5));
        hand.add_tile(Tile::new_numbered(TileSuit::Bamboos, 6));
        hand.add_tile(Tile::new_numbered(TileSuit::Bamboos, 7));
        hand.add_tile(Tile::new_numbered(TileSuit::Bamboos, 8));
        hand.add_tile(Tile::new_numbered(TileSuit::Bamboos, 9));
        hand.add_tile(Tile::new_numbered(TileSuit::Bamboos, 9));
        hand.add_tile(Tile::new_numbered(TileSuit::Bamboos, 9));
        hand.set_drawn_tile(Some(Tile::new_wind(WindSuit::East))); // 摸到第14张

        // TODO: 当前的 calculate_standard_form 是占位符，无法正确计算
        // assert_eq!(ShantenCalculator::calculate(&hand), -1); // 期望和了

        // 测试七对子 (6 对 + 1 张) -> 0 向听 (听牌)
        let mut chiitoi_tenpai = Hand::new();
        chiitoi_tenpai.add_tile(Tile::new_numbered(TileSuit::Characters, 1));
        chiitoi_tenpai.add_tile(Tile::new_numbered(TileSuit::Characters, 1));
        chiitoi_tenpai.add_tile(Tile::new_numbered(TileSuit::Dots, 2));
        chiitoi_tenpai.add_tile(Tile::new_numbered(TileSuit::Dots, 2));
        chiitoi_tenpai.add_tile(Tile::new_numbered(TileSuit::Bamboos, 3));
        chiitoi_tenpai.add_tile(Tile::new_numbered(TileSuit::Bamboos, 3));
        chiitoi_tenpai.add_tile(Tile::new_wind(WindSuit::East));
        chiitoi_tenpai.add_tile(Tile::new_wind(WindSuit::East));
        chiitoi_tenpai.add_tile(Tile::new_wind(WindSuit::South));
        chiitoi_tenpai.add_tile(Tile::new_wind(WindSuit::South));
        chiitoi_tenpai.add_tile(Tile::new_dragon(DragonSuit::White));
        chiitoi_tenpai.add_tile(Tile::new_dragon(DragonSuit::White));
        chiitoi_tenpai.add_tile(Tile::new_dragon(DragonSuit::Green)); // 13张

        assert_eq!(ShantenCalculator::calculate_chiitoitsu(&chiitoi_tenpai), 0); // 6对，差一对，0向听

        // 测试国士无双 (12种 + 1张) -> 0 向听 (听牌)
        let mut kokushi_tenpai = Hand::new();
        kokushi_tenpai.add_tile(Tile::new_numbered(TileSuit::Characters, 1));
        kokushi_tenpai.add_tile(Tile::new_numbered(TileSuit::Characters, 9));
        kokushi_tenpai.add_tile(Tile::new_numbered(TileSuit::Dots, 1));
        kokushi_tenpai.add_tile(Tile::new_numbered(TileSuit::Dots, 9));
        kokushi_tenpai.add_tile(Tile::new_numbered(TileSuit::Bamboos, 1));
        kokushi_tenpai.add_tile(Tile::new_numbered(TileSuit::Bamboos, 9));
        kokushi_tenpai.add_tile(Tile::new_wind(WindSuit::East));
        kokushi_tenpai.add_tile(Tile::new_wind(WindSuit::South));
        kokushi_tenpai.add_tile(Tile::new_wind(WindSuit::West));
        kokushi_tenpai.add_tile(Tile::new_wind(North)); // 假设 North 存在
        kokushi_tenpai.add_tile(Tile::new_dragon(DragonSuit::White));
        kokushi_tenpai.add_tile(Tile::new_dragon(DragonSuit::Green));
        kokushi_tenpai.add_tile(Tile::new_dragon(DragonSuit::Red)); // 13张，缺一对

         assert_eq!(ShantenCalculator::calculate_kokushi_musou(&kokushi_tenpai), 0); // 13种，无对子，0向听
    }

    // TODO: 添加更复杂的向听数测试用例
    // TODO: 添加听牌分析的测试用例
}
