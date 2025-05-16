use std::fmt::Write;
use std::io::{self, Write as IoWrite};
use std::time::{Duration, Instant};

fn main() {
    // Отображение ASCII-арта
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
        (255, 0, 0),    // Красный
        (255, 127, 0),  // Оранжевый
        (255, 255, 0),  // Желтый
        (0, 255, 0),    // Зеленый
        (0, 0, 255),    // Синий
        (75, 0, 130),   // Индиго
        (148, 0, 211),  // Фиолетовый
    ];

    for (i, line) in ascii_art.iter().enumerate() {
        let color = colors[i % colors.len()];
        print!("\x1b[38;2;{};{};{}m{}", color.0, color.1, color.2, line);
        println!();
    }
    println!("\x1b[0m");
    io::stdout().flush().unwrap();

    // Ожидание 25 секунд
    std::thread::sleep(Duration::from_secs(25));

    // Очистка экрана
    print!("\x1b[2J\x1b[H");
    io::stdout().flush().unwrap();

    // Прогресс-бар
    let bar_length = 50;
    let start_time = Instant::now();
    let total_duration = Duration::from_secs(45);

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
                // Исправленные вычисления цвета
                let r = (255.0 * (1.0 - (i as f32 / (bar_length - 1) as f32))) as u8;
                let g = (255.0 * (i as f32 / (bar_length - 1) as f32)) as u8;
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
            break;
        }

        std::thread::sleep(Duration::from_millis(50));
    }

    // Завершающее сообщение
    print!("\x1b[2J\x1b[H");
    println!("\x1b[32mActivation successful! Closing console in 25 seconds...\x1b[0m");
    io::stdout().flush().unwrap();

    std::thread::sleep(Duration::from_secs(25));
}
