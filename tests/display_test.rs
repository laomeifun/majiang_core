use majiang_core::tile::{
    Tile, Suit, Wind, Dragon, Flower,
    DisplayStyle, TileDisplay, ColorStyle, ColoredTileDisplay, TileGrid,
};

#[test]
fn test_default_display() {
    // 测试数牌的默认显示
    let man_tile = Tile::Suit(Suit::Character, 5);
    let pin_tile = Tile::Suit(Suit::Dot, 7);
    
    assert_eq!(man_tile.display(DisplayStyle::Default), "5万");
    assert_eq!(pin_tile.display(DisplayStyle::Default), "7筒");
}

#[test]
fn test_compact_display() {
    // 测试数牌的简洁显示
    let man_tile = Tile::Suit(Suit::Character, 5);
    let pin_tile = Tile::Suit(Suit::Dot, 7);
    
    assert_eq!(man_tile.display(DisplayStyle::Compact), "5m");
    assert_eq!(pin_tile.display(DisplayStyle::Compact), "7p");
}

#[test]
fn test_unicode_display() {
    // 测试数牌的Unicode显示
    let man1 = Tile::Suit(Suit::Character, 1);
    let pin5 = Tile::Suit(Suit::Dot, 5);
    
    assert_eq!(man1.display(DisplayStyle::Unicode), "🀇");
    assert_eq!(pin5.display(DisplayStyle::Unicode), "🀝");
}