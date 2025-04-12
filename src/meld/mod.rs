// src/meld/mod.rs
//
// 副露(吃、碰、杠)模块的入口文件，负责导出类型和功能函数
// 副露是指玩家通过他人打出的牌或自己摸到的牌形成的面子组合，在麻将中是一种重要的战术元素

mod types;
pub mod utils;

// 导出所有副露相关的类型和函数，使其可以从外部模块访问
pub use types::{Meld, MeldType, KanType, MeldSource};
pub use utils::{
    validate_chi, validate_pon, validate_kan, 
    sort_tiles_in_meld, can_form_chi, can_form_pon, can_form_kan,
    get_possible_chi_combinations, get_kan_tile_count, get_kan_tiles,
};

/// 麻将副露（吃、碰、杠）模块
/// 
/// 本模块提供了与麻将游戏中副露相关的所有功能，包括副露的创建、验证和操作。
/// 副露是指玩家通过自己或他人的牌形成并亮出的面子组合，是麻将战术的重要组成部分。
/// 
/// # 副露类型
/// 
/// 麻将中有三种基本的副露类型：
/// 
/// - **吃（Chi）**: 使用自己手牌中的两张牌与他人打出的一张牌，组成顺子。
///   只能吃上家打出的牌，且只能由数牌组成。
/// 
/// - **碰（Pon）**: 使用自己手牌中的两张相同牌与他人打出的一张相同的牌，
///   组成刻子（三张相同的牌）。可以碰任何玩家打出的牌。
/// 
/// - **杠（Kan）**: 由四张相同的牌组成，分为三种类型：
///   - **明杠（Open）**: 使用自己手牌中的三张相同牌与他人打出的一张相同的牌组成
///   - **暗杠（Closed）**: 使用自己手牌中的四张相同牌组成，不公开具体牌面
///   - **加杠（Added）**: 在已有碰的基础上，使用自己摸到的第四张相同牌加杠
/// 
/// # 主要功能
/// 
/// - 副露的创建和验证
/// - 判断手牌是否可以形成特定的副露
/// - 获取可能的吃牌组合
/// - 副露牌的排序和处理
/// 
/// # 使用示例
/// 
/// 创建一个吃副露：
/// 
/// ```
/// use majiang_core::tile::{Tile, Suit};
/// use majiang_core::meld::{Meld, MeldType, MeldSource};
/// 
/// // 创建吃的牌组：2筒、3筒、4筒
/// let tiles = vec![
///     Tile::new_suit(Suit::Dot, 2).unwrap(),
///     Tile::new_suit(Suit::Dot, 3).unwrap(),
///     Tile::new_suit(Suit::Dot, 4).unwrap(),
/// ];
/// 
/// // 假设3筒是从玩家1处吃来的
/// let sources = vec![
///     MeldSource::SelfDrawn,
///     MeldSource::Player(1),
///     MeldSource::SelfDrawn,
/// ];
/// 
/// // 创建吃副露
/// let chi = Meld::new(tiles, MeldType::Chi, sources).unwrap();
/// ```
/// 
/// 判断是否可以吃某张牌：
/// 
/// ```
/// use majiang_core::tile::{Tile, Suit};
/// use majiang_core::meld::can_form_chi;
/// 
/// // 手牌
/// let hand = vec![
///     Tile::new_suit(Suit::Character, 1).unwrap(),
///     Tile::new_suit(Suit::Character, 2).unwrap(),
///     // ...其他牌
/// ];
/// 
/// // 判断是否可以吃"3万"
/// let tile = Tile::new_suit(Suit::Character, 3).unwrap();
/// if can_form_chi(&hand, tile) {
///     println!("可以吃3万");
/// }
/// ```
///
/// # 注意事项
///
/// - 所有副露验证函数遵循严格的麻将规则检查
/// - 吃、碰、杠的规则可能因不同的麻将变种（如日本麻将、中国麻将）而略有差异
/// - 副露操作可能影响玩家的和牌方式和得分
pub mod doc {}