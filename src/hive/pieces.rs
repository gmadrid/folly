use crate::grid::Piece;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Bug {
    Ant,
    Beetle(Box<HivePiece>),
    Grasshopper,
    QueenBee,
    Spider,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HivePiece {
    color: Color,
    bug: Bug,
}

impl Piece for HivePiece {}
