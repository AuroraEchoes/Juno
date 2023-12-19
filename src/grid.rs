use std::slice::Iter;

use crate::vector::{IVec2, Vector};

pub struct Grid<T> {
    size: IVec2,
    grid: Vec<Tile<T>>,
}

impl<T> Grid<T> {
    pub fn new(size: IVec2) -> Self {
        let grid = Vec::with_capacity((size.x() * size.y()) as usize);
        return Self { size, grid };
    }

    pub fn push(&mut self, tile: Tile<T>) {
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

    pub fn tile(&self, pos: IVec2) -> Option<&Tile<T>> {
        return self.grid.get((pos.y() * self.width() + pos.x()) as usize);
    }

    pub fn tile_mut(&mut self, pos: IVec2) -> Option<&mut Tile<T>> {
        let w = *self.width();
        return self.grid.get_mut((pos.y() * w + pos.x()) as usize);
    }

    pub fn tiles(&self) -> Iter<'_, Tile<T>> {
        return self.grid.iter();
    }
}

pub struct Tile<T> {
    pos: IVec2,
    contents: T,
}

impl<T> Tile<T> {
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
