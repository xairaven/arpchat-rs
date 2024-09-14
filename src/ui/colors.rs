use crate::net::ktp;
use cursive::backends::crossterm::crossterm::style::Color;

pub fn from_id(id: &ktp::Id) -> Color {
    const COLOR_COUNT: usize = 8;
    const COLORS: [Color; COLOR_COUNT] = [
        Color::Red,
        Color::DarkRed,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ];

    let index = id
        .iter()
        .copied()
        .reduce(|acc, el| acc.overflowing_add(el).0)
        .unwrap();
    let index = (index as usize) % COLOR_COUNT;
    COLORS[index]
}
