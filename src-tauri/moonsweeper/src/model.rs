use crate::util;

pub enum KeyboardModel {
    Moonlander,
}

pub struct Keyboard {
    pub layout: [[usize; 14]; 6],
}

pub struct Coord {
    pub x: usize,
    pub y: usize,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Keyboard {
    pub fn new(model: KeyboardModel) -> Self {
        let layout = match model {
            KeyboardModel::Moonlander => util::MOONLANDER_LAYOUT,
        };
        Keyboard { layout }
    }

    pub fn get_position(&self, coord: Coord) -> usize {
        self.layout[coord.x][coord.y]
    }
}

