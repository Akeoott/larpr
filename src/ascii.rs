// Copyright (c) Akeoot <akeoot@pm.me>. Licensed under the LGPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.

use std::sync::OnceLock;

/// A static ASCII art pattern stored as a slice of lines.
pub struct AsciiArt {
    pub lines: &'static [&'static str],
}

impl
AsciiArt {
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
            r"██      ▄▄▄  ▄▄▄▄  ▄▄▄▄",
            r"██     ██▀██ ██▄█▄ ██▄█▀",
            r"██████ ██▀██ ██ ██ ██",
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
            r"▀█▐▀    ▀█▐▀███ ▀█▐▀███ ▀█▐▀███",
            r" █▐      █▐▄███  █▐▄██▀  █▐ ███",
            r" █▐ ███  █▐ ███  █▐ ███  █▐▀▀▀▀",
            r"▀▀▀▀▀▀▀ ▀▀▀ ▀▀▀ ▀▀▀ ▀▀▀ ▀▀▀▀",
        ],
    },
    AsciiArt {
        lines: &[
            r"██╗      █████╗ ██████╗ ██████╗",
            r"██║     ██╔══██╗██╔══██╗██╔══██╗",
            r"██║     ███████║██████╔╝██████╔╝",
            r"██║     ██╔══██║██╔══██╗██╔═══╝",
            r"███████╗██║  ██║██║  ██║██║",
            r"╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝",
        ],
    },
    AsciiArt {
        lines: &[
            r"▄▄▄           ▄▄▄▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄▄",
            r"███           ▄▄▄▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄▄",
            r"███                     ███          ███          ███",
            r"███           ███       ███  ███▄▄▄▄▄███  ███▄▄▄▄▄███",
            r"███           ███▄▄▄▄▄▄▄███  ███▄▄▄▄▄     ███▄▄▄▄▄",
            r"███           ███▄▄▄▄▄▄▄███  ███ ███▌     ███",
            r"███ ▄▄▄▄▄▄▄▄  ███       ███  ███ ▐███     ███",
            r"███ ▄▄▄▄▄▄▄▄  ███       ███  ███  ███▌    ███",
        ],
    },
    AsciiArt {
        lines: &[
            r"  ██        ██▀▀▀██    ██▀▀▀██    ██▀▀▀██",
            r"█ ██ ██████ ██ █ ██ ██ ██ █ ██ ██ ██ █ ██ █",
            r"█ ██ ██████ ██ ▀ ██ ██ ██ █ ██ ██ ██ █ ██ █",
            r"█ ██ ██████ ███████ ██ ██ ▀ ██ ██ ██▄▄▄██ █",
            r"█ ██ ██████ ██ ▄ ██ ██ ██████  ██ ██ ▄▄▄▄▄█",
            r"█ ██ ██████ ██ █ ██ ██ ██ ▄ ██ ██ ██ ██████",
            r"█ ██ ██████ ██ █ ██ ██ ██ █ ██ ██ ██ ██████",
            r"█ ██ ██████ ██ █ ██ ██ ██ █ ██ ██ ██ ██████",
            r"▀ ██▄▄▄▄ ▀▀ ██ ▀ ██ ▀▀ ██ ▀ ██ ▀▀ ██ ▀▀▀▀▀▀",
        ],
    },
    AsciiArt {
        lines: &[
            r" ████",
            r"░░███",
            r" ░███   ██████   ████████  ████████",
            r" ░███  ░░░░░███ ░░███░░███░░███░░███",
            r" ░███   ███████  ░███ ░░░  ░███ ░███",
            r" ░███  ███░░███  ░███      ░███ ░███",
            r" █████░░████████ █████     ░███████",
            r"░░░░░  ░░░░░░░░ ░░░░░      ░███░░░",
            r"                           ░███",
            r"                           █████",
            r"                          ░░░░░",
        ],
    },
    AsciiArt {
        lines: &[
            r" ▄▄▄▄        ▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄",
            r"█████       █▄███████▄█ █████████▄█ █████████▄█",
            r"█▓▓▓█       █▓▓▓▓▓▓▓▓▓█ █▓▓▓▓▓▓▓▓▓█ █▓▓▓▓▓▓▓▓▓█",
            r"█▒▒▒█       █▒▒▒█▀█▒▒▒█ █▄▄▄█▀█▒▒▒█ █▄▄▄█▀█▒▒▒█",
            r"█░░░█       █░░░█▄█░░░█ █░░░█▄█░░░█ █░░░█▄█░░░█",
            r"█░░░█ ▄▄▄▄▄ █░░░░░░░░░█ █░░░▀▀▀░░▄█ █░░░░░░░░░█",
            r"█▒▒▒█▄█▒▒▒█ █▒▒▒▒▒▒▒▒▒█ █▒▒▒█ █▒▒▒█ █▒▒▒▒▒▒▒▒▀█",
            r"█▓▓▓▓▓▓▓▓▓█ █▓▓▓█▀█▓▓▓█ █▓▓▓█ █▓▓▓█ █▓▓▓█▀▀▀▀▀",
            r"███████████ █████ █████ █████ █████ █████",
            r"▀▀▀▀▀▀▀▀▀▀▀ ▀▀▀▀▀ ▀▀▀▀▀ ▀▀▀▀▀ ▀▀▀▀▀ ▀▀▀▀▀",
        ],
    },
];

/// Horizontal spacing between grid cells.
pub const GRID_PITCH_X: i64 = 55;
/// Vertical spacing between grid cells.
pub const GRID_PITCH_Y: i64 = 12;
/// Minimum gap between the bounding boxes of any two placed arts.
const MIN_GAP: i64 = 1;

static SEED: OnceLock<u64> = OnceLock::new();

/// Set the random seed for all placements. Must be called before any rendering.
pub fn set_seed(seed: u64) {
    let _ = SEED.set(seed);
}

/// Returns a 64‑bit deterministic hash for a pair of coordinates,
/// mixed with the global seed.
fn hash_pair(x: i64, y: i64) -> u64 {
    let seed = *SEED.get().unwrap_or(&0);
    let mut h = (x as u64).wrapping_mul(0x9E3779B97F4A7C15);
    h ^= (y as u64).wrapping_mul(0xBF58476D1CE4E5B9);
    h ^= seed;
    h ^= h >> 30;
    h = h.wrapping_mul(0xBF58476D1CE4E5B9);
    h ^= h >> 27;
    h.wrapping_mul(0x94D049BB133111EB)
}

/// Priority of a cell – higher value wins conflicts.
fn cell_priority(cell_x: i64, cell_y: i64) -> u64 {
    hash_pair(cell_x, cell_y)
}

/// Select an art variant for a cell
fn variant_for_cell(cell_x: i64, cell_y: i64) -> usize {
    (hash_pair(cell_x, cell_y) as usize) % VARIANTS.len()
}

/// Random offset within the cell (anywhere in [0, PITCH_X) × [0, PITCH_Y)).
fn random_offset_in_cell(cell_x: i64, cell_y: i64) -> (i64, i64) {
    let h = hash_pair(cell_x.wrapping_mul(73), cell_y.wrapping_mul(131));
    let dx = (h % GRID_PITCH_X as u64) as i64;
    let dy = ((h >> 8) % GRID_PITCH_Y as u64) as i64;
    (dx, dy)
}

/// Computes the origin (global coordinates) of the art placed in a given cell,
/// if that cell actually contains an art. Returns `Some((variant_idx, origin_x, origin_y))`
/// or `None` if the cell is empty (due to conflict resolution).
pub(crate) fn placed_art(cell_x: i64, cell_y: i64) -> Option<(usize, i64, i64)> {
    let variant = variant_for_cell(cell_x, cell_y);
    let art = &VARIANTS[variant];
    let art_w = art.width() as i64;
    let art_h = art.height() as i64;

    // Basic position within its own cell (anywhere inside)
    let (dx, dy) = random_offset_in_cell(cell_x, cell_y);
    let origin_x = cell_x * GRID_PITCH_X + dx;
    let origin_y = cell_y * GRID_PITCH_Y + dy;

    // Our bounding box, expanded by the minimum gap
    let gap = MIN_GAP;
    let x1 = origin_x - gap;
    let x2 = origin_x + art_w - 1 + gap;
    let y1 = origin_y - gap;
    let y2 = origin_y + art_h - 1 + gap;

    // Check all neighboring cells that could potentially overlap.
    // The maximum extent of any art + gap defines the required search radius.
    let max_art_w = VARIANTS.iter().map(|a| a.width() as i64).max().unwrap_or(0);
    let max_art_h = VARIANTS.iter().map(|a| a.height() as i64).max().unwrap_or(0);
    let radius_x = (max_art_w + 2 * gap) / GRID_PITCH_X + 2;   // +2 for safety
    let radius_y = (max_art_h + 2 * gap) / GRID_PITCH_Y + 2;

    let my_priority = cell_priority(cell_x, cell_y);

    for dy in -radius_y..=radius_y {
        for dx in -radius_x..=radius_x {
            if dx == 0 && dy == 0 { continue; }
            let nx = cell_x + dx;
            let ny = cell_y + dy;

            // Compute that neighbor's art (if any) recursively – but we avoid infinite loops.
            let n_variant = variant_for_cell(nx, ny);
            let n_art = &VARIANTS[n_variant];
            let n_w = n_art.width() as i64;
            let n_h = n_art.height() as i64;
            let (ndx, ndy) = random_offset_in_cell(nx, ny);
            let n_origin_x = nx * GRID_PITCH_X + ndx;
            let n_origin_y = ny * GRID_PITCH_Y + ndy;

            // Neighbor’s expanded bounding box
            let nx1 = n_origin_x - gap;
            let nx2 = n_origin_x + n_w - 1 + gap;
            let ny1 = n_origin_y - gap;
            let ny2 = n_origin_y + n_h - 1 + gap;

            // Check overlap (expanded boxes)
            if x1 <= nx2 && x2 >= nx1 && y1 <= ny2 && y2 >= ny1 {
                let neigh_priority = cell_priority(nx, ny);
                if neigh_priority > my_priority {
                    return None;
                }
                // If we have higher priority, the neighbor will be suppressed (when it checks us).
            }
        }
    }

    // No higher‑priority overlap found → we keep this placement.
    Some((variant, origin_x, origin_y))
}
