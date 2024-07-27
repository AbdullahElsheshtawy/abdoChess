use std::default;

fn bit_to_position(bit: u64) -> Result<String, String> {
    if bit == 0 {
        return Err("No piece".to_string());
    } else {
        let index = bit_scan(bit);
        return Ok(index_to_position(index));
    }
}

fn index_to_position(index: usize) -> String {
    let files = index % 8;
    let rank = index / 8 + 1;

    format!("{}{}", (97 + files as u8) as char, rank)
}

static LOOKUPTABLE: [usize; 67] = [
    64, 0, 1, 39, 2, 15, 40, 23, 3, 12, 16, 59, 41, 19, 24, 54, 4, 64, 13, 10, 17, 62, 60, 28, 42,
    30, 20, 51, 25, 44, 55, 47, 5, 32, 64, 38, 14, 22, 11, 58, 18, 53, 63, 9, 61, 27, 29, 50, 43,
    46, 31, 37, 21, 57, 52, 8, 26, 49, 45, 36, 56, 7, 48, 35, 6, 34, 33,
];

#[derive(Debug, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Debug, PartialEq)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, PartialEq)]
struct Piece {
    pos: u64,
    color: Color,
    piece_type: PieceType,
}

impl Piece {
    fn to_string(&self) -> &str {
        let result = match self.piece_type {
            PieceType::Pawn => "p",
            PieceType::Knight => "n",
            PieceType::Bishop => "b",
            PieceType::Rook => "r",
            PieceType::Queen => "q",
            PieceType::King => "k",
        };

        if self.color == Color::White {
            result.to_uppercase();
        }

        result
    }
    fn from_string(piece: char, pos: u64) -> Option<Piece> {
        let piece_type: PieceType;
        let color: Color;
        match piece.to_ascii_lowercase() {
            'p' => piece_type = PieceType::Pawn,
            'n' => piece_type = PieceType::Knight,
            'b' => piece_type = PieceType::Bishop,
            'r' => piece_type = PieceType::Rook,
            'q' => piece_type = PieceType::Queen,
            'k' => piece_type = PieceType::King,
            _ => return None,
        };
        if piece.is_uppercase() {
            color = Color::White;
        } else {
            color = Color::Black;
        }

        Some(Piece {
            pos,
            color,
            piece_type,
        })
    }
}

#[derive(Debug, PartialEq)]
enum Square {
    Empty,
    Full(u64),
}

struct Game {
    pieces: Vec<Piece>,
    square: Vec<Square>,
}

impl Game {
    fn new() -> Option<Game> {
        let default = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let fen: Vec<&str> = default.split(' ').collect();
        if fen.len() != 6 {
            return None;
        }

        todo!("Game::new() not yet implemented!")
    }
    fn to_string(&self) -> String {
        let mut board = "".to_string();
        let mut temp = "".to_string();

        for (i, square) in self.square.iter().enumerate() {
            match square {
                Square::Empty => temp.push_str(&index_to_position(i)),
                Square::Full(index) => temp.push_str(self.pieces[*index as usize].to_string()),
            }
            if (i + 1) % 8 == 0 {
                temp.push_str("\n");
                board.insert_str(0, &temp);
                temp.clear();
            }
        }
        board
    }
}

fn bit_scan(bit: u64) -> usize {
    let remainder: usize = (bit % 67) as usize;
    LOOKUPTABLE[remainder]
}

fn main() {
    let game = Game::new();
}
