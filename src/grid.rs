mod dynhex;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash)]
pub struct Coord {
    pub x: i16,
    pub y: i16,
}

impl Coord {
    fn new(x: i16, y: i16) -> Coord {
        Coord { x, y }
    }
}

pub trait Piece {}

pub trait Grid<T>
where
    T: Piece,
{
    fn height(&self) -> usize;
    fn width(&self) -> usize;

    fn min(&self) -> Coord;
    fn max(&self) -> Coord;

    fn add(&mut self, coord: Coord, piece: T);
    fn num_pieces(&self) -> usize;

    fn adjacents(&self) -> Vec<Coord>;
}
