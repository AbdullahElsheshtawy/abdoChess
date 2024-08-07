pub struct Board {
    pub squares: [Option<Piece>; 64],
    pub pieces: [u64; 6],
    pub colors: [u64; 2],
    pub active_color: Color,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<Square>,
    pub halfmove_clock: u16,
    pub fullmove_clock: u16,
}

pub fn print_bitboard(bitboard: &u64) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let index = rank * 8 + file;
            let bit = (bitboard >> index) & 1;
            print!("{} ", bit);
        }
        println!();
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let idx = rank * 8 + file;
                match &self.squares[idx] {
                    Some(piece) => {
                        let symbol = match piece.r#type {
                            PieceType::Pawn => 'P',
                            PieceType::Bishop => 'B',
                            PieceType::Knight => 'N',
                            PieceType::Rook => 'R',
                            PieceType::Queen => 'Q',
                            PieceType::King => 'K',
                        };
                        let symbol = if piece.color == Color::Black {
                            symbol.to_ascii_lowercase()
                        } else {
                            symbol
                        };
                        write!(f, "{} ", symbol)?;
                    }
                    None => write!(f, ". ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Default for Board {
    fn default() -> Board {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        Board::from_fen(fen).unwrap()
    }
}

impl Board {
    fn empty() -> Board {
        Board {
            squares: [None; 64],
            pieces: [0; 6],
            colors: [0; 2],
            active_color: Color::White,
            castling_rights: CastlingRights::NONE,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_clock: 1,
        }
    }
    fn from_fen(fen: &str) -> Result<Board, String> {
        let mut board = Board::empty();

        let part: Vec<&str> = fen.split(' ').collect();
        if part.len() != 6 {
            return Err("ERROR: Invalid FEN String".to_string());
        }

        board.parse_pieces(part[0]);
        board.parse_active_color(part[1]);
        board.parse_castling_availabilty(part[2]);
        board.parse_enpassant(part[3]);
        board.parse_halfmove_clock(part[4]);
        board.parse_fullmove_clock(part[5]);

        Ok(board)
    }

    fn parse_pieces(&mut self, pieces: &str) {
        // nbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR
        let piece_row: Vec<&str> = pieces.splitn(8, '/').collect();
        for (rank, row) in piece_row.iter().enumerate() {
            let mut file = 0;
            for piece in row.chars() {
                if piece.is_ascii_digit() {
                    file += piece.to_digit(10).unwrap();
                    continue;
                }
                let color = if piece.is_uppercase() {
                    Color::White
                } else {
                    Color::Black
                };
                let piece_type = match piece.to_ascii_lowercase() {
                    'p' => PieceType::Pawn,
                    'b' => PieceType::Bishop,
                    'n' => PieceType::Knight,
                    'r' => PieceType::Rook,
                    'q' => PieceType::Queen,
                    'k' => PieceType::King,
                    _ => continue, // Ignore invalid characters
                };

                let bit_index: usize = 63 - (rank * 8 + file as usize);

                self.pieces[piece_type as usize] |= 1 << bit_index;
                self.squares[bit_index] = Some(Piece {
                    r#type: piece_type,
                    color,
                });

                self.colors[color as usize] |= 1 << bit_index;
                file += 1;
            }
        }
    }

    fn parse_active_color(&mut self, active_color: &str) {
        match active_color {
            "w" => self.active_color = Color::White,
            "b" => self.active_color = Color::Black,
            _ => (),
        };
    }

    fn parse_castling_availabilty(&mut self, castling_availablity: &str) {
        if castling_availablity == "KQkq" {
            self.castling_rights = CastlingRights::ALL;
            return;
        } else if castling_availablity.is_empty() {
            self.castling_rights = CastlingRights::NONE;
            return;
        }
        for ch in castling_availablity.chars() {
            match ch {
                'K' => self.castling_rights |= CastlingRights::WHITEKINGSIDE,
                'Q' => self.castling_rights |= CastlingRights::WHITEQUEENSIDE,
                'k' => self.castling_rights |= CastlingRights::BLACKKINGSIDE,
                'q' => self.castling_rights |= CastlingRights::BLACKQUEENSIDE,
                _ => (),
            }
        }
    }
    fn parse_enpassant(&mut self, en_passant: &str) {
        self.en_passant = Square::from_str(en_passant);
    }
    fn parse_halfmove_clock(&mut self, halfmove_clock: &str) {
        self.halfmove_clock = halfmove_clock.parse().unwrap();
    }
    fn parse_fullmove_clock(&mut self, fullmove_clock: &str) {
        self.fullmove_clock = fullmove_clock.parse().unwrap();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub r#type: PieceType,
    pub color: Color,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

bitflags::bitflags! {
    pub struct CastlingRights: u8 {
        const NONE = 0;
        const WHITEKINGSIDE = 1 << 0;
        const WHITEQUEENSIDE = 1 << 1;
        const BLACKKINGSIDE = 1 << 2;
        const BLACKQUEENSIDE = 1 << 3;
        const ALL =
            Self::WHITEKINGSIDE.bits()
            | Self::WHITEQUEENSIDE.bits()
            | Self::BLACKKINGSIDE.bits()
            | Self::BLACKQUEENSIDE.bits();
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, num_derive::FromPrimitive)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

use num_traits::FromPrimitive;

impl Square {
    fn from_str(s: &str) -> Option<Square> {
        if s.len() != 2 {
            return None;
        }

        #[rustfmt::skip]
        let idx: usize = match s.to_lowercase().as_str() {
            "a1" => 0, "b1" => 1, "c1" => 2, "d1" => 3, "e1" => 4, "f1" => 5, "g1" => 6, "h1" => 7,
            "a2" => 8, "b2" => 9, "c2" => 10, "d2" => 11, "e2" => 12, "f2" => 13, "g2" => 14, "h2" => 15,
            "a3" => 16, "b3" => 17, "c3" => 18, "d3" => 19, "e3" => 20, "f3" => 21, "g3" => 22, "h3" => 23,
            "a4" => 24, "b4" => 25, "c4" => 26, "d4" => 27, "e4" => 28, "f4" => 29, "g4" => 30, "h4" => 31,
            "a5" => 32, "b5" => 33, "c5" => 34, "d5" => 35, "e5" => 36, "f5" => 37, "g5" => 38, "h5" => 39,
            "a6" => 40, "b6" => 41, "c6" => 42, "d6" => 43, "e6" => 44, "f6" => 45, "g6" => 46, "h6" => 47,
            "a7" => 48, "b7" => 49, "c7" => 50, "d7" => 51, "e7" => 52, "f7" => 53, "g7" => 54, "h7" => 55,
            "a8" => 56, "b8" => 57, "c8" => 58, "d8" => 59, "e8" => 60, "f8" => 61, "g8" => 62, "h8" => 63,
            _ => return None,
        };
        FromPrimitive::from_usize(idx)
    }
}
