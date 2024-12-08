use std::fmt;

pub struct Grid<V> {
    inner: Vec<Vec<V>>,
    rows: usize,
    cols: usize,
}

impl Grid<u8> {
    #[inline]
    pub fn from_str(s: &str) -> Self {
        let inner: Vec<_> = s
            .split('\n')
            .filter(|l| !l.is_empty())
            .map(|l| l.as_bytes().to_vec())
            .collect();
        Self::new(inner)
    }
}

#[allow(dead_code)]
impl<V> Grid<V> {
    #[inline]
    pub fn with_default(rows: usize, cols: usize, default: V) -> Self
    where
        V: Clone,
    {
        let inner = vec![vec![default; cols]; rows];
        Self { inner, rows, cols }
    }

    #[inline]
    pub fn new(inner: Vec<Vec<V>>) -> Grid<V> {
        let rows = inner.len();
        let cols = if rows == 0 { 0 } else { inner[0].len() };
        Self { inner, rows, cols }
    }

    #[inline]
    pub const fn rows(&self) -> usize {
        self.rows
    }

    #[inline]
    pub const fn cols(&self) -> usize {
        self.cols
    }

    #[inline]
    pub fn set<R, C>(&mut self, row: R, col: C, value: V) -> V
    where
        R: TryInto<usize> + fmt::Debug,
        <R as TryInto<usize>>::Error: fmt::Debug,
        C: TryInto<usize> + fmt::Debug,
        <C as TryInto<usize>>::Error: fmt::Debug,
    {
        let row = row.try_into().unwrap();
        let col = col.try_into().unwrap();
        std::mem::replace(&mut self.inner[row][col], value)
    }

    #[inline]
    pub fn get<R, C>(&self, row: R, col: C) -> &V
    where
        R: TryInto<usize> + fmt::Debug,
        <R as TryInto<usize>>::Error: fmt::Debug,
        C: TryInto<usize> + fmt::Debug,
        <C as TryInto<usize>>::Error: fmt::Debug,
    {
        let row = row.try_into().unwrap();
        let col = col.try_into().unwrap();
        &self.inner[row][col]
    }

    #[inline]
    pub fn in_range<R, C>(&self, row: R, col: C) -> bool
    where
        R: TryInto<i64> + fmt::Debug,
        <R as TryInto<i64>>::Error: fmt::Debug,
        C: TryInto<i64> + fmt::Debug,
        <C as TryInto<i64>>::Error: fmt::Debug,
    {
        let row = row.try_into().unwrap();
        let col = col.try_into().unwrap();
        0 <= row && row < self.rows() as i64 && 0 <= col && col < self.cols() as i64
    }

    #[inline]
    pub fn try_get<R, C>(&self, row: R, col: C) -> Option<&V>
    where
        R: TryInto<i64> + fmt::Debug + Copy,
        <R as TryInto<i64>>::Error: fmt::Debug,
        C: TryInto<i64> + fmt::Debug + Copy,
        <C as TryInto<i64>>::Error: fmt::Debug,
    {
        if self.in_range(row, col) {
            return None;
        }

        let row = row.try_into().unwrap();
        let col = col.try_into().unwrap();

        Some(&self.inner[row as usize][col as usize])
    }
}
