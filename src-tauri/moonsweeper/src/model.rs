use crate::util::{self,  XX};

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

impl Keyboard {
    pub fn new(model: KeyboardModel) -> Self {
        return match model {
            KeyboardModel::Moonlander => Keyboard {
                model_layout: util::MOONLANDER_MODEL_LAYOUT,
                key_layout: util::MOONLANDER_KEY_LAYOUT,
            },
        };
    }

    pub fn get_position(&self, coord: Coord) -> usize {
        self.model_layout[coord.x][coord.y]
    }

    pub fn get_char_position(&self, char: char) -> usize {
         for x in 0..util::MOONLANDER_MODEL_LAYOUT.len() {
             for y in 0..util::MOONLANDER_MODEL_LAYOUT[x].len() {
                if self.key_layout[x][y] == char {
                    return self.model_layout[x][y];
                }
            }
        }
        XX
    }
}
