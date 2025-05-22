use std::fmt::Write;
use std::io::{self, Write as IoWrite};
use std::time::{Duration, Instant};
use std::ptr;

#[cfg(target_os = "windows")]
use windows_sys::Win32::{
    System::Console::SetConsoleTitleW,
    Globalization::{MultiByteToWideChar, CP_UTF8},
};

#[cfg(target_os = "windows")]
fn set_console_title(title: &str) {
    let mut wide_chars: Vec<u16> = title.encode_utf16().collect();
    wide_chars.push(0);
    unsafe {
        SetConsoleTitleW(wide_chars.as_ptr());
    }
}

#[cfg(not(target_os = "windows"))]
fn set_console_title(_title: &str) {
}


fn main() {
    set_console_title("Rust Ultimate Overlord v0.70b");

    let ascii_art = vec![
        r#"  _____           _     _   _______          _       _ "#,
        r#" |  __ \         | |   | | |__   __|        | |     | |"#,
        r#" | |__) |   _ ___| |_  | |    | | ___   ___ | | ___ | |"#,
        r#" |  _  / | | / __| __| | |    | |/ _ \ / _ \| |/ _ \| |"#,
        r#" | | \ \ |_| \__ \ |_  | |____| | (_) | (_) | | (_) | |"#,
        r#" |_|  \_\__,_|___/\__| |______|_|\___/ \___/|_|\___/|_|"#,
        r#"                   Rust Ultimate Overlord v0.70b        "#,
    ];

    let colors = [
        (255, 0, 0),
        (255, 127, 0),
        (255, 255, 0),
        (0, 255, 0),
        (0, 0, 255),
        (75, 0, 130),
        (148, 0, 211),
    ];

    for (i, line) in ascii_art.iter().enumerate() {
        let color = colors[i % colors.len()];
        print!("\x1b[38;2;{};{};{}m{}", color.0, color.1, color.2, line);
        println!();
    }
    println!("\x1b[0m");
    io::stdout().flush().unwrap();

    std::thread::sleep(Duration::from_secs(5));
    print!("\x1b[2J\x1b[H");
    io::stdout().flush().unwrap();

    let bar_length = 50;
    let start_time = Instant::now();
    let total_duration = Duration::from_secs(15);

    loop {
        let elapsed = start_time.elapsed();
        let progress = if elapsed >= total_duration {
            100.0
        } else {
            (elapsed.as_secs_f32() / total_duration.as_secs_f32()) * 100.0
        };

        let filled_chars = (progress / 100.0 * bar_length as f32) as usize;
        let mut bar = String::new();

        for i in 0..bar_length {
            if i < filled_chars {
                let r = (255.0 * (1.0 - (i as f32 / (bar_length -1).max(1) as f32))) as u8;
                let g = (255.0 * (i as f32 / (bar_length -1).max(1) as f32)) as u8;
                let b = 0;
                write!(&mut bar, "\x1b[38;2;{};{};{}m█", r, g, b).unwrap();
            } else {
                write!(&mut bar, "\x1b[0m ").unwrap();
            }
        }

        write!(&mut bar, "\x1b[0m {:.1}%", progress).unwrap();
        print!("\r{}", bar);
        io::stdout().flush().unwrap();

        if progress >= 100.0 {
            println!("\x1b[0m");
            break;
        }

        std::thread::sleep(Duration::from_millis(50));
    }

    print!("\x1b[2J\x1b[H");
    println!("\x1b[32mActivation successful! Closing console in 5 seconds...\x1b[0m");
    io::stdout().flush().unwrap();

    std::thread::sleep(Duration::from_secs(5));
}
