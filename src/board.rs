use std::usize;

#[derive(Debug)]
pub struct Board {
    pub squares: [[Square; 8]; 8],
    pub white_k_castle: bool,
    pub white_q_castle: bool,
    pub black_k_castle: bool,
    pub black_q_castle: bool,
    pub white_to_move: bool,
    pub is_in_check: bool,
    pub is_game_over: bool,
    pub en_passent: EnPassant,
}

#[derive(Debug)]
pub struct EnPassant {
    availabile: bool,
    target: Option<Point>,
}

impl Board {
    pub fn default() -> Self {
        let mut board = Board {
            squares: [[Square::Empty; 8]; 8],
            white_k_castle: false,
            white_q_castle: false,
            black_k_castle: false,
            black_q_castle: false,
            is_game_over: false,
            is_in_check: false,
            white_to_move: true,
            en_passent: EnPassant {
                availabile: false,
                target: None,
            },
        };
        let default: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        board.parse_fen(default);
        board
    }

    pub fn parse_fen(&mut self, fen: &str) {
        let parts: Vec<&str> = fen.split_whitespace().collect();

        if parts.len() != 6 {
            panic!("Invalid FEN string");
        }

        let piece_placement = parts[0];
        let side_to_move = parts[1];
        let castling_availability = parts[2];
        let en_passant_target = parts[3];
        let halfmove_clock: u32 = parts[4].parse().expect("Invalid halfmove clock");
        let fullmove_number: u32 = parts[5].parse().expect("Invalid fullmove number");

        self.parse_pieces(piece_placement);
        self.side_to_move(side_to_move);
        self.castling_availability(castling_availability);
        self.en_passant_target(en_passant_target);
    }

    pub fn parse_pieces(&mut self, piece_placement: &str) {
        let mut ranknum: usize = 0;
        let mut file: usize = 0;
        for rank in piece_placement.split('/') {
            for ch in rank.chars() {
                if ch.is_digit(10) {
                    file += ch.to_digit(10).unwrap() as usize;
                } else {
                    self.place_square(
                        ch,
                        &Point {
                            rank: ranknum,
                            file,
                        },
                    );
                    file += 1;
                }
            }
            ranknum += 1;
            file = 0;
        }
    }

    pub fn side_to_move(&mut self, side_to_move: &str) {
        match side_to_move {
            "w" => self.white_to_move = true,
            "b" => self.white_to_move = false,
            _ => eprintln!("ERROR: side_to_move in the FEN string is incorrect!"),
        }
    }

    pub fn castling_availability(&mut self, castling_availability: &str) {
        for ch in castling_availability.chars() {
            match ch {
                'K' => self.white_k_castle = true,
                'Q' => self.white_q_castle = true,
                'k' => self.black_k_castle = true,
                'q' => self.black_q_castle = true,
                _ => eprintln!(
                    "ERROR castlings castling_availability in the FEN string is incorrect!"
                ),
            }
        }
    }

    pub fn en_passant_target(&mut self, en_passant_target: &str) {
        if en_passant_target == "-" {
            self.en_passent = EnPassant {
                availabile: false,
                target: None,
            }
        } else {
            let rank = en_passant_target.chars().next().unwrap() as usize - 'a' as usize;
            let file = en_passant_target
                .chars()
                .nth(1)
                .unwrap()
                .to_digit(10)
                .unwrap() as usize
                - 1;

            self.en_passent = EnPassant {
                availabile: true,
                target: Some(Point { rank, file }),
            }
        }
    }

    pub fn place_square(&mut self, piece_name: char, pos: &Point) {
        self.squares[pos.rank][pos.file] = Square::place_piece(piece_name);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Square {
    Empty,
    Full(Piece),
}

impl Square {
    fn default() -> Square {
        Square::Empty
    }

    fn place_piece(name: char) -> Square {
        let pc = if name.is_uppercase() {
            PieceColor::White
        } else {
            PieceColor::Black
        };
        let pt = match name.to_ascii_uppercase() {
            'P' => PieceType::Pawn,
            'R' => PieceType::Rook,
            'N' => PieceType::Knight,
            'B' => PieceType::Bishop,
            'Q' => PieceType::Queen,
            'K' => PieceType::King,
            _ => panic!("Unknown PieceType!"),
        };

        Square::Full(Piece {
            Type: pt,
            Color: pc,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub Type: PieceType,
    pub Color: PieceColor,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug)]
pub struct Point {
    rank: usize,
    file: usize,
}

impl Point {
    fn empty() -> Point {
        Point { file: 0, rank: 0 }
    }
}
