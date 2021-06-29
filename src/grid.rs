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

pub trait Grid<P>
where
    P: Piece,
{
    // Returns the current height of the board.
    fn height(&self) -> usize;
    // Returns the current width of the board.
    fn width(&self) -> usize;

    // Returns the minimum x and minimum y of any spaces in the grid.
    // (x, y) may not be a space in the grid, but (x, _) and (_, y) will
    // be a space.
    fn min(&self) -> (i16, i16);
    fn max(&self) -> (i16, i16);

    fn add(&mut self, coord: Coord, piece: P);
    fn remove(&mut self, coord: Coord);
    fn at(&self, coord: Coord) -> Option<&P>;
    fn occupied(&self, coord: Coord) -> bool {
        self.at(coord).is_some()
    }

    fn num_pieces(&self) -> usize;

    fn adjacents(&self, coord: Coord) -> Vec<Coord>;
}
