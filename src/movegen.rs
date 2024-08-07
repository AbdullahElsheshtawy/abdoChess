use num_traits::PrimInt;
use rand::{Rng, SeedableRng};

use crate::bitboards::Color;
const NOT_A_FILE: u64 = 0xfefefefefefefefe; // ~0x0101010101010101
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f; // ~0x8080808080808080

#[rustfmt::skip]
pub const ROOK_BITS: [u64; 64] = [
  12, 11, 11, 11, 11, 11, 11, 12,
  11, 10, 10, 10, 10, 10, 10, 11,
  11, 10, 10, 10, 10, 10, 10, 11,
  11, 10, 10, 10, 10, 10, 10, 11,
  11, 10, 10, 10, 10, 10, 10, 11,
  11, 10, 10, 10, 10, 10, 10, 11,
  11, 10, 10, 10, 10, 10, 10, 11,
  12, 11, 11, 11, 11, 11, 11, 12
];

#[rustfmt::skip]
pub const BISHOP_BITS: [u64; 64] = [  
  6, 5, 5, 5, 5, 5, 5, 6,
  5, 5, 5, 5, 5, 5, 5, 5,
  5, 5, 7, 7, 7, 7, 5, 5,
  5, 5, 7, 9, 9, 7, 5, 5,
  5, 5, 7, 9, 9, 7, 5, 5,
  5, 5, 7, 7, 7, 7, 5, 5,
  5, 5, 5, 5, 5, 5, 5, 5,
  6, 5, 5, 5, 5, 5, 5, 6
];

pub struct LookUp {
    pub king_attacks: [u64; 64],
    pub knight_attacks: [u64; 64],
    pub pawn_attacks: [[u64; 64]; 2],
    pub bishop_attacks: [[u64; 512]; 64],
    pub rook_attacks: [[u64; 4096]; 64],
}

impl LookUp {
    pub fn init() -> LookUp {
        let magics = MagicNumbers::init();
        let mut king_attacks_mask: [u64; 64] = [0; 64];
        let mut knight_attacks_mask: [u64; 64] = [0; 64];
        let mut pawn_attacks_mask: [[u64; 64]; 2] = [[0; 64]; 2];
        let mut bishop_attacks_mask: [[u64; 512]; 64] = [[0u64; 512]; 64];
        let mut rook_attacks_mask: [[u64; 4096]; 64] = [[0u64; 4096]; 64];

        let mut sq_bb: u64 = 1;
        for sq in 0..64 {
            king_attacks_mask[sq] = king_attacks(sq_bb);
            knight_attacks_mask[sq] = knight_attacks(sq_bb);
            let white_attacks = w_pawn_east_attacks(sq_bb) | w_pawn_west_attacks(sq_bb);
            let black_attacks = b_pawn_east_attacks(sq_bb) | b_pawn_west_attacks(sq_bb);
            pawn_attacks_mask[Color::White as usize][sq] = white_attacks;
            pawn_attacks_mask[Color::Black as usize][sq] = black_attacks;

            let occupancy_masks = generate_bishop_occupancy_masks(sq as u64); // Generate bishop occupancy masks
            for &occupancy in &occupancy_masks {
                let index = transform(occupancy, magics.bishop_magics[sq], BISHOP_BITS[sq]);
                bishop_attacks_mask[sq][index as usize] = bishop_attacks(sq as u64, occupancy);
            }

            let occupancy_masks = generate_rook_occupancy_masks(sq as u64); // Generate rook occupancy masks
            for &occupancy in &occupancy_masks {
                let index = transform(occupancy, magics.rook_magics[sq], ROOK_BITS[sq]);
                rook_attacks_mask[sq][index as usize] = rook_attacks(sq as u64, occupancy);
            }
            sq_bb <<= 1;
        }

        LookUp {
            king_attacks: king_attacks_mask,
            knight_attacks: knight_attacks_mask,
            pawn_attacks: pawn_attacks_mask,
            bishop_attacks: bishop_attacks_mask,
            rook_attacks: rook_attacks_mask,
        }
    }
}
use rand_xoshiro::Xoshiro256PlusPlus;
pub struct MagicNumbers {
    bishop_magics: [u64; 64],
    rook_magics: [u64; 64],
}

impl MagicNumbers {
    pub fn init() -> MagicNumbers {
        let mut bishop_magics = [0u64; 64];
        let mut rook_magics = [0u64; 64];

        for sq in 0..64 {
            let sq_bb = 1u64 << sq;
            bishop_magics[sq] = find_magic(sq_bb, BISHOP_BITS[sq], true);
            rook_magics[sq] = find_magic(sq_bb, ROOK_BITS[sq], false);
        }

        MagicNumbers {
            bishop_magics,
            rook_magics,
        }
    }
}

fn generate_bishop_occupancy_masks(square: u64) -> Vec<u64> {
    let mut mask = 0u64;
    let rank = square / 8;
    let file = square % 8;

    // Diagonals: up-right, up-left, down-right, down-left
    // up-right
    for (r, f) in (rank + 1..8).zip(file + 1..8) {
        mask |= 1u64 << (f + r * 8);
    }
    // up-left
    for (r, f) in (rank + 1..8).zip((0..file).rev()) {
        mask |= 1u64 << (f + r * 8);
    }
    // down-right
    for (r, f) in (0..rank).rev().zip(file + 1..8) {
        mask |= 1u64 << (f + r * 8);
    }
    // down-left
    for (r, f) in (0..rank).rev().zip((0..file).rev()) {
        mask |= 1u64 << (f + r * 8);
    }

    let mut occupancy_masks = vec![];
    let bit_count = mask.count_ones() as usize;
    let combinations = 1 << bit_count;

    for i in 0..combinations {
        let mut occupancy = 0u64;
        for j in 0..bit_count {
            if i & (1 << j) != 0 {
                occupancy |= 1u64 << (mask.trailing_zeros() as u64 + j as u64);
            }
        }
        occupancy_masks.push(occupancy);
    }

    occupancy_masks
}
fn generate_rook_occupancy_masks(square: u64) -> Vec<u64> {
    let mut mask = 0u64;
    let rank = square / 8;
    let file = square % 8;

    // Horizontal and vertical lines
    // Horizontal
    for f in (0..file).rev() {
        mask |= 1u64 << (f + rank * 8);
    }
    for f in file + 1..8 {
        mask |= 1u64 << (f + rank * 8);
    }
    // Vertical
    for r in (0..rank).rev() {
        mask |= 1u64 << (file + r * 8);
    }
    for r in rank + 1..8 {
        mask |= 1u64 << (file + r * 8);
    }

    let mut occupancy_masks = vec![];
    let bit_count = mask.count_ones() as usize;
    let combinations = 1 << bit_count;

    for i in 0..combinations {
        let mut occupancy = 0u64;
        for j in 0..bit_count {
            if i & (1 << j) != 0 {
                occupancy |= 1u64 << (mask.trailing_zeros() as u64 + j as u64);
            }
        }
        occupancy_masks.push(occupancy);
    }

    occupancy_masks
}

pub fn find_magic(square: u64, mask_bits: u64, is_bishop: bool) -> u64 {
    let mask: u64 = if is_bishop {
        bishop_mask(square)
    } else {
        rook_mask(square)
    };
    let occupancy_count = mask.count_ones() as u64;
    let occupation_indices = 1 << occupancy_count;
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(square);

    // Iterate with a larger number of attempts for more robust results
    for attempt in 0..1_000_000_000_000_i64 {
        let magic = random_magic_number(&mut rng);
        // Ensure the magic number has enough leading zero bits
        if (mask.wrapping_mul(magic) & 0xFF00000000000000).count_ones() < 6 {
            continue;
        }

        let mut used_attacks = vec![0u64; 1 << mask_bits];
        let mut fail = false;

        for i in 0..occupation_indices {
            let occupancy = index_to_u64(i, occupancy_count, mask);
            let attacks = if is_bishop {
                bishop_attacks(square, occupancy)
            } else {
                rook_attacks(square, occupancy)
            };
            let index = transform(occupancy, magic, mask_bits) as usize;

            if used_attacks[index] == 0 {
                used_attacks[index] = attacks;
            } else if used_attacks[index] != attacks {
                fail = true;
                break;
            }
        }

        if !fail {
            return magic;
        }
    }

    eprintln!("MAGIC NUMBER FAILED for square {}", square.trailing_zeros());
    0
}

fn random_magic_number(rng: &mut Xoshiro256PlusPlus) -> u64 {
    let u1 = rng.gen::<u64>() & 0xFFFF;
    let u2 = rng.gen::<u64>() & 0xFFFF;
    let u3 = rng.gen::<u64>() & 0xFFFF;
    let u4 = rng.gen::<u64>() & 0xFFFF;
    u1 | (u2 << 16) | (u3 << 32) | (u4 << 48)
}

#[inline(always)]
pub fn transform(occupancy: u64, magic: u64, bits: u64) -> u64 {
    (occupancy.wrapping_mul(magic)) >> (64 - bits)
}

pub fn index_to_u64(index: u64, bits: u64, mask: u64) -> u64 {
    let mut result = 0;
    let mut bit = 1;
    for i in 0..bits {
        if index & (1 << i) != 0 {
            while mask & bit == 0 {
                bit <<= 1;
            }
            result |= bit;
        }
        bit <<= 1;
    }
    result
}
pub fn rook_mask(square: u64) -> u64 {
    let mut attacks: u64 = 0;
    let rk = (square.trailing_zeros() / 8) as u64;
    let fl = (square.trailing_zeros() % 8) as u64;

    // North Movement
    let mut r = rk + 1;
    while r <= 6 {
        attacks |= 1u64 << (fl + r * 8);
        r += 1;
    }

    // South Movement
    let mut r = rk as isize - 1;
    while r >= 1 {
        attacks |= 1u64 << (fl + r as u64 * 8);
        r -= 1;
    }

    // East Movement
    let mut f = fl + 1;
    while f <= 6 {
        attacks |= 1u64 << (f + rk * 8);
        f += 1;
    }

    // West Movement
    let mut f = fl as isize - 1;
    while f >= 1 {
        attacks |= 1u64 << (f as u64 + rk * 8);
        f -= 1;
    }
    attacks
}

pub fn random_u64() -> u64 {
    use rand::random;

    let u1 = random::<u64>() & 0xFFFF;
    let u2 = random::<u64>() & 0xFFFF;
    let u3 = random::<u64>() & 0xFFFF;
    let u4 = random::<u64>() & 0xFFFF;

    u1 | (u2 << 16) | (u3 << 32) | (u4 << 48)
}

pub fn random_u64_few_bits() -> u64 {
    random_u64() & random_u64() & random_u64()
}
pub fn rook_attacks(square: u64, block: u64) -> u64 {
    let mut result: u64 = 0;
    let sq_index = square.trailing_zeros() as usize;
    let rank = sq_index / 8;
    let file = sq_index % 8;

    // Vertical moves - Up
    for r in (rank + 1)..8 {
        let pos = file + r * 8;
        if pos >= 64 {
            break;
        } // Ensure pos is within bounds
        result |= 1u64 << pos;
        if block & (1u64 << pos) != 0 {
            break;
        }
    }

    // Vertical moves - Down
    for r in (0..rank).rev() {
        let pos = file + r * 8;
        if pos >= 64 {
            break;
        } // Ensure pos is within bounds
        result |= 1u64 << pos;
        if block & (1u64 << pos) != 0 {
            break;
        }
    }

    // Horizontal moves - Right
    for f in (file + 1)..8 {
        let pos = f + rank * 8;
        if pos >= 64 {
            break;
        } // Ensure pos is within bounds
        result |= 1u64 << pos;
        if block & (1u64 << pos) != 0 {
            break;
        }
    }

    // Horizontal moves - Left
    for f in (0..file).rev() {
        let pos = f + rank * 8;
        if pos >= 64 {
            break;
        } // Ensure pos is within bounds
        result |= 1u64 << pos;
        if block & (1u64 << pos) != 0 {
            break;
        }
    }

    result
}

pub fn bishop_mask(square: u64) -> u64 {
    let mut attacks: u64 = 0;
    let rk = (square.trailing_zeros() / 8) as u64;
    let fl = (square.trailing_zeros() % 8) as u64;

    // North East Movement
    let mut r: isize = rk as isize + 1;
    let mut f: isize = fl as isize + 1;
    while r <= 6 && f <= 6 {
        attacks |= 1u64 << (f as u64 + r as u64 * 8);
        r += 1;
        f += 1;
    }

    // North West Movement
    r = rk as isize + 1;
    f = fl as isize - 1;
    while r <= 6 && f >= 1 {
        attacks |= 1u64 << (f as u64 + r as u64 * 8);
        r += 1;
        f -= 1;
    }

    // South East Movement
    r = rk as isize - 1;
    f = fl as isize + 1;
    for (r, f) in (1..rk).rev().zip(fl + 1..7) {
        attacks |= 1u64 << (f + r * 8);
    }
    // South West Movement
    r = rk as isize - 1;
    f = fl as isize - 1;
    while r >= 1 && f >= 1 {
        attacks |= 1u64 << (f as u64 + r as u64 * 8);
        r -= 1;
        f -= 1;
    }
    attacks
}
pub fn bishop_attacks(square: u64, block: u64) -> u64 {
    let mut attacks: u64 = 0;
    let rank = square.trailing_zeros() / 8;
    let file = square.trailing_zeros() % 8;

    // Diagonal moves - Up-right
    for (r, f) in (rank + 1..8).zip(file + 1..8) {
        let pos = f + r * 8;
        attacks |= 1u64 << pos;
        if block & (1u64 << pos) != 0 {
            break;
        }
    }

    // Diagonal moves - Up-left
    for (r, f) in (rank + 1..8).zip((0..file).rev()) {
        let pos = f + r * 8;
        attacks |= 1u64 << pos;
        if block & (1u64 << pos) != 0 {
            break;
        }
    }

    // Diagonal moves - Down-right
    for (r, f) in (0..rank).rev().zip(file + 1..8) {
        let pos = f + r * 8;
        attacks |= 1u64 << pos;
        if block & (1u64 << pos) != 0 {
            break;
        }
    }

    // Diagonal moves - Down-left
    for (r, f) in (0..rank).rev().zip((0..file).rev()) {
        let pos = f + r * 8;
        attacks |= 1u64 << pos;
        if block & (1u64 << pos) != 0 {
            break;
        }
    }

    attacks
}
pub fn w_pawn_east_attacks(wpawns: u64) -> u64 {
    no_ea_one(wpawns)
}
pub fn w_pawn_west_attacks(wpawns: u64) -> u64 {
    no_we_one(wpawns)
}

pub fn b_pawn_east_attacks(bpawns: u64) -> u64 {
    so_ea_one(bpawns)
}
pub fn b_pawn_west_attacks(bpawns: u64) -> u64 {
    so_we_one(bpawns)
}

fn knight_attacks(knights: u64) -> u64 {
    let l1 = (knights >> 1) & NOT_H_FILE;
    let l2 = (knights >> 2) & 0x3f3f3f3f3f3f3f3f;
    let r1 = (knights << 1) & NOT_A_FILE;
    let r2 = (knights << 2) & 0xfcfcfcfcfcfcfcfc;
    let h1 = l1 | r1;
    let h2 = l2 | r2;
    (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8)
}
pub fn king_attacks(kingset: u64) -> u64 {
    east_one(kingset)
        | west_one(kingset)
        | nort_one(kingset)
        | sout_one(kingset)
        | no_ea_one(kingset)
        | no_we_one(kingset)
        | so_ea_one(kingset)
        | so_we_one(kingset)
}
pub fn sout_one(b: u64) -> u64 {
    b >> 8
}
pub fn nort_one(b: u64) -> u64 {
    b << 8
}
pub fn east_one(b: u64) -> u64 {
    (b << 1) & NOT_A_FILE
}
pub fn no_ea_one(b: u64) -> u64 {
    (b << 9) & NOT_A_FILE
}
pub fn so_ea_one(b: u64) -> u64 {
    (b >> 7) & NOT_A_FILE
}
pub fn west_one(b: u64) -> u64 {
    (b >> 1) & NOT_H_FILE
}
pub fn count_1s(bb: u64) -> u64 {
    bb.count_ones() as u64
}
pub fn so_we_one(b: u64) -> u64 {
    (b >> 9) & NOT_H_FILE
}
pub fn no_we_one(b: u64) -> u64 {
    (b << 7) & NOT_H_FILE
}

pub fn bitscan_forwards(bb: u64) -> u64 {
    bb.trailing_zeros() as u64
}

pub fn bitscan_backwards(bb: &u64) -> u64 {
    bb.leading_zeros() as u64
}

pub fn bitscan_forwards_with_reset(bb: u64) -> u64 {
    bb & (bb - 1)
}
