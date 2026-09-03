// Copyright (c) Akeoot <akeoot@pm.me>. Licensed under the LGPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.

mod app;
mod ascii;
mod cli;
mod render;

use clap::Parser;
use cli::Args;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.larp || args.color.is_some() {
        app::run(&args)?;
    } else {
        println!("\n\"The larp can never end\"");
        println!("Use -h for help, you larper!\n");

        println!("████      ▄███████▄ ███████▄  ███████▄");
        println!("████      ███   ███ ███   ███ ███   ███");
        println!("████      █████████ ████████  ███████▀");
        println!("████▄▄▄▄▄ ███   ███ ███   ███ ███");
        println!("▀▀▀▀▀▀▀▀▀ ▀▀▀   ▀▀▀ ▀▀▀   ▀▀▀ ▀▀▀");
    }

    Ok(())
}