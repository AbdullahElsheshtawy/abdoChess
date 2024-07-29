mod bitboards;
fn main() {
    let board = bitboards::Board::default();
    println!("{:?}", board.en_passant);
}
