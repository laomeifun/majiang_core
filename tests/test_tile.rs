// tests/test_tile.rs

// 告诉 Rust 编译器我们在测试模式下，并且要使用外部的 majiang_core crate
#[cfg(test)]
mod tests {
    // 从 majiang_core crate 的 tile 模块导入我们需要测试的类型
    use majiang_core::tile::{Tile, Suit, Wind, Dragon, Flower};

    // --- 测试牌的创建 ---

    #[test]
    fn test_create_suit_tiles() {
        // 测试创建万字牌
        for i in 1..=9 {
            let tile = Tile::man(i).unwrap();
            assert_eq!(tile.get_suit_and_value(), Some((Suit::Wan, i)));
            assert!(!tile.is_red_dora());
            assert!(tile.is_man());
            assert!(tile.is_suit());
        }
        assert!(Tile::man(0).is_none());
        assert!(Tile::man(10).is_none());

        // 测试创建筒子牌
        for i in 1..=9 {
            let tile = Tile::pin(i).unwrap();
            assert_eq!(tile.get_suit_and_value(), Some((Suit::Tong, i)));
            assert!(!tile.is_red_dora());
            assert!(tile.is_pin());
            assert!(tile.is_suit());
        }
        assert!(Tile::pin(0).is_none());
        assert!(Tile::pin(10).is_none());

        // 测试创建索子牌
        for i in 1..=9 {
            let tile = Tile::sou(i).unwrap();
            assert_eq!(tile.get_suit_and_value(), Some((Suit::Tiao, i)));
            assert!(!tile.is_red_dora());
            assert!(tile.is_sou());
            assert!(tile.is_suit());
        }
        assert!(Tile::sou(0).is_none());
        assert!(Tile::sou(10).is_none());
    }

    #[test]
    fn test_create_honor_tiles() {
        // 测试创建风牌
        let east = Tile::wind(Wind::East);
        assert_eq!(east.get_wind(), Some(Wind::East));
        assert!(east.is_wind());
        assert!(east.is_honor());
        assert!(!east.is_red_dora());

        let south = Tile::wind(Wind::South);
        assert_eq!(south.get_wind(), Some(Wind::South));
        assert!(south.is_wind());

        let west = Tile::wind(Wind::West);
        assert_eq!(west.get_wind(), Some(Wind::West));
        assert!(west.is_wind());

        let north = Tile::wind(Wind::North);
        assert_eq!(north.get_wind(), Some(Wind::North));
        assert!(north.is_wind());

        // 测试创建箭牌
        let white = Tile::dragon(Dragon::White);
        assert_eq!(white.get_dragon(), Some(Dragon::White));
        assert!(white.is_dragon());
        assert!(white.is_honor());
        assert!(!white.is_red_dora());

        let green = Tile::dragon(Dragon::Green);
        assert_eq!(green.get_dragon(), Some(Dragon::Green));
        assert!(green.is_dragon());

        let red = Tile::dragon(Dragon::Red);
        assert_eq!(red.get_dragon(), Some(Dragon::Red));
        assert!(red.is_dragon());
    }

     #[test]
    fn test_create_flower_tiles() {
        let spring = Tile::flower(Flower::Spring);
        assert_eq!(spring.get_flower(), Some(Flower::Spring));
        assert!(spring.is_flower());
        assert!(!spring.is_red_dora());

        let plum = Tile::flower(Flower::Plum);
        assert_eq!(plum.get_flower(), Some(Flower::Plum));
        assert!(plum.is_flower());
    }

    #[test]
    fn test_create_red_dora() {
        let r5m = Tile::red_man5();
        assert_eq!(r5m.get_suit_and_value(), Some((Suit::Wan, 5)));
        assert!(r5m.is_red_dora());
        assert!(r5m.is_man());

        let r5p = Tile::red_pin5();
        assert_eq!(r5p.get_suit_and_value(), Some((Suit::Tong, 5)));
        assert!(r5p.is_red_dora());
        assert!(r5p.is_pin());

        let r5s = Tile::red_sou5();
        assert_eq!(r5s.get_suit_and_value(), Some((Suit::Tiao, 5)));
        assert!(r5s.is_red_dora());
        assert!(r5s.is_sou());

        // 普通 5 不应该是红宝牌
        let m5 = Tile::man(5).unwrap();
        assert!(!m5.is_red_dora());
        let p5 = Tile::pin(5).unwrap();
        assert!(!p5.is_red_dora());
        let s5 = Tile::sou(5).unwrap();
        assert!(!s5.is_red_dora());

        // 非 5 的牌不应该是红宝牌
        let m1 = Tile::man(1).unwrap();
        assert!(!m1.is_red_dora());
        let east = Tile::wind(Wind::East);
        assert!(!east.is_red_dora());
    }

    // --- 测试牌的属性检查 ---

    #[test]
    fn test_is_type_checks() {
        let m1 = Tile::man(1).unwrap();
        let p5 = Tile::pin(5).unwrap();
        let s9 = Tile::sou(9).unwrap();
        let east = Tile::wind(Wind::East);
        let white = Tile::dragon(Dragon::White);
        let spring = Tile::flower(Flower::Spring);
        let r5m = Tile::red_man5();

        // is_suit / is_man / is_pin / is_sou
        assert!(m1.is_suit() && m1.is_man() && !m1.is_pin() && !m1.is_sou());
        assert!(p5.is_suit() && !p5.is_man() && p5.is_pin() && !p5.is_sou());
        assert!(s9.is_suit() && !s9.is_man() && !s9.is_pin() && s9.is_sou());
        assert!(r5m.is_suit() && r5m.is_man());
        assert!(!east.is_suit());
        assert!(!white.is_suit());
        assert!(!spring.is_suit());

        // is_honor / is_wind / is_dragon
        assert!(east.is_honor() && east.is_wind() && !east.is_dragon());
        assert!(white.is_honor() && !white.is_wind() && white.is_dragon());
        assert!(!m1.is_honor());
        assert!(!spring.is_honor());

        // is_flower
        assert!(spring.is_flower());
        assert!(!m1.is_flower());
        assert!(!east.is_flower());
    }

    #[test]
    fn test_is_terminal_simple_honor() {
        let m1 = Tile::man(1).unwrap();
        let m5 = Tile::man(5).unwrap();
        let m9 = Tile::man(9).unwrap();
        let east = Tile::wind(Wind::East);
        let white = Tile::dragon(Dragon::White);
        let r5m = Tile::red_man5();

        // is_terminal (老头牌: 1, 9 数牌)
        assert!(m1.is_terminal());
        assert!(!m5.is_terminal());
        assert!(m9.is_terminal());
        assert!(!east.is_terminal());
        assert!(!white.is_terminal());
        assert!(!r5m.is_terminal());

        // is_simple (中张牌: 2-8 数牌)
        assert!(!m1.is_simple());
        assert!(m5.is_simple());
        assert!(!m9.is_simple());
        assert!(!east.is_simple());
        assert!(!white.is_simple());
        assert!(r5m.is_simple()); // 红5也是中张

        // is_terminal_or_honor (幺九牌: 1, 9 数牌 或 字牌)
        assert!(m1.is_terminal_or_honor());
        assert!(!m5.is_terminal_or_honor());
        assert!(m9.is_terminal_or_honor());
        assert!(east.is_terminal_or_honor());
        assert!(white.is_terminal_or_honor());
        assert!(!r5m.is_terminal_or_honor()); // 红5不是幺九
    }

    // --- 测试牌的 ID 和红宝牌标记 ---
    #[test]
    fn test_id_and_red_dora_flag() {
        let m5 = Tile::man(5).unwrap();
        let r5m = Tile::red_man5();
        let east = Tile::wind(Wind::East);

        assert_eq!(m5.id(), r5m.id()); // 5万和红5万的 id 应该相同
        assert_ne!(m5, r5m); // 但它们不相等，因为红宝牌标记不同

        assert!(!m5.is_red_dora());
        assert!(r5m.is_red_dora());
        assert!(!east.is_red_dora());

        // 确保 id() 返回的是基础 ID
        assert_eq!(m5.id(), 4); // 1m=0, 2m=1, ..., 5m=4
        assert_eq!(r5m.id(), 4);
        assert_eq!(east.id(), 27);
    }

    // --- 测试牌的显示 ---
    #[test]
    fn test_display_and_debug() {
        // Display
        assert_eq!(format!("{}", Tile::man(1).unwrap()), "1m");
        assert_eq!(format!("{}", Tile::pin(9).unwrap()), "9p");
        assert_eq!(format!("{}", Tile::sou(3).unwrap()), "3s");
        assert_eq!(format!("{}", Tile::wind(Wind::East)), "E");
        assert_eq!(format!("{}", Tile::wind(Wind::South)), "S");
        assert_eq!(format!("{}", Tile::wind(Wind::West)), "W");
        assert_eq!(format!("{}", Tile::wind(Wind::North)), "N");
        assert_eq!(format!("{}", Tile::dragon(Dragon::White)), "P"); // 白
        assert_eq!(format!("{}", Tile::dragon(Dragon::Green)), "F"); // 發
        assert_eq!(format!("{}", Tile::dragon(Dragon::Red)), "C");   // 中
        assert_eq!(format!("{}", Tile::flower(Flower::Spring)), "Fl1");
        assert_eq!(format!("{}", Tile::flower(Flower::Plum)), "Fp1");

        // Red Dora Display (uses '0')
        assert_eq!(format!("{}", Tile::red_man5()), "0m");
        assert_eq!(format!("{}", Tile::red_pin5()), "0p");
        assert_eq!(format!("{}", Tile::red_sou5()), "0s");

        // Debug
        assert_eq!(format!("{:?}", Tile::man(1).unwrap()), "1m");
        assert_eq!(format!("{:?}", Tile::red_man5()), "0m(R)");
        assert_eq!(format!("{:?}", Tile::wind(Wind::East)), "E");
        assert_eq!(format!("{:?}", Tile::flower(Flower::Spring)), "Fl1");
    }

    // --- 测试牌的比较和排序 ---
    #[test]
    fn test_ordering() {
        let m1 = Tile::man(1).unwrap();
        let m9 = Tile::man(9).unwrap();
        let p1 = Tile::pin(1).unwrap();
        let s1 = Tile::sou(1).unwrap();
        let east = Tile::wind(Wind::East);
        let north = Tile::wind(Wind::North);
        let white = Tile::dragon(Dragon::White);
        let red = Tile::dragon(Dragon::Red);
        let spring = Tile::flower(Flower::Spring);
        let chrysanthemum = Tile::flower(Flower::Chrysanthemum);
        let m5 = Tile::man(5).unwrap();
        let r5m = Tile::red_man5();

        // 基本顺序 (基于 ID)
        assert!(m1 < m9);
        assert!(m9 < p1);
        assert!(p1 < s1);
        assert!(s1 < east);
        assert!(east < north);
        assert!(north < white);
        assert!(white < red);
        assert!(red < spring);
        assert!(spring < chrysanthemum);

        // 红宝牌与普通牌的比较 (ID 相同，但 is_red_dora 不同)
        // Ord 派生会先比较 id，如果 id 相同，则比较 is_red_dora (false < true)
        use std::cmp::Ordering;
        assert_eq!(m5.cmp(&r5m), Ordering::Less); // m5 (red=false) < r5m (red=true)
        // 但 PartialEq 是不同的
        assert_ne!(m5, r5m); // 它们的值不相等

        // 排序测试
        let mut tiles = vec![p1, m9, east, m1, r5m, white, m5];
        // 预期排序结果 (基于 ID, 然后 is_red_dora): 1m, 5m, 0m(R), 9m, 1p, E, P
        tiles.sort();

        // 验证排序后的顺序
        assert_eq!(tiles[0], m1);  // 1m (id 0)
        assert_eq!(tiles[1], m5);  // 5m (id 4, red false)
        assert_eq!(tiles[2], r5m); // 0m(R) (id 4, red true)
        assert_eq!(tiles[3], m9);  // 9m (id 8)
        assert_eq!(tiles[4], p1);  // 1p (id 9)
        assert_eq!(tiles[5], east); // E (id 27)
        assert_eq!(tiles[6], white); // P (id 31)
    }
}
