// Copyright (c) Akeoot <akeoot@pm.me>. Licensed under the LGPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.

use crossterm::{
    cursor, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{self, Write};
use crate::ascii;
use crate::cli::Args;

/// Restores the terminal when dropped.
pub struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = crossterm::execute!(
            io::stdout(),
            cursor::Show,
            terminal::LeaveAlternateScreen
        );
        let _ = terminal::disable_raw_mode();
    }
}

/// Render the current viewport to the terminal.
pub fn render_frame(
    stdout: &mut impl Write,
    scroll_x: i64,
    scroll_y: i64,
    args: &Args,
) -> io::Result<()> {
    let (width, height) = terminal::size()?;
    let width = width as usize;
    let height = height as usize;

    queue!(
        stdout,
        cursor::MoveTo(0, 0),
        terminal::Clear(ClearType::All)
    )?;

    if let Some(color) = args.color {
        queue!(stdout, SetForegroundColor(Color::from(color)))?;
    }

    let screen_buffer = build_screen_buffer(width, height, scroll_x, scroll_y);
    queue!(stdout, Print(screen_buffer))?;

    if args.color.is_some() {
        queue!(stdout, ResetColor)?;
    }

    stdout.flush()
}

/// Construct a flat string containing all characters for the current viewport.
fn build_screen_buffer(width: usize, height: usize, scroll_x: i64, scroll_y: i64) -> String {
    let mut buffer = String::with_capacity(width * height + height);
    for y in 0..height {
        for x in 0..width {
            let global_x = scroll_x + x as i64;
            let global_y = scroll_y + y as i64;
            buffer.push(ascii::char_at(global_x, global_y));
        }
    }
    buffer
}