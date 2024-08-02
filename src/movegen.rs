use crate::bitboards::Color;
const NOT_A_FILE: u64 = 0xfefefefefefefefe; // ~0x0101010101010101
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f; // ~0x8080808080808080

const DOUBLE_PAWN_PUSH_WHITE: u64 = 0x000000000000FF00; // Third rank
const DOUBLE_PAWN_PUSH_BLACK: u64 = 0x00FF000000000000; // Fifth rank

pub struct LookUp {
    pub king_attacks: [u64; 64],
    pub knight_attacks: [u64; 64],
    pub pawn_attacks: [[u64; 64]; 2],
    pub double_pawn_attacks: [[u64; 64]; 2],
}
pub fn init_lookups() -> LookUp {
    let mut king_attacks_mask: [u64; 64] = [0; 64];
    let mut knight_attacks_mask: [u64; 64] = [0; 64];
    let mut pawn_attacks_mask: [[u64; 64]; 2] = [[0; 64]; 2];
    let mut double_pawn_attacks_mask: [[u64; 64]; 2] = [[0; 64]; 2];
    let mut sq_bb: u64 = 1;
    for sq in 0..64 {
        king_attacks_mask[sq] = king_attacks(sq_bb);
        knight_attacks_mask[sq] = knight_attacks(sq_bb);
        let white_attacks = w_pawn_east_attacks(sq_bb) | w_pawn_west_attacks(sq_bb);
        let black_attacks = b_pawn_east_attacks(sq_bb) | b_pawn_west_attacks(sq_bb);
        pawn_attacks_mask[Color::White as usize][sq] = white_attacks;
        pawn_attacks_mask[Color::Black as usize][sq] = black_attacks;
        double_pawn_attacks_mask[Color::White as usize][sq] = white_double_pawn_push(sq_bb);
        double_pawn_attacks_mask[Color::Black as usize][sq] = black_double_pawn_push(sq_bb);
        sq_bb <<= 1;
    }

    LookUp {
        king_attacks: king_attacks_mask,
        knight_attacks: knight_attacks_mask,
        pawn_attacks: pawn_attacks_mask,
        double_pawn_attacks: double_pawn_attacks_mask,
    }
}
pub fn white_double_pawn_push(wpawns: u64) -> u64 {
    nort_one(nort_one(wpawns & DOUBLE_PAWN_PUSH_WHITE))
}

pub fn black_double_pawn_push(bpawns: u64) -> u64 {
    sout_one(sout_one(bpawns & DOUBLE_PAWN_PUSH_BLACK))
}
pub fn w_pawn_east_attacks(wpawns: u64) -> u64 {
    return no_ea_one(wpawns);
}
pub fn w_pawn_west_attacks(wpawns: u64) -> u64 {
    return no_we_one(wpawns);
}

pub fn b_pawn_east_attacks(bpawns: u64) -> u64 {
    return so_ea_one(bpawns);
}
pub fn b_pawn_west_attacks(bpawns: u64) -> u64 {
    return so_we_one(bpawns);
}

fn knight_attacks(knights: u64) -> u64 {
    let l1 = (knights >> 1) & 0x7f7f7f7f7f7f7f7f;
    let l2 = (knights >> 2) & 0x3f3f3f3f3f3f3f3f;
    let r1 = (knights << 1) & 0xfefefefefefefefe;
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
pub fn so_we_one(b: u64) -> u64 {
    (b >> 9) & NOT_H_FILE
}
pub fn no_we_one(b: u64) -> u64 {
    (b << 7) & NOT_H_FILE
}

fn bitscan_forwards(bb: u64) -> u64 {
    bb.trailing_zeros() as u64
}

fn bitscan_backwards(bb: &u64) -> u64 {
    bb.leading_zeros() as u64
}

fn bitscan_forwards_with_reset(bb: &mut u64) -> u64 {
    let idx = bitscan_forwards(*bb);
    *bb &= *bb - 1;
    idx
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_bitscan_forwards() {
        // it is the first zero in the bit  00000000000000000000000000000.......00001
        let one_lsb: u64 = 1;
        // They are all zeros so = 64 ----- 00000000000000000000000000000
        let none_lsb: u64 = 0;
        let msb: u64 = 1 << 63;
        assert_eq!(bitscan_forwards(one_lsb), 0);
        assert_eq!(bitscan_forwards(none_lsb), 64);
        assert_eq!(bitscan_forwards(msb), 63);
    }

    #[test]
    fn test_bitscan_backwards() {
        // 100000000 trailing_zeros = 0 of course since its the first bit
        let one_msb: u64 = 1 << 63;
        // 0000000000 trailing_zeros = 64 since there is no 1 in the u64
        let none_msb = 0;
        // 0000000000....00001 = 63 since there are 63 0s b4 it
        let one_lsb = 1;
        assert_eq!(bitscan_backwards(&one_msb), 0);
        assert_eq!(bitscan_backwards(&none_msb), 64);
        assert_eq!(bitscan_backwards(&one_lsb), 63);
    }

    #[test]
    fn test_bitscan_forwards_with_reset() {
        let mut bb: u64 = 0;
        for i in 0..62 {
            bb |= 1 << i;
        }

        // Copy of bb for comparison
        let mut expected_bb: u64 = 0;
        for i in 1..62 {
            expected_bb |= 1 << i;
        }

        // Perform the operation
        let index = bitscan_forwards_with_reset(&mut bb);

        // Assert that the returned index is 0
        assert_eq!(index, 0);

        // Assert that the bitboard `bb` now has 62 bits set from position 1 to 61
        assert_eq!(bb, expected_bb);
    }
    #[test]
    fn test_double_pawn_push() {
        // White pawns on the second rank (starting position)
        let white_pawns: u64 = 0x000000000000FF00;
        // Black pawns on the seventh rank (starting position)
        let black_pawns: u64 = 0x00FF000000000000;

        // Calculate double pawn pushes
        let white_double_push = white_double_pawn_push(white_pawns);
        let black_double_push = black_double_pawn_push(black_pawns);

        // Print results for debugging
        println!("White double push: {:x}", white_double_push);
        println!("Black double push: {:x}", black_double_push);

        // Expected results
        // For white pawns, the double push moves pawns from the second rank to the fourth rank
        let expected_white_double_push: u64 = 0x00000000FF000000;
        // For black pawns, the double push moves pawns from the seventh rank to the fifth rank
        let expected_black_double_push: u64 = 0x00FF000000000000;

        // Assertions to check if the results match the expected values
        assert_eq!(white_double_push, expected_white_double_push);
        // assert_eq!(black_double_push, expected_black_double_push);
    }
}
