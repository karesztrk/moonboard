use std::future::Future;

use kontroll::Kontroll;
use rand::Rng;

use crate::util;

use super::model::*;

pub trait Animation {
    fn play(&self, app: &Kontroll) -> impl Future<Output = ()> {
        async {
            self.run(app).await;
            self.clean(app).await;
        }
    }
    fn run(&self, app: &Kontroll) -> impl Future<Output = ()>;

    fn clean(&self, app: &Kontroll) -> impl Future<Output = ()>;
}

pub struct Sequence {
    color: Color,
    keyboard: Keyboard,
    speed: f64,
}

pub struct Wipe {
    color: Color,
    speed: f64,
    velocity: f64,
    keyboard: Keyboard,
}

pub struct SingleKey {
    keyboard: Keyboard,
    char: char,
    color: Color,
}

pub struct Clear {
    keyboard: Keyboard,
    color: Color,
}

impl Clear {
    pub fn new(keyboard: Keyboard) -> Self {
        Self {
            color: Color::black(),
            keyboard,
        }
    }
}

impl SingleKey {
    pub fn new(keyboard: Keyboard, char: char) -> Self {
        let mut rng = rand::thread_rng();

        let r = rng.gen_range(0..255);
        let g = rng.gen_range(0..255);
        let b = rng.gen_range(0..255);
        Self {
            color: Color { r, g, b },
            keyboard,
            char,
        }
    }
}

impl Sequence {
    pub fn new(keyboard: Keyboard) -> Self {
        let mut rng = rand::thread_rng();

        let r = rng.gen_range(0..255);
        let g = rng.gen_range(0..255);
        let b = rng.gen_range(0..255);
        Self {
            color: Color { r, g, b },
            keyboard,
            speed: 0.5,
        }
    }

    async fn flash_up(&self, coord: Coord, app: &Kontroll) {
        let flash_speed = (self.speed / 2.0 * 1000.0) as u64;

        let position = self.get_position(coord);
        if position != util::XX {
            let Color { r, g, b } = self.color;
            let _ = app.set_rgb_led(position, r, g, b, 0).await;
            std::thread::sleep(std::time::Duration::from_millis(flash_speed));
        }
    }

    async fn left(&self, app: &Kontroll) {
        let layout = self.keyboard.model_layout;
        let height = layout.len();
        let width = layout[0].len();

        for x in 0..height {
            for y in 0..width / 2 {
                self.flash_up(Coord { x, y }, &app).await
            }
        }
    }

    async fn right(&self, app: &Kontroll) {
        let layout = self.keyboard.model_layout;
        let height = layout.len();
        let width = layout[0].len();

        for x in 0..height {
            for y in width / 2..width {
                self.flash_up(Coord { x, y }, &app).await
            }
        }
    }

    async fn flash_up_all(&self, app: &Kontroll) {
        let flash_speed = (self.speed / 2.0 * 1000.0) as u64;
        let Color { r, g, b } = self.color;

        let _ = app.set_rgb_all(r, g, b, 0).await;
        std::thread::sleep(std::time::Duration::from_millis(flash_speed));
        let _ = app.set_rgb_all(0, 0, 0, 0).await;
        std::thread::sleep(std::time::Duration::from_millis(flash_speed));
    }

    fn get_position(&self, coord: Coord) -> usize {
        self.keyboard.get_position(coord)
    }
}

impl Animation for Sequence {
    async fn run(&self, app: &Kontroll) {
        self.left(app).await;
        self.right(app).await;

        for _ in 0..4 {
            self.flash_up_all(&app).await;
        }

        let _ = app.restore_rgb_leds().await;
    }

    async fn clean(&self, app: &Kontroll) {
        let _ = app.restore_rgb_leds().await;
    }
}

impl Wipe {
    pub fn new(keyboard: Keyboard) -> Self {
        let mut rng = rand::thread_rng();

        let r = rng.gen_range(0..255);
        let g = rng.gen_range(0..255);
        let b = rng.gen_range(0..255);

        Self {
            color: Color { r, g, b },
            keyboard,
            speed: 0.25,
            velocity: 5.0,
        }
    }

    fn get_position(&self, coord: Coord) -> usize {
        self.keyboard.get_position(coord)
    }

    async fn clear_column(&self, col: usize, app: &Kontroll) {
        let height = self.keyboard.model_layout.len();

        for x in 0..height {
            let position = self.get_position(Coord { x, y: col });
            if position != util::XX {
                let _ = app.set_rgb_led(position, 0, 0, 0, 0).await;
            }
        }
    }

    async fn left_right(&self, app: &Kontroll) {
        let layout = self.keyboard.model_layout;
        let height = layout.len();
        let width = layout[0].len();

        for y in 0..width {
            for x in 0..height {
                let position = self.get_position(Coord { x, y });
                if position != util::XX {
                    let Color { r, g, b } = self.color;
                    let _ = app.set_rgb_led(position, r, g, b, 0).await;
                }
            }

            let flash_speed = (self.speed / 2.0 * 1000.0 - y as f64 * self.velocity) as u64;

            std::thread::sleep(std::time::Duration::from_millis(flash_speed));

            if y > 0 {
                self.clear_column(y - 1, app).await;
            }
        }
    }

    async fn right_left(&self, app: &Kontroll) {
        let layout = self.keyboard.model_layout;
        let height = layout.len();
        let width = layout[0].len();

        for y in (0..width).rev() {
            for x in (0..height).rev() {
                let position = self.get_position(Coord { x, y });
                if position != util::XX {
                    let Color { r, g, b } = self.color;
                    let _ = app.set_rgb_led(position, r, g, b, 0).await;
                }
            }

            let flash_speed =
                (self.speed / 2.0 * 1000.0 - (width - y) as f64 * self.velocity) as u64;

            std::thread::sleep(std::time::Duration::from_millis(flash_speed));

            if y < width - 1 {
                self.clear_column(y + 1, app).await;
            }
        }
    }
}

impl Animation for Wipe {
    async fn run(&self, app: &Kontroll) {
        for _ in 0..3 {
            self.left_right(&app).await;
            self.right_left(&app).await;
        }
    }

    async fn clean(&self, app: &Kontroll) {
        let _ = app.restore_rgb_leds().await;
    }
}

impl Animation for SingleKey {
    async fn run(&self, app: &Kontroll) {
        for _ in 0..3 {
            let position = self.keyboard.get_char_position(self.char);
            if position != util::XX {
                let Color { r, g, b } = self.color;
                let _ = app.set_rgb_led(position, r, g, b, 0).await;
            }
        }
    }

    async fn clean(&self, app: &Kontroll) {
        let _ = app.restore_rgb_leds().await;
    }
}

impl Animation for Clear {
    async fn run(&self, app: &Kontroll) {
        let Color { r, g, b } = self.color;
        let _ = app.set_rgb_all(r, g, b, 0).await;
    }

    async fn clean(&self, app: &Kontroll) {
        let _ = app.restore_rgb_leds().await;
    }
}
