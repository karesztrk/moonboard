use crate::util::{self, XX};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum KeyboardLayout {
    Qwerty,
    Norman,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardModel {
    Moonlander,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Keyboard {
    pub model_layout: [[usize; 14]; 6],
    pub key_layout: [[char; 14]; 6],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Character(char),
    Space,
    Enter,
    Backspace,
    Tab,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn black() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}

impl Keyboard {
    pub fn new(model: KeyboardModel, layout: KeyboardLayout) -> Self {
        let model_layout_data = match model {
            KeyboardModel::Moonlander => util::MODEL_LAYOUT,
        };
        let key_layout_data = match layout {
            KeyboardLayout::Qwerty => util::QWERTY_LAYOUT,
            KeyboardLayout::Norman => util::NORMAN_LAYOUT,
        };
        Self {
            model_layout: model_layout_data,
            key_layout: key_layout_data,
        }
    }

    pub fn get_position(&self, coord: Coord) -> usize {
        self.model_layout[coord.x][coord.y]
    }

    pub fn get_char_position(&self, char: char) -> usize {
        for x in 0..util::MODEL_LAYOUT.len() {
            for y in 0..util::MODEL_LAYOUT[x].len() {
                if self.key_layout[x][y] == char {
                    return self.model_layout[x][y];
                }
            }
        }
        XX
    }

    pub fn get_char_coord(&self, char: char) -> Coord {
        for x in 0..util::MODEL_LAYOUT.len() {
            for y in 0..util::MODEL_LAYOUT[x].len() {
                if self.key_layout[x][y] == char {
                    return Coord { x, y };
                }
            }
        }
        Coord { x: XX, y: XX }
    }
}
