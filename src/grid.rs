use std::slice::{Iter, IterMut};

use crate::{
    ivec,
    vector::{IVec2, Vector},
};

pub struct Grid<T> {
    size: IVec2,
    grid: Vec<GridItem<T>>,
}

impl<T: Clone> Grid<T> {
    pub fn new(size: IVec2) -> Self {
        let grid = Vec::with_capacity((size.x() * size.y()) as usize);
        return Self { size, grid };
    }

    pub fn fill(&mut self, item: T) {
        for y in 0..*self.height() {
            for x in 0..*self.width() {
                self.push(GridItem::new(ivec!(x, y), item.clone()));
            }
        }
    }

    pub fn push(&mut self, tile: GridItem<T>) {
        self.grid.push(tile);
    }

    pub fn width(&self) -> &i32 {
        return self.size.x();
    }

    pub fn height(&self) -> &i32 {
        return self.size.y();
    }

    pub fn size(&self) -> &IVec2 {
        return &self.size;
    }

    pub fn tile(&self, pos: &IVec2) -> Option<&GridItem<T>> {
        return self.grid.get((pos.y() * self.width() + pos.x()) as usize);
    }

    pub fn tile_mut(&mut self, pos: &IVec2) -> Option<&mut GridItem<T>> {
        let w = *self.width();
        return self.grid.get_mut((pos.y() * w + pos.x()) as usize);
    }

    pub fn tiles(&self) -> Iter<'_, GridItem<T>> {
        return self.grid.iter();
    }

    pub fn tiles_mut(&mut self) -> IterMut<'_, GridItem<T>> {
        return self.grid.iter_mut();
    }

    pub fn adjacent(&self, pos: &IVec2) -> impl Iterator<Item = &GridItem<T>> {
        let adjacent_positions = [
            ivec!(1, 0) + pos,
            ivec!(0, -1) + pos,
            ivec!(-1, 0) + pos,
            ivec!(0, 1) + pos,
        ];
        return self
            .tiles()
            .filter(move |t| adjacent_positions.contains(t.pos()));
    }

    pub fn adjacent_mut(&mut self, pos: &IVec2) -> impl Iterator<Item = &mut GridItem<T>> {
        let adjacent_positions = [
            ivec!(1, 0) + pos,
            ivec!(0, -1) + pos,
            ivec!(-1, 0) + pos,
            ivec!(0, 1) + pos,
        ];
        return self
            .tiles_mut()
            .filter(move |t| adjacent_positions.contains(t.pos()));
    }

    pub fn adjacent_diagonal(&self, pos: &IVec2) -> impl Iterator<Item = &GridItem<T>> {
        let adjacent_positions = [
            ivec!(1, 0) + pos,
            ivec!(1, -1) + pos,
            ivec!(0, -1) + pos,
            ivec!(-1, 1) + pos,
            ivec!(-1, 0) + pos,
            ivec!(-1, 1) + pos,
            ivec!(0, 1) + pos,
            ivec!(1, 1) + pos,
        ];
        return self
            .tiles()
            .filter(move |t| adjacent_positions.contains(t.pos()));
    }

    pub fn adjacent_diagonal_mut(&mut self, pos: &IVec2) -> impl Iterator<Item = &mut GridItem<T>> {
        let adjacent_positions = [
            ivec!(1, 0) + pos,
            ivec!(1, -1) + pos,
            ivec!(0, -1) + pos,
            ivec!(-1, 1) + pos,
            ivec!(-1, 0) + pos,
            ivec!(-1, 1) + pos,
            ivec!(0, 1) + pos,
            ivec!(1, 1) + pos,
        ];
        return self
            .tiles_mut()
            .filter(move |t| adjacent_positions.contains(t.pos()));
    }
}

#[derive(Clone, Debug)]
pub struct GridItem<T> {
    pos: IVec2,
    contents: T,
}

impl<T> GridItem<T> {
    pub fn new(pos: IVec2, contents: T) -> GridItem<T> {
        return Self { pos, contents };
    }
    pub fn contents(&self) -> &T {
        return &self.contents;
    }

    pub fn contents_mut(&mut self) -> &mut T {
        return &mut self.contents;
    }

    pub fn pos(&self) -> &IVec2 {
        return &self.pos;
    }
}
