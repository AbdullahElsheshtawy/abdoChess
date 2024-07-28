pub struct Bitboard {
    pub squares: [Square; 64],
    pub pawn: u64,
    pub knight: u64,
    pub bishop: u64,
    pub rook: u64,
    pub queen: u64,
    pub king: u64,
    pub white: u64,
    pub black: u64,
}

#[derive(Clone, Copy)]
pub enum Square {
    Empty,
    Full(u64),
}

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bitboard = "".to_string();
        for (i, square) in self.squares.iter().enumerate() {
            match square {
                Square::Empty => bitboard.push('.'),
                Square::Full(bit) => {
                    bitboard.push(bitboard_to_piece(bit));
                    println!("{bit}");
                }
            }
            if (i + 1) % 8 == 0 {
                bitboard.push('\n');
            }
        }

        writeln!(f, "{}", bitboard)
    }
}

pub fn bitboard_to_piece(_bit: &u64) -> char {
    'a'
}
