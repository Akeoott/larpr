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
    let mut buffer = vec![' '; width * height];

    // Maximum possible width/height of any art (used to expand the cell search range)
    let max_art_w = ascii::VARIANTS.iter().map(|a| a.width() as i64).max().unwrap_or(0);
    let max_art_h = ascii::VARIANTS.iter().map(|a| a.height() as i64).max().unwrap_or(0);

    // Determine the range of cells that could intersect the viewport.
    let min_cx = (scroll_x - max_art_w).div_euclid(ascii::GRID_PITCH_X) - 1;
    let max_cx = (scroll_x + width as i64 + max_art_w).div_euclid(ascii::GRID_PITCH_X) + 1;
    let min_cy = (scroll_y - max_art_h).div_euclid(ascii::GRID_PITCH_Y) - 1;
    let max_cy = (scroll_y + height as i64 + max_art_h).div_euclid(ascii::GRID_PITCH_Y) + 1;

    for cy in min_cy..=max_cy {
        for cx in min_cx..=max_cx {
            if let Some((variant, ox, oy)) = ascii::placed_art(cx, cy) {
                let art = &ascii::VARIANTS[variant];
                // Draw the art onto the buffer, but only if it is inside the viewport.
                for (local_y, line) in art.lines.iter().enumerate() {
                    let global_y = oy + local_y as i64;
                    let screen_y = global_y - scroll_y;
                    if screen_y < 0 || screen_y >= height as i64 {
                        continue;
                    }
                    for (local_x, ch) in line.chars().enumerate() {
                        if ch != ' ' {
                            let global_x = ox + local_x as i64;
                            let screen_x = global_x - scroll_x;
                            if screen_x >= 0 && screen_x < width as i64 {
                                let idx = screen_y as usize * width + screen_x as usize;
                                buffer[idx] = ch;
                            }
                        }
                    }
                }
            }
        }
    }

    buffer.into_iter().collect()
}