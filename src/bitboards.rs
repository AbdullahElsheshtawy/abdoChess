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

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for rank in 0..8 {
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
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e4 0 1";
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
        let mut i = 0;
        let piece_row: Vec<&str> = pieces.splitn(8, '/').collect();
        for (idx, _) in piece_row.iter().enumerate() {
            for piece in piece_row[idx].chars() {
                if piece.is_ascii_digit() {
                    i += piece.to_digit(10).unwrap();
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

                self.pieces[piece_type as usize] |= 1 << i;
                self.squares[i as usize] = Some(Piece {
                    r#type: piece_type,
                    color,
                });

                self.colors[color as usize] |= 1 << i;
                i += 1;
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
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
}

use num_traits::FromPrimitive;
impl Square {
    fn from_str(s: &str) -> Option<Square> {
        if s.len() != 2 {
            return None;
        }

        #[rustfmt::skip]
        let idx: usize = match s.to_uppercase().as_str() {
            "A8" => 0, "B8" => 1, "C8" => 2, "D8" => 3, "E8" => 4, "F8" => 5, "G8" => 6, "H8" => 7,
            "A7" => 8, "B7" => 9, "C7" => 10, "D7" => 11, "E7" => 12, "F7" => 13, "G7" => 14, "H7" => 15,
            "A6" => 16, "B6" => 17, "C6" => 18, "D6" => 19, "E6" => 20, "F6" => 21, "G6" => 22, "H6" => 23,
            "A5" => 24, "B5" => 25, "C5" => 26, "D5" => 27, "E5" => 28, "F5" => 29, "G5" => 30, "H5" => 31,
            "A4" => 32, "B4" => 33, "C4" => 34, "D4" => 35, "E4" => 36, "F4" => 37, "G4" => 38, "H4" => 39,
            "A3" => 40, "B3" => 41, "C3" => 42, "D3" => 43, "E3" => 44, "F3" => 45, "G3" => 46, "H3" => 47,
            "A2" => 48, "B2" => 49, "C2" => 50, "D2" => 51, "E2" => 52, "F2" => 53, "G2" => 54, "H2" => 55,
            "A1" => 56, "B1" => 57, "C1" => 58, "D1" => 59, "E1" => 60, "F1" => 61, "G1" => 62, "H1" => 63,
            _ => return None,
        };
        FromPrimitive::from_usize(idx)
    }
}
