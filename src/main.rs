use bitboards::Square;

mod bitboards;
fn main() {
    let mut bitboards = bitboards::Bitboard {
        squares: [Square::Empty; 64],
        pawn: 1 << 1,
        knight: 1 << 2,
        bishop: 1 << 4,
        rook: 1 << 8,
        queen: 1 << 16,
        king: 1 << 32,
        white: 1 << 63,
        black: 1,
    };

    bitboards.squares[63] = Square::Full(46);

    println!("{}", bitboards);
}
