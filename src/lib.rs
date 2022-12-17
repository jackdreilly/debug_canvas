use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Index, IndexMut},
};

pub struct DebugCanvas {
    points: HashMap<(i64, i64), char>,
    min_row: i64,
    max_row: i64,
    min_col: i64,
    max_col: i64,
}
impl DebugCanvas {
    fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    pub fn new() -> Self {
        Self {
            points: HashMap::new(),
            min_row: i64::MAX,
            max_row: i64::MIN,
            min_col: i64::MAX,
            max_col: i64::MIN,
        }
    }

    pub fn remove(&mut self, (row, col): (i64, i64)) {
        self.points.remove(&(row, col));
        self.max_col = self.points.keys().map(|&f| f.1).max().unwrap_or_default();
        self.min_col = self.points.keys().map(|&f| f.1).min().unwrap_or_default();
        self.max_row = self.points.keys().map(|&f| f.0).max().unwrap_or_default();
        self.min_row = self.points.keys().map(|&f| f.0).min().unwrap_or_default();
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
        for row in (self.min_row..=self.max_row).rev() {
            for col in self.min_col..=self.max_col {
                write!(f, "{}", self[(row, col)])?;
            }
            if row > self.min_row {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Index<(i64, i64)> for DebugCanvas {
    type Output = char;

    fn index(&self, (row, col): (i64, i64)) -> &Self::Output {
        self.points.get(&(row, col)).unwrap_or(&' ')
    }
}

impl IndexMut<(i64, i64)> for DebugCanvas {
    fn index_mut(&mut self, (row, col): (i64, i64)) -> &mut Self::Output {
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
