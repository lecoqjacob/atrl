use crate::prelude::*;
use std::{iter::Zip, ops::Index};

#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq, Eq)]
pub struct BitGrid {
    pub size: UVec2,
    pub cells: BitVec,
}

impl BitGrid {
    /// Gets the `GridPoint` corresponding to an index
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check if the index is out of bounds.
    pub unsafe fn get_mut_unchecked(
        &mut self,
        index: impl Point2d,
    ) -> BitRef<'_, bitvec::ptr::Mut> {
        let w = self.width() as usize;
        self.cells.get_unchecked_mut(index.as_index(w))
    }

    pub fn set(&mut self, index: impl Point2d, value: bool) {
        if index.is_valid(self.size()) {
            let index = self.get_idx_unchecked(index);
            self.cells.set(index, value);
        }
    }

    pub fn set_unchecked(&mut self, index: impl Point2d, value: bool) {
        let index = self.get_idx_unchecked(index);
        self.cells.set(index, value);
    }

    ///////////////////////////////////////////////////////////////////////////
    // Iterator Functionality
    ///////////////////////////////////////////////////////////////////////////

    /// An iterator over all elements in the grid.
    #[inline]
    pub fn iter(&self) -> bitvec::slice::Iter<'_, usize, bitvec::order::Lsb0> {
        self.cells.iter()
    }

    /// A mutable iterator over all elements in the grid.
    #[inline]
    pub fn iter_mut(&mut self) -> bitvec::slice::IterMut<'_, usize, bitvec::order::Lsb0> {
        self.cells.iter_mut()
    }

    pub fn point_iter(&self) -> PointIterRowMajor {
        self.size.iter()
    }

    pub fn enumerate(
        &self,
    ) -> Zip<PointIterRowMajor, bitvec::slice::Iter<'_, usize, bitvec::order::Lsb0>> {
        self.point_iter().zip(self.iter())
    }
}

impl GridLayer<bool> for BitGrid {
    type MutableReturn<'a> = BitRef<'a, bitvec::ptr::Mut>;

    #[inline(always)]
    fn new_clone(size: impl Size2d, value: bool) -> Self
    where
        bool: Clone,
    {
        let count = size.count();
        let mut cells = BitVec::with_capacity(count);
        cells.resize(count, value);
        Self { cells, size: size.as_uvec2() }
    }

    #[inline(always)]
    fn blit_clone(&mut self, to: impl Point2d, source: &Self, from: impl Point2d, size: impl Size2d)
    where
        bool: Clone,
    {
        for y in 0..size.height() {
            for x in 0..size.width() {
                if let Some(val) = source.get((x + from.x() as u32, y + from.y() as u32)) {
                    self.set((x + to.x() as u32, y + to.y() as u32), *val);
                }
            }
        }
    }

    #[inline(always)]
    fn new_copy(size: impl Size2d, value: bool) -> Self
    where
        bool: Copy,
    {
        let count = size.count();
        let mut cells = BitVec::with_capacity(count);
        cells.resize_with(count, |_| value);
        Self { cells, size: size.as_uvec2() }
    }

    #[inline(always)]
    fn blit_copy(&mut self, to: impl Point2d, source: &Self, from: impl Point2d, size: impl Size2d)
    where
        bool: Copy,
    {
        for y in 0..size.height() {
            for x in 0..size.width() {
                if let Some(val) = source.get((x + from.x() as u32, y + from.y() as u32)) {
                    self.set((x + to.x() as u32, y + to.y() as u32), *val);
                }
            }
        }
    }

    #[inline(always)]
    fn new_default(size: impl Size2d) -> Self {
        let count = size.count();
        Self { cells: bitvec![0_usize; count], size: size.as_uvec2() }
    }

    #[inline(always)]
    fn new_fn(size: impl Size2d, f: impl Fn(IVec2) -> bool) -> Self {
        let count = size.count();
        let mut cells = BitVec::with_capacity(count);
        for coord in size.iter() {
            cells.push(f(coord));
        }
        Self { size: size.as_uvec2(), cells }
    }

    #[inline]
    fn width(&self) -> u32 {
        self.size.width()
    }

    #[inline]
    fn height(&self) -> u32 {
        self.size.height()
    }

    #[inline]
    fn size(&self) -> UVec2 {
        self.size
    }

    #[inline]
    fn len(&self) -> usize {
        self.cells.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    #[inline]
    fn in_bounds(&self, point: impl Point2d) -> bool {
        point.is_valid(self.size())
    }

    #[inline]
    fn get_idx_unchecked(&self, point: impl Point2d) -> usize {
        point.as_index(self.width() as usize)
    }

    #[inline]
    fn get_idx(&self, coord: impl Point2d) -> Option<usize> {
        if coord.is_valid(self.size()) {
            Some(self.get_idx_unchecked(coord))
        } else {
            None
        }
    }

    #[inline]
    fn index_to_pt_unchecked(&self, idx: usize) -> IVec2 {
        let x = idx % self.width() as usize;
        let y = idx / self.width() as usize;
        IVec2::new(x as i32, y as i32)
    }

    #[inline]
    fn index_to_pt(&self, idx: usize) -> Option<IVec2> {
        let pt = self.index_to_pt_unchecked(idx);
        if pt.is_valid(self.size()) {
            Some(pt)
        } else {
            None
        }
    }

    #[inline]
    fn get(&self, pos: impl Point2d) -> Option<&bool> {
        self.get_idx(pos).map(|idx| &self.cells[idx])
    }

    fn get_mut(&mut self, pos: impl Point2d) -> Option<Self::MutableReturn<'_>> {
        let width = self.width() as usize;
        self.cells.get_mut(pos.as_index(width))
    }

    fn get_unchecked(&self, pos: impl Point2d) -> &bool {
        self.cells.index(self.get_idx_unchecked(pos))
    }

    /// Gets a mutable reference corresponding to an index
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check if the index is out of bounds.
    fn get_mut_unchecked(&mut self, pos: impl Point2d) -> Self::MutableReturn<'_> {
        let w = self.width() as usize;
        unsafe { self.cells.get_unchecked_mut(pos.as_index(w)) }
    }

    fn set(&mut self, pos: impl Point2d, value: bool) -> Option<bool> {
        if pos.is_valid(self.size()) {
            let index = self.get_idx_unchecked(pos);
            Some(self.cells.replace(index, value))
        } else {
            None
        }
    }

    fn set_unchecked(&mut self, pos: impl Point2d, value: bool) -> bool {
        let index = self.get_idx_unchecked(pos);
        self.cells.replace(index, value)
    }
}

///////////////////////////////////////////////////////////////////////////
// Indexing
///////////////////////////////////////////////////////////////////////////

impl Index<usize> for BitGrid {
    type Output = bool;

    #[inline]
    fn index(&self, index: usize) -> &bool {
        &self.cells[index]
    }
}

impl<P: Point2d> Index<P> for BitGrid {
    type Output = bool;

    #[inline]
    fn index(&self, index: P) -> &bool {
        self.get_unchecked(index)
    }
}