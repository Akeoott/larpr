// Copyright (c) Akeoot <akeoot@pm.me>. Licensed under the LGPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.

use clap::{Parser, ValueEnum};
use crossterm::style::Color;

/// Color choices for the ASCII art foreground.
#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum ColorChoice {
    White,
    Blue,
    Cyan,
    Green,
    Yellow,
    Red,
}

impl From<ColorChoice> for Color {
    fn from(c: ColorChoice) -> Self {
        match c {
            ColorChoice::White => Color::White,
            ColorChoice::Blue => Color::Blue,
            ColorChoice::Cyan => Color::Cyan,
            ColorChoice::Green => Color::Green,
            ColorChoice::Yellow => Color::Yellow,
            ColorChoice::Red => Color::Red,
        }
    }
}

/// Command-line arguments for `larpr`.
#[derive(Parser)]
#[command(name = "larpr")]
#[command(version)]
#[command(about = "Jarvis, enable larp mode\n\
                   HelloWorld(\"print\")\n\n\
                   (larpr is written in rust btw)")]
pub struct Args {
    /// Enable the full larp experience (infinite scrolling ASCII art).
    #[arg(short = 'l', long, default_value_t = false)]
    pub larp: bool,

    /// Set a fixed foreground color for all art.
    #[arg(short = 'c', long, value_enum)]
    pub color: Option<ColorChoice>,
}