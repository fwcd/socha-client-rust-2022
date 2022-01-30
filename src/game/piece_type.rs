#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PieceType {
    /// Moves only diagonally forwards.
    Herzmuschel,
    /// Moves only to adjacent fields.
    Moewe,
    /// Moves only diagonally or forwards.
    Seestern,
    /// Like a knight in chess. Only non-light figure.
    Robbe,
}

impl PieceType {
    /// Checks whether a piece is lightweight. Only the 'robbe' is non-light.
    pub fn is_light(&self) -> bool {
        !matches!(self, Self::Robbe)
    }
}
