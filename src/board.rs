#[derive(Debug)]
pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
    pub white_k_castle: bool,
    pub white_q_castle: bool,
    pub black_k_castle: bool,
    pub black_q_castle: bool,
    pub white_to_move: bool,
    pub is_in_check: bool,
    pub is_game_over: bool,
    pub en_passent: Option<Point>,
    pub halfmove_clock: usize,
    pub fullmove_number: usize,
}

impl Board {
    pub fn default() -> Self {
        let mut board = Board {
            squares: [[None; 8]; 8],
            white_k_castle: false,
            white_q_castle: false,
            black_k_castle: false,
            black_q_castle: false,
            is_game_over: false,
            is_in_check: false,
            white_to_move: true,
            en_passent: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        };
        let default: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        board.parse_fen(default);
        board
    }

    pub fn print(&self) {
        for x in 0..8 {
            for y in 0..8 {
                match self.squares[x][y] {
                    Some(Piece {
                        Type: PieceType::Pawn,
                        Color: PieceColor::Black,
                    }) => print!("|p|"),
                    Some(Piece {
                        Type: PieceType::Knight,
                        Color: PieceColor::Black,
                    }) => print!("|n|"),
                    Some(Piece {
                        Type: PieceType::Bishop,
                        Color: PieceColor::Black,
                    }) => print!("|b|"),
                    Some(Piece {
                        Type: PieceType::Rook,
                        Color: PieceColor::Black,
                    }) => print!("|r|"),
                    Some(Piece {
                        Type: PieceType::Queen,
                        Color: PieceColor::Black,
                    }) => print!("|q|"),
                    Some(Piece {
                        Type: PieceType::King,
                        Color: PieceColor::Black,
                    }) => print!("|k|"),
                    Some(Piece {
                        Type: PieceType::Pawn,
                        Color: PieceColor::White,
                    }) => print!("|P|"),
                    Some(Piece {
                        Type: PieceType::Knight,
                        Color: PieceColor::White,
                    }) => print!("|N|"),
                    Some(Piece {
                        Type: PieceType::Bishop,
                        Color: PieceColor::White,
                    }) => print!("|B|"),
                    Some(Piece {
                        Type: PieceType::Rook,
                        Color: PieceColor::White,
                    }) => print!("|R|"),
                    Some(Piece {
                        Type: PieceType::Queen,
                        Color: PieceColor::White,
                    }) => print!("|Q|"),
                    Some(Piece {
                        Type: PieceType::King,
                        Color: PieceColor::White,
                    }) => print!("|K|"),
                    None => print!("|.|"),
                }
            }
            println!();
            print!("\n");
        }
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
        let halfmove_clock: usize = parts[4].parse().expect("Invalid halfmove clock");
        let fullmove_number: usize = parts[5].parse().expect("Invalid fullmove number");

        self.parse_pieces(piece_placement);
        self.side_to_move(side_to_move);
        self.castling_availability(castling_availability);
        self.en_passant_target(en_passant_target);
        self.halfmove_clock(halfmove_clock);
        self.fullmove_number(fullmove_number);
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
            self.en_passent = None;
        } else {
            let rank = en_passant_target.chars().next().unwrap() as usize - 'a' as usize;
            let file = en_passant_target
                .chars()
                .nth(1)
                .unwrap()
                .to_digit(10)
                .unwrap() as usize
                - 1;

            self.en_passent = Some(Point { rank, file });
        }
    }

    pub fn halfmove_clock(&mut self, halfmove_clock: usize) {
        self.halfmove_clock = halfmove_clock;
    }

    pub fn fullmove_number(&mut self, fullmove_number: usize) {
        self.fullmove_number = fullmove_number;
    }

    pub fn place_square(&mut self, piece_name: char, pos: &Point) {
        let Color = if piece_name.is_uppercase() {
            PieceColor::White
        } else {
            PieceColor::Black
        };
        let Type = match piece_name.to_uppercase().next().unwrap() {
            'P' => PieceType::Pawn,
            'N' => PieceType::Knight,
            'B' => PieceType::Bishop,
            'R' => PieceType::Rook,
            'Q' => PieceType::Queen,
            'K' => PieceType::King,
            _ => panic!("WRONG PIECE TYPE {piece_name}!"),
        };
        self.squares[pos.rank][pos.file] = Some(Piece { Type, Color });
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
