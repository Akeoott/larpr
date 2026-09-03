# larpr - There's no limit to the larp.

What is larpr?<br>
larpr transforms your terminal into an endless field of LARP, scrolling diagonally forever.<br>
It's written in Rust, powered by crossterm, and designed to bring a little chaos to your command line.

### Quick Start

Clone and run `larpr` from source
```bash
git clone https://github.com/Akeoott/larpr
cd ./larpr

cargo install
cargo run -- -h

# To permanently install on system
makepkg -si
```

Requires rust and cargo. Quick setup:
```bash
sudo pacman -S rustup
rustup default stable
```

### Key Features

- **Infinite larp** – Launch with `-l` or `--larp` and watch a seamless, procedurally generated field of ASCII art scroll across your screen.
- **Multiple ASCII art variants** – A diverse set of styles, from binary and hex to large block letters, are placed across the grid using deterministic hashing. No two cells are the same.
- **Color customization** – Choose from white, blue, cyan, green, yellow, or red foreground colors via `-c` or `--color`.
- **Arch Linux packaging** – A `PKGBUILD` and `.SRCINFO` are included for easy installation on Arch-based systems.
- **Command-line help** – Run `larpr -h` for options and a friendly reminder to larp.

<br>

# I use arch btw
