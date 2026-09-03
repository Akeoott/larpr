// Copyright (c) Akeoot <akeoot@pm.me>. Licensed under the LGPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.

/// A static ASCII art pattern stored as a slice of lines.
pub struct AsciiArt {
    pub lines: &'static [&'static str],
}

impl AsciiArt {
    /// Maximum line width in characters.
    pub fn width(&self) -> usize {
        self.lines.iter().map(|l| l.chars().count()).max().unwrap_or(0)
    }

    /// Number of lines.
    pub fn height(&self) -> usize {
        self.lines.len()
    }
}

/// All available art variants.
pub const VARIANTS: &[AsciiArt] = &[
    AsciiArt {
        lines: &[
            r"01001100 01100001 01110010 01110000",
        ],
    },
    AsciiArt {
        lines: &[
            r"4C 61 72 70",
        ],
    },
    AsciiArt {
        lines: &[
            r".-.. .- .-. .--.",
        ],
    },
    AsciiArt {
        lines: &[
            r"#####   #   ####  #####",
            r" #  #  # #  #   # #   #",
            r" #  # ##### ####  #   #",
            r" #  # #   # #     #   #",
            r"#   # #   # #     #   #",
        ],
    },
    AsciiArt {
        lines: &[
            r"▗▖    ▗▄▖ ▗▄▄▖ ▗▄▄▖",
            r"▐▌   ▐▌ ▐▌▐▌ ▐▌▐▌ ▐▌",
            r"▐▌   ▐▛▀▜▌▐▛▀▚▖▐▛▀▘",
            r"▐▙▄▄▖▐▌ ▐▌▐▌ ▐▌▐▌",
        ],
    },
    AsciiArt {
        lines: &[
            r"██      ▄▄▄  ▄▄▄▄  ▄▄▄▄",
            r"██     ██▀██ ██▄█▄ ██▄█▀",
            r"██████ ██▀██ ██ ██ ██",
        ],
    },
    AsciiArt {
        lines: &[
            r".____",
            r"|    |   _____ _____________",
            r"|    |   \__  \\_  __ \____ \",
            r"|    |___ / __ \|  | \/  |_> >",
            r"|_______ (____  /__|  |   __/",
            r"        \/    \/      |__|",
        ],
    },
    AsciiArt {
        lines: &[
            r"   __",
            r"  / /  __ _ _ __ _ __",
            r" / /  / _` | '__| '_ \",
            r"/ /__| (_| | |  | |_) |",
            r"\____/\__,_|_|  | .__/",
            r"                |_|",
        ],
    },
    AsciiArt {
        lines: &[
            r" ___      _______  ______    _______",
            r"|   |    |   _   ||    _ |  |       |",
            r"|   |    |  |_|  ||   | ||  |    _  |",
            r"|   |    |       ||   |_||_ |   |_| |",
            r"|   |___ |       ||    __  ||    ___|",
            r"|       ||   _   ||   |  | ||   |",
            r"|_______||__| |__||___|  |_||___|",
        ],
    },
    AsciiArt {
        lines: &[
            r"dP",
            r"88",
            r"88        .d8888b. 88d888b. 88d888b.",
            r"88        88'  `88 88'  `88 88'  `88",
            r"88        88.  .88 88       88.  .88",
            r"88888888P `88888P8 dP       88Y888P'",
            r"                            88",
            r"                            dP",
        ],
    },
];

/// Horizontal spacing between grid cells.
pub const GRID_PITCH_X: i64 = 44;
/// Vertical spacing between grid cells.
pub const GRID_PITCH_Y: i64 = 9;

/// Deterministically hash a grid cell coordinate to a `usize`.
fn hash_coords(cell_x: i64, cell_y: i64) -> usize {
    let mut h = (cell_x as u64).wrapping_mul(0x9E3779B97F4A7C15);
    h ^= (cell_y as u64).wrapping_mul(0xBF58476D1CE4E5B9);
    h ^= h >> 30;
    h = h.wrapping_mul(0xBF58476D1CE4E5B9);
    h ^= h >> 27;
    h as usize
}

/// Select an art variant based on cell coordinates.
pub fn hash_variant(cell_x: i64, cell_y: i64) -> usize {
    hash_coords(cell_x, cell_y) % VARIANTS.len()
}

/// Return a random-looking offset within the allowed max bounds.
fn cell_offset(cell_x: i64, cell_y: i64, max_dx: i64, max_dy: i64) -> (i64, i64) {
    let h = hash_coords(cell_x.wrapping_mul(73), cell_y.wrapping_mul(131));
    let dx = if max_dx > 0 { (h as i64) % max_dx } else { 0 };
    let dy = if max_dy > 0 { ((h >> 8) as i64) % max_dy } else { 0 };
    (dx, dy)
}

/// Returns the character that should be drawn at the given world‑space coordinate.
/// This inspects the surrounding grid cells and selects the first non‑space character
/// from the overlapping art piece.
pub fn char_at(global_x: i64, global_y: i64) -> char {
    let base_cx = global_x.div_euclid(GRID_PITCH_X);
    let base_cy = global_y.div_euclid(GRID_PITCH_Y);

    for cy_offset in -1..=1 {
        for cx_offset in -1..=1 {
            let cell_x = base_cx + cx_offset;
            let cell_y = base_cy + cy_offset;

            let variant_idx = hash_variant(cell_x, cell_y);
            let art = &VARIANTS[variant_idx];
            let art_w = art.width() as i64;
            let art_h = art.height() as i64;

            let max_dx = (GRID_PITCH_X - art_w).max(0);
            let max_dy = (GRID_PITCH_Y - art_h).max(0);

            let (dx, dy) = cell_offset(cell_x, cell_y, max_dx, max_dy);
            let origin_x = cell_x * GRID_PITCH_X + dx;
            let origin_y = cell_y * GRID_PITCH_Y + dy;

            if global_x >= origin_x && global_y >= origin_y {
                let local_x = (global_x - origin_x) as usize;
                let local_y = (global_y - origin_y) as usize;

                if local_y < art.lines.len() {
                    let line = art.lines[local_y];
                    if local_x < line.chars().count() {
                        if let Some(c) = line.chars().nth(local_x) {
                            if c != ' ' {
                                return c;
                            }
                        }
                    }
                }
            }
        }
    }

    ' '
}