use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(Default, Debug, Clone, Copy)]
pub struct Options {
    pub bottom_oriented: bool,
    pub filler: char,
}

#[derive(Debug)]
pub struct DebugCanvas {
    points: HashMap<(i64, i64), char>,
    min_row: i64,
    max_row: i64,
    min_col: i64,
    max_col: i64,
    options: Options,
}
impl DebugCanvas {
    fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
    pub fn size(&self) -> (u64, u64) {
        if self.is_empty() {
            (0, 0)
        } else {
            (
                (self.max_row - self.min_row + 1) as u64,
                (self.max_col - self.min_col + 1) as u64,
            )
        }
    }

    pub fn new() -> Self {
        Self::with_options(Options::default())
    }
    pub fn with_options(options: Options) -> Self {
        Self {
            options,
            points: HashMap::new(),
            min_row: i64::MAX,
            max_row: i64::MIN,
            min_col: i64::MAX,
            max_col: i64::MIN,
        }
    }

    pub fn clear(&mut self) {
        *self = Self::with_options(self.options);
    }

    pub fn remove(&mut self, (row, col): (i64, i64)) {
        self.points.remove(&(row, col));
        self.max_col = self.points.keys().map(|&f| f.1).max().unwrap_or(i64::MIN);
        self.min_col = self.points.keys().map(|&f| f.1).min().unwrap_or(i64::MAX);
        self.max_row = self.points.keys().map(|&f| f.0).max().unwrap_or(i64::MIN);
        self.min_row = self.points.keys().map(|&f| f.0).min().unwrap_or(i64::MAX);
    }
}

impl Default for DebugCanvas {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for DebugCanvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }
        let (n, m) = self.size();
        for i in 0..n {
            if i > 0 {
                writeln!(f)?;
            }
            for j in 0..m {
                let row = if self.options.bottom_oriented {
                    self.min_row + i as i64
                } else {
                    self.max_row - i as i64
                };
                let col = self.min_col + j as i64;
                write!(f, "{}", self[(row, col)])?;
            }
        }
        writeln!(f)?;
        Ok(())
    }
}

impl<T> Index<(T, T)> for DebugCanvas
where
    T: Into<i64>,
{
    type Output = char;

    fn index(&self, (row, col): (T, T)) -> &Self::Output {
        self.points
            .get(&(row.into(), col.into()))
            .unwrap_or(&self.options.filler)
    }
}

impl<T> IndexMut<(T, T)> for DebugCanvas
where
    T: Into<i64>,
{
    fn index_mut(&mut self, (row, col): (T, T)) -> &mut Self::Output {
        let (row, col) = (row.into(), col.into());
        self.max_col = self.max_col.max(col);
        self.min_col = self.min_col.min(col);
        self.max_row = self.max_row.max(row);
        self.min_row = self.min_row.min(row);
        self.points.entry((row, col)).or_insert(' ')
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_blank_canvas() {
        let mut canvas = DebugCanvas::new();
        assert_eq!(format!("{canvas}"), "");
        canvas[(3, 2)] = '#';
        assert_eq!(format!("{canvas}"), "#");
        canvas[(3, 2)] = '.';
        assert_eq!(format!("{canvas}"), ".");
        canvas[(3, 3)] = '-';
        assert_eq!(format!("{canvas}"), ".-");
        canvas[(2, 3)] = 'a';
        assert_eq!(
            format!("{canvas}"),
            "
.-
 a"
            .trim_start()
        );
        canvas[(2, 5)] = 'b';
        assert_eq!(
            format!("{canvas}"),
            "
.-  
 a b"
            .trim_start()
        );
        canvas.remove((2, 5));
        assert_eq!(
            format!("{canvas}"),
            "
.-
 a"
            .trim_start()
        );
        canvas.remove((2, 8));
        assert_eq!(
            format!("{canvas}"),
            "
.-
 a"
            .trim_start()
        );
    }
}
