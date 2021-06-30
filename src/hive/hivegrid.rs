use crate::grid::DynamicHexGrid;
use crate::grid::{Coord, Grid};
use crate::hive::pieces::HivePiece;

#[derive(Debug)]
struct HiveGrid {
    base_grid: DynamicHexGrid<HivePiece>,
}

impl HiveGrid {
    pub fn new() -> HiveGrid {
        HiveGrid {
            base_grid: DynamicHexGrid::new(),
        }
    }

    pub fn adjacent_occupied(&self, coord: Coord) -> Vec<Coord> {
        self.adjacents(coord)
            .filter(|c| !self.base_grid.occupied(*c))
            .collect::<Vec<_>>()
    }

    pub fn adjacent_empty(&self, coord: Coord) -> Vec<Coord> {
        self.adjacents(coord)
            .filter(|c| !self.base_grid.occupied(*c))
            .collect::<Vec<_>>()
    }
}

impl Grid<HivePiece> for HiveGrid {
    type CoordIter = <DynamicHexGrid<HivePiece> as Grid<HivePiece>>::CoordIter;

    fn height(&self) -> usize {
        self.base_grid.height()
    }
    fn width(&self) -> usize {
        self.base_grid.width()
    }
    fn min(&self) -> (i16, i16) {
        self.base_grid.min()
    }
    fn max(&self) -> (i16, i16) {
        self.base_grid.max()
    }

    fn add(&mut self, coord: Coord, piece: HivePiece) {
        self.base_grid.add(coord, piece)
    }
    fn remove(&mut self, coord: Coord) {
        self.base_grid.remove(coord)
    }
    fn at(&self, coord: Coord) -> Option<&HivePiece> {
        self.base_grid.at(coord)
    }

    fn num_pieces(&self) -> usize {
        self.base_grid.num_pieces()
    }
    fn adjacents(&self, coord: Coord) -> Self::CoordIter {
        self.base_grid.adjacents(coord)
    }
}

#[cfg(test)]
mod test {}
