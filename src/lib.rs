#[derive(Debug, Clone, PartialEq)]
pub enum CellValue {
    Empty,
    Number(f64),
    Text(String),
    Formula(String),
}

pub struct CalcGrid {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<CellValue>,
}

impl CalcGrid {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols, cells: vec![CellValue::Empty; rows * cols] }
    }

    pub fn set_cell(&mut self, row: usize, col: usize, val: CellValue) {
        if row < self.rows && col < self.cols {
            self.cells[row * self.cols + col] = val;
        }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<&CellValue> {
        if row < self.rows && col < self.cols {
            Some(&self.cells[row * self.cols + col])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_grid() {
        // Arrange
        let mut grid = CalcGrid::new(10, 10);
        // Act
        grid.set_cell(0, 0, CellValue::Number(42.0));
        // Assert
        assert_eq!(grid.get_cell(0, 0), Some(&CellValue::Number(42.0)));
    }
}
