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
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        ascii::set_seed(seed);

        app::run(&args)?;
    } else {
        println!("\"There's no limit to the larp\"");
        println!("Use -h for help!\n");

        println!("████      ▄███████▄ ███████▄  ███████▄");
        println!("████      ███   ███ ███   ███ ███   ███");
        println!("████      █████████ ████████  ███████▀");
        println!("████▄▄▄▄▄ ███   ███ ███   ███ ███");
        println!("▀▀▀▀▀▀▀▀▀ ▀▀▀   ▀▀▀ ▀▀▀   ▀▀▀ ▀▀▀");
    }

    Ok(())
}