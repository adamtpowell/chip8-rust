mod emulator;
mod screen;

use screen::{U32BufferScreen};
use emulator::{Emulator};
use minifb::{Window, WindowOptions, Key};

fn main() {
    let mut emulator = Emulator::new("test");
    let mut buffer_screen = U32BufferScreen::new(emulator::WIDTH, emulator::HEIGHT);

    let mut window = Window::new(
        "Chip-8",
        buffer_screen.width as usize,
        buffer_screen.height as usize,
        WindowOptions {
            borderless: false,
            title: true,
            resize: false,
            scale: minifb::Scale::X2,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            topmost: false,
            transparency: false,
        },
    ).unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(1000000 / 24)));
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // This is a very manual mapping of input right now, this may change in the future but is fine for now. 
        emulator.update::<U32BufferScreen>(&emulator::HexPad {
            zero: window.is_key_down(Key::X),
            one: window.is_key_down(Key::Key1),
            two: window.is_key_down(Key::Key2),
            three: window.is_key_down(Key::Key3),
            four: window.is_key_down(Key::Q),
            five: window.is_key_down(Key::W),
            six: window.is_key_down(Key::E),
            seven: window.is_key_down(Key::A),
            eight: window.is_key_down(Key::S),
            nine: window.is_key_down(Key::D),
            a: window.is_key_down(Key::Z),
            b: window.is_key_down(Key::C),
            c: window.is_key_down(Key::Key4),
            d: window.is_key_down(Key::R),
            e: window.is_key_down(Key::F),
            f: window.is_key_down(Key::V),
        }, &mut buffer_screen);
        window
            .update_with_buffer(&buffer_screen.get_buffer(), buffer_screen.width as usize, buffer_screen.height as usize)
            .unwrap();
    }
}