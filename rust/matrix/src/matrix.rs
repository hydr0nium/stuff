use std::fmt::Debug;
use std::{iter::zip, mem};

#[derive(Debug)]
pub struct Matrix<T> {
    rows: i64,
    cols: i64,
    values: Option<Vec<Vec<T>>>,
}
#[derive(Debug)]
pub enum MatrixError {
    SizeMismatch(String),
    EmptyMatrix,
}

impl<T> Matrix<T>
where
    T: Debug,
{
    pub fn new() -> Self {
        Matrix {
            rows: 0,
            cols: 0,
            values: None,
        }
    }

    pub fn add_row(&mut self, row: Vec<T>) -> Result<(), MatrixError> {
        match self.values.take() {
            None => {
                self.cols = row.len() as i64;
                let new_vec = vec![row];
                self.values = Some(new_vec);
                self.rows += 1;
                Ok(())
            }
            Some(mut values) => {
                if (row.len() as i64) != self.cols {
                    self.values = Some(values);
                    return Err(MatrixError::SizeMismatch(format!(
                        "Expected row with {} columns but got {}",
                        self.cols,
                        row.len()
                    )));
                }
                values.push(row);
                self.values = Some(values);
                self.rows += 1;
                Ok(())
            }
        }
    }

    pub fn add_col(&mut self, col: Vec<T>) -> Result<(), MatrixError> {
        match self.values.take() {
            None => {
                self.rows = col.len() as i64;
                let my_vec: Vec<Vec<T>> = col.into_iter().map(|i| vec![i]).collect();
                self.cols += 1;
                self.values = Some(my_vec);
                Ok(())
            }
            Some(values) => {
                if self.rows != (col.len() as i64) {
                    self.values = Some(values);
                    return Err(MatrixError::SizeMismatch(format!(
                        "Expected column with {} rows but got {}",
                        self.rows,
                        col.len()
                    )));
                }
                let my_vec = zip(values, col)
                    .map(|(mut row, value)| {
                        row.push(value);
                        row
                    })
                    .collect();
                self.cols += 1;
                self.values = Some(my_vec);
                Ok(())
            }
        }
    }

    pub fn transpose(&mut self) -> Result<(), MatrixError> {
        match self.values.take() {
            None => Err(MatrixError::EmptyMatrix),
            Some(values) => {
                let mut matrix = Matrix::new();
                values.into_iter().for_each(|row| {
                    matrix.add_col(row).unwrap(); // Safe unwrap: all rows are guaranteed to be equal length
                });
                self.values = matrix.values;
                self.rows = matrix.rows;
                self.cols = matrix.cols;
                Ok(())
            }
        }
    }
}
