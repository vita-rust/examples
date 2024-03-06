use std::backtrace::Backtrace;
use std::fmt::Write;
use std::panic::{self, PanicInfo};
use std::thread;
use std::time::Duration;

mod debug;

pub fn main() {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        custom_panic_hook(info);
        default_hook(info);
    }));

    let mut screen = debug::screen::DebugScreen::new();
    writeln!(screen, "This not-so-bare-metal is starting to rust!").ok();
    thread::sleep(Duration::from_secs(2));
    writeln!(screen, "See? Told ya!").ok();
    thread::sleep(Duration::from_secs(2));

    let random_numbers: Vec<u8> = (0..8).map(|_i| rand::random::<u8>()).collect();
    writeln!(screen, "Some random numbers: {:?}", random_numbers).ok();

    thread::sleep(Duration::from_secs(5));
}

fn custom_panic_hook(info: &PanicInfo<'_>) {
    // The current implementation always returns `Some`.
    let location = info.location().unwrap();

    let msg = match info.payload().downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<Any>",
        },
    };
    let name = "unknown";

    let mut screen = debug::screen::DebugScreen::new();

    writeln!(
        screen,
        "thread '{}' panicked at '{}', {}",
        name, msg, location
    )
    .ok();

    // Give 2 seconds to see the error in case capturing the stack trace fails
    // (capturing the stack trace allocates memory)
    thread::sleep(Duration::from_secs(2));

    // The backtrace is full of "unknown" as there's no elf to parse on the vita
    let backtrace = Backtrace::force_capture();
    writeln!(screen, "{}", backtrace).ok();

    thread::sleep(Duration::from_secs(10));
}
