pub enum Color {
	White,
	Black,
}

pub enum Piece {
	King(Color),
	Queen(Color),
	Rook(Color),
	Bishop(Color),
	Knight(Color),
	Pawn(Color),
	None,
}
