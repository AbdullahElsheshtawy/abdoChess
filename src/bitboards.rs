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

impl Default for Board {
    fn default() -> Board {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e4 0 1";
        let board = Board::from_fen(fen).unwrap();
        board
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
        for idx in 0..8 {
            for piece in piece_row[idx].chars() {
                if piece.is_digit(10) {
                    i += piece.to_digit(10).unwrap();
                    continue;
                }
                match piece.to_ascii_lowercase() {
                    'p' => {
                        self.pieces[PieceType::Pawn as usize] |= 1 << i;
                    }
                    'b' => {
                        self.pieces[PieceType::Bishop as usize] |= 1 << i;
                    }
                    'n' => {
                        self.pieces[PieceType::Knight as usize] |= 1 << i;
                    }
                    'r' => {
                        self.pieces[PieceType::Rook as usize] |= 1 << i;
                    }
                    'q' => {
                        self.pieces[PieceType::Queen as usize] |= 1 << i;
                    }
                    'k' => {
                        self.pieces[PieceType::King as usize] |= 1 << i;
                    }
                    _ => (),
                }
                if piece.is_uppercase() {
                    self.colors[Color::White as usize] |= 1 << i;
                } else {
                    self.colors[Color::Black as usize] |= 1 << i;
                }

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
        } else if castling_availablity == "" {
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
        self.en_passant = match en_passant {
            "-" => None,
            _ => en_passant.to_uppercase().parse().ok(),
        };
    }
    fn parse_halfmove_clock(&mut self, halfmove_clock: &str) {
        self.halfmove_clock = halfmove_clock.parse().unwrap();
    }
    fn parse_fullmove_clock(&mut self, fullmove_clock: &str) {
        self.fullmove_clock = fullmove_clock.parse().unwrap();
    }
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub r#type: PieceType,
    pub color: Color,
}

#[derive(Clone, Copy)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
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

impl std::str::FromStr for Square {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A8" => Ok(Square::A8),
            "B8" => Ok(Square::B8),
            "C8" => Ok(Square::C8),
            "D8" => Ok(Square::D8),
            "E8" => Ok(Square::E8),
            "F8" => Ok(Square::F8),
            "G8" => Ok(Square::G8),
            "H8" => Ok(Square::H8),
            "A7" => Ok(Square::A7),
            "B7" => Ok(Square::B7),
            "C7" => Ok(Square::C7),
            "D7" => Ok(Square::D7),
            "E7" => Ok(Square::E7),
            "F7" => Ok(Square::F7),
            "G7" => Ok(Square::G7),
            "H7" => Ok(Square::H7),
            "A6" => Ok(Square::A6),
            "B6" => Ok(Square::B6),
            "C6" => Ok(Square::C6),
            "D6" => Ok(Square::D6),
            "E6" => Ok(Square::E6),
            "F6" => Ok(Square::F6),
            "G6" => Ok(Square::G6),
            "H6" => Ok(Square::H6),
            "A5" => Ok(Square::A5),
            "B5" => Ok(Square::B5),
            "C5" => Ok(Square::C5),
            "D5" => Ok(Square::D5),
            "E5" => Ok(Square::E5),
            "F5" => Ok(Square::F5),
            "G5" => Ok(Square::G5),
            "H5" => Ok(Square::H5),
            "A4" => Ok(Square::A4),
            "B4" => Ok(Square::B4),
            "C4" => Ok(Square::C4),
            "D4" => Ok(Square::D4),
            "E4" => Ok(Square::E4),
            "F4" => Ok(Square::F4),
            "G4" => Ok(Square::G4),
            "H4" => Ok(Square::H4),
            "A3" => Ok(Square::A3),
            "B3" => Ok(Square::B3),
            "C3" => Ok(Square::C3),
            "D3" => Ok(Square::D3),
            "E3" => Ok(Square::E3),
            "F3" => Ok(Square::F3),
            "G3" => Ok(Square::G3),
            "H3" => Ok(Square::H3),
            "A2" => Ok(Square::A2),
            "B2" => Ok(Square::B2),
            "C2" => Ok(Square::C2),
            "D2" => Ok(Square::D2),
            "E2" => Ok(Square::E2),
            "F2" => Ok(Square::F2),
            "G2" => Ok(Square::G2),
            "H2" => Ok(Square::H2),
            "A1" => Ok(Square::A1),
            "B1" => Ok(Square::B1),
            "C1" => Ok(Square::C1),
            "D1" => Ok(Square::D1),
            "E1" => Ok(Square::E1),
            "F1" => Ok(Square::F1),
            "G1" => Ok(Square::G1),
            "H1" => Ok(Square::H1),
            _ => Err(()),
        }
    }
}
