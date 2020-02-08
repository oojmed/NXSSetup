use crate::options;

extern crate crossterm;
extern crate termion;

use crossterm::{
    execute, cursor, terminal, Result
};

use std::io::{stdout, Write};

pub fn hide_cursor() {
    if !cfg!(windows) {
        print!("{}", termion::cursor::Hide);
    }
}

pub fn show_cursor() {
    if !cfg!(windows) {
        print!("{}", termion::cursor::Show);
    }
}

pub fn clear() {
    let mut stdout = stdout();

    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    print!("{}", termion::cursor::Goto(1, 1));
}

pub fn center(text: &str, length: u16, cursor_y: u16) -> String {
    let (term_width, term_height) = termion::terminal_size().unwrap();

    return format!("{}{}", termion::cursor::Goto(term_width / 2 - (length / 2), cursor_y as u16), text);
}

pub fn title(text: &str) -> String {
    return underline(bold(text).as_str());
}

pub fn back_rgb(r: i32, g: i32, b: i32, text: &str) -> String {
    unsafe {
        if options::OPTIONS.color && options::OPTIONS.styling {
            return format!("\x1b[48;2;{};{};{}m{}\x1b[0m", r, g, b, text).to_string();
        } else {
            return text.to_string();
        }
    }
}

pub fn for_rgb(r: i32, g: i32, b: i32, text: &str) -> String {
    unsafe {
        if options::OPTIONS.color && options::OPTIONS.styling {
            return format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, text).to_string();
        } else {
            return text.to_string();
        }
    }
}

pub fn underline(text: &str) -> String {
    unsafe {
        if options::OPTIONS.styling {
            return format!("\x1b[4m{}\x1b[0m", text).to_string();
        } else {
            return text.to_string();
        }
    }
}
  
pub fn bold(text: &str) -> String {
    unsafe {
        if options::OPTIONS.styling {
            return format!("\x1b[1m{}\x1b[0m", text).to_string();
        } else {
            return text.to_string();
        }
    }
}

pub fn italic(text: &str) -> String {
    unsafe {
        if options::OPTIONS.styling {
            return format!("\x1b[3m{}\x1b[0m", text).to_string();
        } else {
            return text.to_string();
        }
    }
}

pub fn strikethrough(text: &str) -> String {
    unsafe {
        if options::OPTIONS.styling {
            return format!("\x1b[9m{}\x1b[0m", text).to_string();
        } else {
            return text.to_string();
        }
    }
}