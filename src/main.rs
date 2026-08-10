use rpntui::{Stack, render};
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::clear;
use termion::terminal_size;
use termion::cursor;
use std::io::{Write, stdin, stdout};

fn main() {
    let mut stack: Stack<f64> = Stack::new();
    stack.push(0.0);
    stack.push(0.0);
    stack.push(0.0);
    stack.push(0.0);
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}", clear::All);
    let (columns, rows) = terminal_size().unwrap();
    write!(stdout, "{}{}x{}", cursor::Goto(0, rows), rows, columns);
    render(stack);
    stdout.flush();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            _  => print!("hups"),
        }
        stdout.flush().unwrap();
    }
}
