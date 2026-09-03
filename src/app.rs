// Copyright (c) Akeoot <akeoot@pm.me>. Licensed under the LGPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute, terminal,
};
use std::io::{self, Write};
use std::time::{Duration, Instant};

use crate::cli::Args;
use crate::render::{render_frame, TerminalGuard};

/// Set up the terminal and run the larp mode.
pub fn run(args: &Args) -> io::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(
        io::stdout(),
        terminal::EnterAlternateScreen,
        cursor::Hide
    )?;

    // Ensure the terminal resets if the app panics or ends
    let _guard = TerminalGuard;
    let mut stdout = io::stdout();

    run_fall_mode(args, &mut stdout)?;

    Ok(())
}

/// Main loop: scroll the ASCII art field indefinitely,
/// reacting to key presses and terminal resizes.
fn run_fall_mode(args: &Args, stdout: &mut impl Write) -> io::Result<()> {
    let mut scroll_x: i64 = 0;
    let mut scroll_y: i64 = 0;

    render_frame(stdout, scroll_x, scroll_y, args)?;

    let frame_delay = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
        let timeout = frame_delay.saturating_sub(last_tick.elapsed());

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if handle_key_event(&key) {
                        break;
                    }
                }
                Event::Resize(_, _) => {
                    render_frame(stdout, scroll_x, scroll_y, args)?;
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= frame_delay {
            update_and_render(stdout, &mut scroll_x, &mut scroll_y, args)?;
            last_tick = Instant::now();
        }
    }

    Ok(())
}

/// Process a key event. Returns `true` if the application should exit.
fn handle_key_event(key: &event::KeyEvent) -> bool {
    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
        return true;
    }
    if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
        return true;
    }
    false
}

/// Move the viewport diagonally and re‑render.
fn update_and_render(
    stdout: &mut impl Write,
    scroll_x: &mut i64,
    scroll_y: &mut i64,
    args: &Args,
) -> io::Result<()> {
    *scroll_x += 1;
    *scroll_y += 1;
    render_frame(stdout, *scroll_x, *scroll_y, args)
}