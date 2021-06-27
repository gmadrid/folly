use std::cell::Cell;
use std::collections::HashMap;

use super::{Coord, Grid, Piece};

#[derive(Default, Debug)]
pub struct DynamicHexGrid<T>
where
    T: Default,
{
    min_max_dirty: bool,

    min_x: Cell<i16>,
    min_y: Cell<i16>,
    max_x: Cell<i16>,
    max_y: Cell<i16>,

    pieces: HashMap<Coord, T>,
}

impl<T> DynamicHexGrid<T>
where
    T: Default,
{
    fn ensure_min_max(&self) {
        if self.min_max_dirty {
            // TODO: make this more efficient by computing all in one pass.
            self.min_x
                .set(self.pieces.keys().map(|c| c.x).min().unwrap_or(0));
            self.max_x
                .set(self.pieces.keys().map(|c| c.x).max().unwrap_or(0));
            self.min_y
                .set(self.pieces.keys().map(|c| c.y).min().unwrap_or(0));
            self.max_y
                .set(self.pieces.keys().map(|c| c.y).max().unwrap_or(0));
        }
    }
}

impl<T> Grid<T> for DynamicHexGrid<T>
where
    T: Piece + Default,
{
    fn min(&self) -> Coord {
        self.ensure_min_max();

        Coord::new(self.min_x.get(), self.min_y.get())
    }

    fn max(&self) -> Coord {
        self.ensure_min_max();

        // unwrap:: okay because ensure_min_max guarantees valid values
        Coord::new(self.max_x.get(), self.max_y.get())
    }

    fn height(&self) -> usize {
        if self.num_pieces() == 0 {
            0
        } else {
            (self.max().y - self.min().y + 1) as usize
        }
    }

    fn width(&self) -> usize {
        if self.num_pieces() == 0 {
            0
        } else {
            (self.max().x - self.min().x + 1) as usize
        }
    }

    fn add(&mut self, coord: Coord, piece: T) {
        // TODO: check bounds before setting dirty.
        self.min_max_dirty = true;

        self.pieces.insert(coord, piece);
    }

    fn num_pieces(&self) -> usize {
        self.pieces.len()
    }

    fn adjacents(&self) -> Vec<Coord> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Default)]
    struct TestPiece;
    impl Piece for TestPiece {}

    fn def() -> DynamicHexGrid<TestPiece> {
        DynamicHexGrid::<TestPiece>::default()
    }

    #[test]
    fn size_of_default() {
        assert_eq!(0, def().height());
        assert_eq!(0, def().width());
        assert_eq!(0, def().num_pieces());
    }

    #[test]
    fn min_of_default() {
        assert_eq!(Coord::new(0, 0), def().min());
    }

    #[test]
    fn max_of_default() {
        assert_eq!(Coord::new(0, 0), def().max());
    }

    #[test]
    fn add_one() {
        let mut grid = def();

        grid.add(Coord::new(1, 2), TestPiece);
        assert_eq!(1, grid.num_pieces());
        assert_eq!(1, grid.height());
        assert_eq!(1, grid.width());
        assert_eq!(Coord::new(1, 2), grid.min());
        assert_eq!(Coord::new(1, 2), grid.max());
    }
}
