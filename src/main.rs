use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 512;
const HEIGHT: usize = 512;
const STEPS: usize = 512;



pub struct Display64x64 {
    pixels: [[bool; WIDTH]; HEIGHT],
}

impl Display64x64 {
    pub fn new() -> Self {
        Self {
            pixels: [[false; WIDTH]; HEIGHT],
        }
    }

    #[allow(non_snake_case)]
    pub fn SetPixel(&mut self, x: usize, y: usize, value: u8) {
        if x < WIDTH && y < HEIGHT {
            self.pixels[y][x] = value != 0;
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<bool> {
        if x < WIDTH && y < HEIGHT {
            Some(self.pixels[y][x])
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.pixels = [[false; WIDTH]; HEIGHT];
    }

    pub fn to_u32_buffer(&self) -> Vec<u32> {
        let mut buffer = Vec::with_capacity(WIDTH * HEIGHT);
        for row in &self.pixels {
            for &pixel in row {
                let color = if pixel { 0xFFFFFF } else { 0x000000 };
                buffer.push(color);
            }
        }
        buffer
    }
}

impl Default for Display64x64 {
    fn default() -> Self {
        Self::new()
    }
}

fn apply_rule_126(left: bool, center: bool, right: bool) -> bool {
    (left || center || right) && !(left && center && right)
}

fn draw_row(display: &mut Display64x64, y: usize, row: &[bool]) {
    for (x, &pixel) in row.iter().enumerate().take(WIDTH) {
        display.SetPixel(x, y, u8::from(pixel));
    }
}

fn generate_automaton(display: &mut Display64x64) {
    let mut row = vec![false; WIDTH];
    row[WIDTH / 2] = true;

    let rows_to_draw = STEPS.min(HEIGHT);
    for y in 0..rows_to_draw {
        draw_row(display, y, &row);

        let mut next_row = vec![false; WIDTH];
        for x in 0..WIDTH {
            let left = if x > 0 { row[x - 1] } else { false };
            let center = row[x];
            let right = if x + 1 < WIDTH { row[x + 1] } else { false };
            next_row[x] = apply_rule_126(left, center, right);
        }

        row = next_row;
    }
}



fn main() {
    let mut display = Display64x64::new();

    let mut window = Window::new(
        "Display 512x512",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale: Scale::X1,
            scale_mode: ScaleMode::Stretch,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    generate_automaton(&mut display);

    // Limit to max ~60 fps update rate.
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&display.to_u32_buffer(), WIDTH, HEIGHT)
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_display_starts_empty() {
        let display = Display64x64::new();

        assert_eq!(display.get_pixel(WIDTH / 2, 0), Some(false));
        assert_eq!(display.get_pixel(0, 0), Some(false));
    }

    #[test]
    fn set_pixel_turns_pixel_on_and_off() {
        let mut display = Display64x64::new();

        display.SetPixel(1, 1, 1);
        assert_eq!(display.get_pixel(1, 1), Some(true));

        display.SetPixel(1, 1, 0);
        assert_eq!(display.get_pixel(1, 1), Some(false));
    }

    #[test]
    fn apply_rule_behaves_as_expected() {
        assert_eq!(apply_rule_126(false, false, false), false);
        assert_eq!(apply_rule_126(false, false, true), true);
        assert_eq!(apply_rule_126(true, false, false), true);
        assert_eq!(apply_rule_126(false, true, false), true);
        assert_eq!(apply_rule_126(false, true, true), true);
        assert_eq!(apply_rule_126(true, false, true), true);
        assert_eq!(apply_rule_126(true, true, false), true);
        assert_eq!(apply_rule_126(true, true, true), false);
    }
}
