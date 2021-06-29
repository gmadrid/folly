use std::cell::Cell;
use std::collections::HashMap;

use super::{Coord, Grid, Piece};

/*
  A grid laid out on hex paper.

       0 1 2 3 4 5 6 7 8
    0   * * * * * * * * *
    1    * * * + * * * * *
    2     * * * * * @ * * *

  So, for example, + = (3, 1) and @ = (5, 2).

  Cells adjacent to '+' are (3, 0), (4, 0), (2, 1), (4, 1), (2, 2), and (3, 2).

  The grid is expanded (and contracted) dynamically depending on the pieces
  which are placed in it. This means that the min/max may change with each piece
  added or removed.

  The grid is unbounded on all sides, so negative coordinates are possible
  and the behavior is well-defined. (Although, the underlying Coord type uses
  i16 types, so there is a size limit to the implementation.  Practically, this
  shouldn't be a problem, since most games don't have boards larger than 32K
  along a side.
*/
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
    fn min(&self) -> (i16, i16) {
        self.ensure_min_max();

        (self.min_x.get(), self.min_y.get())
    }

    fn max(&self) -> (i16, i16) {
        self.ensure_min_max();

        (self.max_x.get(), self.max_y.get())
    }

    fn height(&self) -> usize {
        if self.num_pieces() == 0 {
            0
        } else {
            (self.max().1 - self.min().1 + 1) as usize
        }
    }

    fn width(&self) -> usize {
        if self.num_pieces() == 0 {
            0
        } else {
            (self.max().0 - self.min().0 + 1) as usize
        }
    }

    fn add(&mut self, coord: Coord, piece: T) {
        // TODO: check bounds before setting dirty.
        self.min_max_dirty = true;

        self.pieces.insert(coord, piece);
    }

    fn remove(&mut self, coord: Coord) {
        self.min_max_dirty = true;
        self.pieces.remove(&coord);
    }

    fn at(&self, coord: Coord) -> Option<&T> {
        self.pieces.get(&coord)
    }

    fn num_pieces(&self) -> usize {
        self.pieces.len()
    }

    fn adjacents(&self, coord: Coord) -> Vec<Coord> {
        let x = coord.x;
        let y = coord.y;
        vec![
            Coord::new(x, y - 1),
            Coord::new(x + 1, y - 1),
            Coord::new(x - 1, y),
            Coord::new(x + 1, y),
            Coord::new(x - 1, y + 1),
            Coord::new(x, y + 1),
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Default, PartialEq, Eq)]
    struct TestPiece(u8);
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
        assert_eq!((0, 0), def().min());
    }

    #[test]
    fn max_of_default() {
        assert_eq!((0, 0), def().max());
    }

    #[test]
    fn add_one() {
        let mut grid = def();

        grid.add(Coord::new(1, 2), TestPiece(0));
        assert_eq!(1, grid.num_pieces());
        assert_eq!(1, grid.height());
        assert_eq!(1, grid.width());
        assert_eq!((1, 2), grid.min());
        assert_eq!((1, 2), grid.max());
    }

    #[test]
    fn add_two() {
        let mut grid = def();

        grid.add(Coord::new(1, 3), TestPiece(0));
        grid.add(Coord::new(4, 2), TestPiece(1));
        assert_eq!(2, grid.num_pieces());
        assert_eq!(2, grid.height());
        assert_eq!(4, grid.width());
        assert_eq!((1, 2), grid.min());
        assert_eq!((4, 3), grid.max());
    }

    #[test]
    fn add_four() {
        let mut grid = def();
        grid.add(Coord::new(1, 8), TestPiece(0));
        grid.add(Coord::new(3, 16), TestPiece(1));
        grid.add(Coord::new(-5, 3), TestPiece(2));
        grid.add(Coord::new(0, -4), TestPiece(3));

        assert_eq!(4, grid.num_pieces());
        assert_eq!(21, grid.height());
        assert_eq!(9, grid.width());
        assert_eq!((-5, -4), grid.min());
        assert_eq!((3, 16), grid.max());
    }

    #[test]
    fn replace() {
        let mut grid = def();
        grid.add(Coord::new(1, 8), TestPiece(0));
        grid.add(Coord::new(3, 16), TestPiece(1));

        // replace a piece.
        grid.add(Coord::new(3, 16), TestPiece(42));

        assert_eq!(2, grid.num_pieces());
        assert_eq!(9, grid.height());
        assert_eq!(3, grid.width());
        assert_eq!((1, 8), grid.min());
        assert_eq!((3, 16), grid.max());
        assert_eq!(&TestPiece(42), grid.at(Coord::new(3, 16)).unwrap());
    }

    #[test]
    fn removal() {
        let mut grid = def();

        grid.add(Coord::new(1, 8), TestPiece(0));
        grid.add(Coord::new(3, 16), TestPiece(1));
        grid.add(Coord::new(2, 18), TestPiece(2));

        grid.remove(Coord::new(3, 16));

        assert_eq!(2, grid.num_pieces());
        assert_eq!(11, grid.height());
        assert_eq!(2, grid.width());
        assert_eq!((1, 8), grid.min());
        assert_eq!((2, 18), grid.max());
    }

    #[test]
    fn at_and_occupied() {
        let mut grid = def();
        grid.add(Coord::new(1, 8), TestPiece(0));
        grid.add(Coord::new(3, 16), TestPiece(1));

        // TODO: add a way to differentiate between TestPieces
        assert_eq!(&TestPiece(0), grid.at(Coord::new(1, 8)).unwrap());
        assert_eq!(&TestPiece(1), grid.at(Coord::new(3, 16)).unwrap());
        assert!(grid.at(Coord::new(0, 0)).is_none());

        assert!(grid.occupied(Coord::new(1, 8)));
        assert!(grid.occupied(Coord::new(3, 16)));
        assert!(!grid.occupied(Coord::new(3, 8)));
    }

    #[test]
    fn remove_missing() {
        let mut grid = def();

        grid.remove(Coord::new(8, 8));
    }

    #[test]
    fn adjacents() {
        let grid = def();
        assert_eq!(
            vec![
                Coord::new(3, 0),
                Coord::new(4, 0),
                Coord::new(2, 1),
                Coord::new(4, 1),
                Coord::new(2, 2),
                Coord::new(3, 2)
            ],
            grid.adjacents(Coord::new(3, 1))
        );
    }
}
