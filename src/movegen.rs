const FILE_A: u64 = 0x0101010101010101;
const FILE_H: u64 = 0x8080808080808080;
pub fn white_single_pawn_push(pawns: u64, empty: u64) -> u64 {
    (pawns >> 8) & empty
}

pub fn white_double_pawn_push(pawns: u64, empty: u64) -> u64 {
    let RANK4 = 0x000000FF00000000;
    let pushed_one = white_single_pawn_push(pawns, empty);
    white_single_pawn_push(pushed_one, empty) & RANK4
}

pub fn black_single_pawn_push(pawns: u64, empty: u64) -> u64 {
    (pawns << 8) & empty
}

pub fn black_double_pawn_push(pawns: u64, empty: u64) -> u64 {
    let pushed_one = black_single_pawn_push(pawns, empty);
    black_single_pawn_push(pushed_one, empty)
}

pub fn pawn_attacks(pawns: u64, opponents: u64) -> u64 {
    ((pawns >> 7) & !FILE_A) & opponents | ((pawns >> 9) & !FILE_H) & opponents
}

pub fn pawn_attacks_black(pawns: u64, opponents: u64) -> u64 {
    ((pawns << 7) & !FILE_H) & opponents | ((pawns << 9) & !FILE_A) & opponents
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
        assert_eq!(bitscan_forwards(one_lsb), 0);
        assert_eq!(bitscan_forwards(none_lsb), 64)
    }

    #[test]
    fn test_bitscan_backwards() {
        // 100000000 trailing_zeros = 0 of course since its the first bit
        let one_msb: u64 = 1 << 63;
        // 0000000000 trailing_zeros = 64 since there is no 1 in the u64
        let none_msb = 0;
        // 0000000000....00001 = 63 since there are 63 0s b4 it
        let one_lsb = 1;
        println!("{one_msb} AND {}", bitscan_backwards(&one_msb));
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
}
