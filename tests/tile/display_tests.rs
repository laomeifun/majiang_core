// tests/tile/display_tests.rs
//
// 为display.rs模块提供单元测试

use majiang_core::tile::{
    Tile, Suit, Wind, Dragon, Flower,
    DisplayStyle, TileDisplay, ColorStyle, ColoredTileDisplay, TileGrid,
};

#[test]
fn test_default_display() {
    // 测试数牌的默认显示
    let man_tile = Tile::Suit(Suit::Character, 5);
    let pin_tile = Tile::Suit(Suit::Dot, 7);
    let sou_tile = Tile::Suit(Suit::Bamboo, 3);
    
    assert_eq!(man_tile.display(DisplayStyle::Default), "5万");
    assert_eq!(pin_tile.display(DisplayStyle::Default), "7筒");
    assert_eq!(sou_tile.display(DisplayStyle::Default), "3条");
    
    // 测试风牌的默认显示
    let east = Tile::Wind(Wind::East);
    let west = Tile::Wind(Wind::West);
    
    assert_eq!(east.display(DisplayStyle::Default), "东");
    assert_eq!(west.display(DisplayStyle::Default), "西");
    
    // 测试三元牌的默认显示
    let white = Tile::Dragon(Dragon::White);
    let red = Tile::Dragon(Dragon::Red);
    
    assert_eq!(white.display(DisplayStyle::Default), "白");
    assert_eq!(red.display(DisplayStyle::Default), "中");
    
    // 测试花牌的默认显示
    let spring = Tile::Flower(Flower::Spring);
    let plum = Tile::Flower(Flower::Plum);
    
    assert_eq!(spring.display(DisplayStyle::Default), "春");
    assert_eq!(plum.display(DisplayStyle::Default), "梅");
    
    // 测试百搭牌的默认显示
    let joker = Tile::Joker;
    assert_eq!(joker.display(DisplayStyle::Default), "百搭");
}

#[test]
fn test_compact_display() {
    // 测试数牌的简洁显示
    let man_tile = Tile::Suit(Suit::Character, 5);
    let pin_tile = Tile::Suit(Suit::Dot, 7);
    let sou_tile = Tile::Suit(Suit::Bamboo, 3);
    
    assert_eq!(man_tile.display(DisplayStyle::Compact), "5m");
    assert_eq!(pin_tile.display(DisplayStyle::Compact), "7p");
    assert_eq!(sou_tile.display(DisplayStyle::Compact), "3s");
    
    // 测试风牌的简洁显示
    let east = Tile::Wind(Wind::East);
    let west = Tile::Wind(Wind::West);
    
    assert_eq!(east.display(DisplayStyle::Compact), "E");
    assert_eq!(west.display(DisplayStyle::Compact), "W");
    
    // 测试三元牌的简洁显示
    let white = Tile::Dragon(Dragon::White);
    let red = Tile::Dragon(Dragon::Red);
    
    assert_eq!(white.display(DisplayStyle::Compact), "W");
    assert_eq!(red.display(DisplayStyle::Compact), "R");
    
    // 测试花牌的简洁显示
    let spring = Tile::Flower(Flower::Spring);
    let plum = Tile::Flower(Flower::Plum);
    
    assert_eq!(spring.display(DisplayStyle::Compact), "F1");
    assert_eq!(plum.display(DisplayStyle::Compact), "F5");
    
    // 测试百搭牌的简洁显示
    let joker = Tile::Joker;
    assert_eq!(joker.display(DisplayStyle::Compact), "J");
}

#[test]
fn test_unicode_display() {
    // 测试数牌的Unicode显示
    let man1 = Tile::Suit(Suit::Character, 1);
    let pin5 = Tile::Suit(Suit::Dot, 5);
    let sou9 = Tile::Suit(Suit::Bamboo, 9);
    
    assert_eq!(man1.display(DisplayStyle::Unicode), "🀇");
    assert_eq!(pin5.display(DisplayStyle::Unicode), "🀝");
    assert_eq!(sou9.display(DisplayStyle::Unicode), "🀘");
    
    // 测试风牌的Unicode显示
    let east = Tile::Wind(Wind::East);
    let south = Tile::Wind(Wind::South);
    
    assert_eq!(east.display(DisplayStyle::Unicode), "🀀");
    assert_eq!(south.display(DisplayStyle::Unicode), "🀁");
    
    // 测试三元牌的Unicode显示
    let white = Tile::Dragon(Dragon::White);
    let red = Tile::Dragon(Dragon::Red);
    
    assert_eq!(white.display(DisplayStyle::Unicode), "🀆");
    assert_eq!(red.display(DisplayStyle::Unicode), "🀄");
    
    // 测试花牌的Unicode显示
    let flower = Tile::Flower(Flower::Spring);
    assert_eq!(flower.display(DisplayStyle::Unicode), "🎴");
    
    // 测试百搭牌的Unicode显示
    let joker = Tile::Joker;
    assert_eq!(joker.display(DisplayStyle::Unicode), "🃟");
}

#[test]
fn test_ascii_display() {
    // 测试数牌的ASCII显示
    let man_tile = Tile::Suit(Suit::Character, 5);
    let pin_tile = Tile::Suit(Suit::Dot, 7);
    let sou_tile = Tile::Suit(Suit::Bamboo, 3);
    
    assert_eq!(man_tile.display(DisplayStyle::Ascii), "|5m|");
    assert_eq!(pin_tile.display(DisplayStyle::Ascii), "|7p|");
    assert_eq!(sou_tile.display(DisplayStyle::Ascii), "|3s|");
    
    // 测试风牌的ASCII显示
    let east = Tile::Wind(Wind::East);
    let north = Tile::Wind(Wind::North);
    
    assert_eq!(east.display(DisplayStyle::Ascii), "|E|");
    assert_eq!(north.display(DisplayStyle::Ascii), "|N|");
    
    // 测试三元牌的ASCII显示
    let white = Tile::Dragon(Dragon::White);
    let green = Tile::Dragon(Dragon::Green);
    
    assert_eq!(white.display(DisplayStyle::Ascii), "|W|");
    assert_eq!(green.display(DisplayStyle::Ascii), "|G|");
    
    // 测试花牌的ASCII显示
    let spring = Tile::Flower(Flower::Spring);
    let plum = Tile::Flower(Flower::Plum);
    
    assert_eq!(spring.display(DisplayStyle::Ascii), "|F1|");
    assert_eq!(plum.display(DisplayStyle::Ascii), "|F5|");
    
    // 测试百搭牌的ASCII显示
    let joker = Tile::Joker;
    assert_eq!(joker.display(DisplayStyle::Ascii), "|J|");
}

#[test]
fn test_display_tiles() {
    // 创建一组牌
    let tiles = vec![
        Tile::Suit(Suit::Character, 1),
        Tile::Suit(Suit::Character, 2),
        Tile::Suit(Suit::Character, 3),
        Tile::Wind(Wind::East),
    ];
    
    // 测试默认风格的牌组显示
    let default_display = TileDisplay::display_tiles(&tiles, DisplayStyle::Default);
    assert_eq!(default_display, "1万 2万 3万 东");
    
    // 测试简洁风格的牌组显示
    let compact_display = TileDisplay::display_tiles(&tiles, DisplayStyle::Compact);
    assert_eq!(compact_display, "1m 2m 3m E");
    
    // 测试Unicode风格的牌组显示
    let unicode_display = TileDisplay::display_tiles(&tiles, DisplayStyle::Unicode);
    assert_eq!(unicode_display, "🀇 🀈 🀉 🀀");
    
    // 测试ASCII风格的牌组显示
    let ascii_display = TileDisplay::display_tiles(&tiles, DisplayStyle::Ascii);
    assert_eq!(ascii_display, "|1m| |2m| |3m| |E|");
}

#[test]
fn test_colored_display() {
    let man5 = Tile::Suit(Suit::Character, 5);
    let east = Tile::Wind(Wind::East);
    
    // 测试无色模式
    let no_color = man5.display_colored(DisplayStyle::Compact, ColorStyle::None);
    assert_eq!(no_color, "5m");
    
    // 测试有色模式 - 注意这里我们只能测试格式上的正确性，因为终端颜色代码在测试环境中不好验证
    let with_color = man5.display_colored(DisplayStyle::Compact, ColorStyle::Ansi);
    assert!(with_color.contains("5m"));  // 检查包含基本文本
    assert!(with_color.contains("\x1b[")); // 检查包含ANSI转义序列
    
    // 测试一组牌的颜色显示
    let tiles = vec![man5, east];
    let colored_tiles = ColoredTileDisplay::display_tiles_colored(
        &tiles, 
        DisplayStyle::Compact, 
        ColorStyle::Ansi
    );
    
    assert!(colored_tiles.contains("5m"));  // 检查包含第一张牌的文本
    assert!(colored_tiles.contains("E"));   // 检查包含第二张牌的文本
    assert!(colored_tiles.contains("\x1b[")); // 检查包含ANSI转义序列
}

#[test]
fn test_tile_grid() {
    // 创建一组牌
    let tiles = vec![
        Tile::Suit(Suit::Character, 1),
        Tile::Suit(Suit::Character, 2),
        Tile::Suit(Suit::Character, 3),
        Tile::Wind(Wind::East),
        Tile::Wind(Wind::South),
        Tile::Wind(Wind::West),
    ];
    
    // 创建一个2列的网格
    let grid = TileGrid::new(tiles.clone(), 2)
        .with_style(DisplayStyle::Compact);
    
    let grid_string = grid.to_string();
    let lines: Vec<&str> = grid_string.lines().collect();
    
    // 验证网格行数和列数
    assert_eq!(lines.len(), 3);  // 6个牌，每行2个，应该有3行
    assert!(lines[0].contains("1m") && lines[0].contains("2m"));
    assert!(lines[1].contains("3m") && lines[1].contains("E"));
    assert!(lines[2].contains("S") && lines[2].contains("W"));
    
    // 测试3列网格
    let grid3 = TileGrid::new(tiles, 3)
        .with_style(DisplayStyle::Compact);
    
    let grid3_string = grid3.to_string();
    let lines3: Vec<&str> = grid3_string.lines().collect();
    
    // 验证网格行数和列数
    assert_eq!(lines3.len(), 2);  // 6个牌，每行3个，应该有2行
    assert!(lines3[0].contains("1m") && lines3[0].contains("2m") && lines3[0].contains("3m"));
    assert!(lines3[1].contains("E") && lines3[1].contains("S") && lines3[1].contains("W"));
}

#[test]
fn test_red_tiles_display() {
    // 假设红五万是红宝牌，特殊测试
    let red_man5 = Tile::Suit(Suit::Character, 5);
    // 获取基本显示
    assert_eq!(red_man5.display(DisplayStyle::Default), "5万");
    assert_eq!(red_man5.display(DisplayStyle::Compact), "5m");
    
    // 测试颜色显示 - 红牌应该有颜色标记
    let colored = red_man5.display_colored(DisplayStyle::Compact, ColorStyle::Ansi);
    assert!(colored.contains("5m"));
    assert!(colored.contains("\x1b["));  // 包含ANSI颜色代码
}

#[test]
fn test_is_red_with_display() {
    use majiang_core::tile::types::{RED_MAN5_ID, RED_PIN5_ID, RED_SOU5_ID};
    
    // 创建红五万、红五筒、红五条
    let red_man5 = Tile::from_id(RED_MAN5_ID).unwrap();
    let red_pin5 = Tile::from_id(RED_PIN5_ID).unwrap();
    let red_sou5 = Tile::from_id(RED_SOU5_ID).unwrap();
    
    // 检查这些牌是否正确识别为红宝牌
    assert!(red_man5.is_red());
    assert!(red_pin5.is_red());
    assert!(red_sou5.is_red());
    
    // 验证它们的显示方式
    assert_eq!(red_man5.display(DisplayStyle::Default), "5万");
    assert_eq!(red_pin5.display(DisplayStyle::Default), "5筒");
    assert_eq!(red_sou5.display(DisplayStyle::Default), "5条");
    
    // 检查简洁风格
    assert_eq!(red_man5.display(DisplayStyle::Compact), "5m");
    assert_eq!(red_pin5.display(DisplayStyle::Compact), "5p");
    assert_eq!(red_sou5.display(DisplayStyle::Compact), "5s");
}