extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::terminal_size;
use termion::screen::AlternateScreen;

use std::fs::File;
use std::io::{Write, stdout, stdin, Stdout};
use std::convert::TryInto;

// TODO: Keep file in memory, in vector, memory mapped files
// TODO: Scroll
// TODO: Change cursor based on mode
// TODO: Handle wrapping for very long lines

fn write_to_file(content: Vec<Vec<u8>>) {
    let mut f = File::create("tmp.py").unwrap();

    for item in content {
        f.write(item.as_slice()).unwrap();
    }
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut screen = AlternateScreen::from(stdout);

    write!(screen, "{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1)).unwrap();
    screen.flush().unwrap();

    let mut x: usize = 1;
    let mut y: usize = 1;
    let (horizontal, vertical) = terminal_size().unwrap();
    let mut a: Vec<Vec<u8>> = Vec::new();
    a.push(Vec::new());

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Esc => {
                write_to_file(a);
                break;
            }
            Key::Left => {
                if x > 1 {
                    x -= 1;
                }
            }
            Key::Right => {
                if x <= a[y - 1].len() {
                    x += 1
                }
            }
            Key::Up => {
                // TODO: If line is shorter than one below, move cursor
                if y > 1 {
                    y -= 1;
                }
            }
            Key::Down => {
                // TODO: If line is shorter than one above, move cursor
                if y < a.len().try_into().unwrap() {
                    y += 1;
                }
            }
            Key::Char(c) => {
                if c == '\n' {
                    // TODO: Based on cursor, insert new line and move text after below
                    match a[y-1].last() {
                        Some(c) if *c != '\n' as u8 => {
                            a[y-1].push('\n' as u8);
                        }
                        _ => {}
                    }
                    a.insert(y, Vec::new());
                    a[y].push('\n' as u8);
                    x = 1;
                    y += 1;
                } else if c == '\t' {
                    // TODO: Find a better way to insert tab
                    a[y - 1].insert(x - 1, 0x20);
                    x += 1;
                    a[y - 1].insert(x - 1, 0x20);
                    x += 1;
                    a[y - 1].insert(x - 1, 0x20);
                    x += 1;
                    a[y - 1].insert(x - 1, 0x20);
                    x += 1;
                } else {
                    // TODO: Fix tmp hax on empty line, the second boolean is a quick hack
                    if x == a[y - 1].len() && a[y-1][x-1] != '\n' as u8 {
                        a[y - 1].push(c as u8);
                    } else {
                        a[y - 1].insert(x - 1, c as u8);
                    }
                    x += 1;
                }
            }
            Key::Backspace => {
                if x - 1 == a[y - 1].len() {
                    a[y - 1].pop();
                } else {
                    a[y - 1].remove(x - 1);
                }
                x -= 1;
                write!(screen, "{}", termion::clear::CurrentLine).unwrap();
            }
            _ => {}
        }
        debug_message(&mut screen, format!("a={:?}", a));
        draw_to_screen(&mut screen, a.clone(), x, y);
    }

    write!(screen, "{}", termion::cursor::Show).unwrap();
}


fn draw_to_screen(mut screen: &mut AlternateScreen<RawTerminal<Stdout>>, lines: Vec<Vec<u8>>, x: usize, y: usize) {
    let mut count = 1;
    for item in lines {
        let mut b = String::from_utf8(item.clone()).unwrap();
        if item == [0xa] {
            write!(screen,
                   "{}{}{}",
                   termion::cursor::Goto(1, count),
                   termion::clear::CurrentLine,
                   termion::cursor::Goto(x as u16, y as u16)).unwrap();
        } else {
            write!(screen,
                   "{}{}{}",
                   termion::cursor::Goto(1, count),
                   b,
                   termion::cursor::Goto(x as u16, y as u16)).unwrap();
        }
        count += 1;
    }
    screen.flush().unwrap();
}

fn debug_message(screen: &mut AlternateScreen<RawTerminal<Stdout>>, debug_message: String) {
    write!(screen, "{}{}{}", termion::cursor::Goto(50, 50), termion::clear::CurrentLine, debug_message).unwrap();
    screen.flush().unwrap();
}