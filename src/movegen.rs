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
}

impl LookUp {
    pub fn init() -> LookUp {
        let mut king_attacks_mask: [u64; 64] = [0; 64];
        let mut knight_attacks_mask: [u64; 64] = [0; 64];
        let mut pawn_attacks_mask: [[u64; 64]; 2] = [[0; 64]; 2];
        let mut sq_bb: u64 = 1;
        for sq in 0..64 {
            king_attacks_mask[sq] = king_attacks(sq_bb);
            knight_attacks_mask[sq] = knight_attacks(sq_bb);
            let white_attacks = w_pawn_east_attacks(sq_bb) | w_pawn_west_attacks(sq_bb);
            let black_attacks = b_pawn_east_attacks(sq_bb) | b_pawn_west_attacks(sq_bb);
            pawn_attacks_mask[Color::White as usize][sq] = white_attacks;
            pawn_attacks_mask[Color::Black as usize][sq] = black_attacks;
            sq_bb <<= 1;
        }

        LookUp {
            king_attacks: king_attacks_mask,
            knight_attacks: knight_attacks_mask,
            pawn_attacks: pawn_attacks_mask,
        }
    }
}

pub fn find_magic(square: u64, mask_bits: u64, is_bishop: bool) -> u64 {
    let mut occupancy_variations = [0u64; 4096];
    let mut attack_masks = [0u64; 4096];
    let mut used_attacks = [0u64; 4096];
    let mut magic: u64;
    let mut fail: bool;

    let mask: u64 = if is_bishop {
        bishop_mask(square)
    } else {
        rook_mask(square)
    };
    let occupancy_count = count_1s(mask);

    for i in 0..(1 << occupancy_count) {
        occupancy_variations[i] = index_to_u64(i as u64, occupancy_count, mask);
        attack_masks[i] = if is_bishop {
            bishop_attacks(square, occupancy_variations[i])
        } else {
            rook_attacks(square, occupancy_variations[i])
        };
    }

    for _ in 0..1_000_000_000 {
        magic = random_u64_few_bits();

        if count_1s(mask.wrapping_mul(magic) & 0xFF00000000000000) < 6 {
            continue;
        }

        used_attacks.iter_mut().for_each(|x| *x = 0);

        fail = false;
        for i in 0..(1 << occupancy_count) {
            let transformed_index = transform(occupancy_variations[i], magic, mask_bits) as usize;
            if used_attacks[transformed_index] == 0 {
                used_attacks[i] = attack_masks[i];
            } else if used_attacks[transformed_index] != attack_masks[i] {
                fail = true;
                break;
            }
        }

        if !fail {
            return magic;
        }
    }

    eprintln!("MAGIC NUMBER FAILLEED");
    0
}

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
    let rk = square / 8;
    let fl = square % 8;

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
    let rank = square / 8;
    let file = square % 8;

    // Vertical moves - Up
    for r in (rank + 1)..8 {
        let pos = file + r * 8;
        result |= 1u64 << pos;
        if block & (1u64 << pos) != 0 {
            break;
        }
    }

    // Vertical moves - Down
    for r in (0..rank).rev() {
        let pos = file + r * 8;
        result |= 1u64 << pos;
        if block & (1u64 << pos) != 0 {
            break;
        }
    }

    // Horizontal moves - Right
    for f in (file + 1)..8 {
        let pos = f + rank * 8;
        result |= 1u64 << pos;
        if block & (1u64 << pos) != 0 {
            break;
        }
    }

    // Horizontal moves - Left
    for f in (0..file).rev() {
        let pos = f + rank * 8;
        result |= 1u64 << pos;
        if block & (1u64 << pos) != 0 {
            break;
        }
    }

    result
}
pub fn bishop_mask(square: u64) -> u64 {
    let mut attacks: u64 = 0;
    let rk = square / 8;
    let fl = square % 8;

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
    while r >= 1 && f <= 6 {
        attacks |= 1u64 << ((f as u64 + r as u64) * 8);
        r -= 1;
        f += 1;
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
    let rank = square / 8;
    let file = square % 8;

    // Diagonal moves - Up-right
    let mut r = rank + 1;
    let mut f = file + 1;
    while r < 8 && f < 8 {
        let pos = f + r * 8;
        attacks |= 1u64 << pos;
        if block & (1u64 << pos) != 0 {
            break;
        }
        r += 1;
        f += 1;
    }

    // Diagonal moves - Up-left
    let mut r = rank + 1;
    let mut f = file as isize - 1;
    while r < 8 && f >= 0 {
        let pos = f as u64 + r * 8;
        attacks |= 1u64 << pos;
        if block & (1u64 << pos) != 0 {
            break;
        }
        r += 1;
        f -= 1;
    }

    // Diagonal moves - Down-right
    let mut r = rank as isize - 1;
    let mut f = file + 1;
    while r >= 0 && f < 8 {
        let pos = f + r as u64 * 8;
        attacks |= 1u64 << pos;
        if block & (1u64 << pos) != 0 {
            break;
        }
        r -= 1;
        f += 1;
    }

    // Diagonal moves - Down-left
    let mut r = rank as isize - 1;
    let mut f = file as isize - 1;
    while r >= 0 && f >= 0 {
        let pos = f as u64 + r as u64 * 8;
        attacks |= 1u64 << pos;
        if block & (1u64 << pos) != 0 {
            break;
        }
        r -= 1;
        f -= 1;
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
