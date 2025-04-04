use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum MatrixError {
    #[error("Not a square matix, the number of rows and cols are not the same.")]
    NotSquareMatrix,
    #[error("Can't invert a matrix with a zero determinant.")]
    DetZeroForInverse,
    #[error("Two matrix have different number of rows.")]
    NotTheSameNmrOfRows,
    #[error("Two matrix have different number of cols.")]
    NotTheSameNmrOfCols,
    #[error("Two matrix have different number of cols and rows for mul.")]
    ColsNotEqToRows,
}


#[derive(Clone, Debug)]
pub struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}


// todo, rank, guassa, solve linear system, eigenvalues, etc.

impl Matrix {
    pub fn new(data: Vec<f64>, rows: usize, cols: usize) -> Self {
        Self { data, rows, cols }
    }

    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0.0; rows * cols],
            rows,
            cols,
        }
    }

    pub fn ones(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![1.0; rows * cols],
            rows,
            cols,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] = value;
    }

    pub fn transpose(&self) -> Self {
        let mut res = vec![0.0; self.rows * self.cols];
        for i in 0..self.rows {
            for j in 0..self.cols {
                res[j * self.rows + i] = self.data[i * self.cols + j];
            }
        }
        Self {
            data: res,
            rows: self.cols,
            cols: self.rows,
        }
    }

    pub fn identity_matrix(size: usize) -> Self {
        let mut res = Self::zeros(size, size);
        for i in 0..size {
            res.set(i, i, 1.0);
        }
        res
    }

    pub fn col_norm(&self) -> f64 {
        let mut res: f64 = 0.0;
        for i in 0..self.cols {
            let mut sum = 0.0;
            for j in 0..self.rows {
                sum += self.get(j, i).abs();
            }
            res = res.max(sum);
        }
        res
    }

    pub fn row_norm(&self) -> f64 {
        let mut res: f64 = 0.0;
        for i in 0..self.rows {
            let mut sum = 0.0;
            for j in 0..self.cols {
                sum += self.get(i, j).abs();
            }
            res = res.max(sum);
        }
        res
    }

    pub fn f_norm(&self) -> f64 {
        let mut res: f64 = 0.0;
        for i in 0..self.rows {
            for j in 0..self.cols {
                res += self.get(i, j).powi(2);
            }
        }
        res.sqrt()
    }

    /// Calculate an approximation of the inverse of a matrix using newton's method
    pub fn inverse(&self) -> Result<Matrix, MatrixError> {
        if self.rows != self.cols { return Err(MatrixError::NotSquareMatrix) }
        if self.det()? == 0.0 { return Err(MatrixError::DetZeroForInverse) }

        let mut res_old = self.transpose().matrix_mul_f64(1.0 / (self.col_norm() * self.row_norm()));
        let mut res_new = Self::zeros(self.rows, self.cols);
        let mut diff = 1.0;
        while diff > 1e-6 {
            res_new = res_old.matrix_mul(&(Self::identity_matrix(self.rows).matrix_mul_f64(2.0).matrix_sub(&self.matrix_mul(&res_old)?))?)?;
            diff = res_old.matrix_sub(&res_new)?.f_norm() / res_old.f_norm();
            res_old = res_new.clone();
        }
        Ok(res_new)
    }

    /// Calculate the determinant of a 3x3 matrix
    fn det_3x3(&self) -> f64 {
        self.get(0, 0) * self.get(1, 1) * self.get(2, 2)
            + self.get(0, 1) * self.get(1, 2) * self.get(2, 0)
            + self.get(0, 2) * self.get(1, 0) * self.get(2, 1)
            - self.get(0, 2) * self.get(1, 1) * self.get(2, 0)
            - self.get(0, 1) * self.get(1, 0) * self.get(2, 2)
            - self.get(0, 0) * self.get(1, 2) * self.get(2, 1)
    }

    /// Calculate the determinant of a 2x2 matrix
    fn det_2x2(&self) -> f64 {
        self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0)
    }

    /// Calculate the determinant of a matrix
    pub fn det(&self) -> Result<f64, MatrixError> {
        if self.rows != self.cols { return Err(MatrixError::NotSquareMatrix) }
        if self.rows == 2 {
            return Ok(self.det_2x2());
        } else if self.rows == 3 {
            return Ok(self.det_3x3());
        } else {
            let mut res = 0.0;
            for i in 0..self.cols {
                let mut minor = Self::zeros(self.rows - 1, self.cols - 1);
                for j in 1..self.rows {
                    for k in 0..self.cols {
                        if k < i {
                            minor.set(j - 1, k, self.get(j, k));
                        } else if k > i {
                            minor.set(j - 1, k - 1, self.get(j, k));
                        }
                    }
                }
                res += self.get(0, i) * minor.det()? * if i % 2 == 0 { 1.0 } else { -1.0 };
            }
            Ok(res)
        }
    }

    pub fn matrix_add(&self, rhs: &Matrix) -> Result<Matrix, MatrixError> {
        if self.rows != rhs.rows { return Err(MatrixError::NotTheSameNmrOfRows) }
        if self.cols != rhs.cols { return Err(MatrixError::NotTheSameNmrOfCols) }
        let data: Vec<f64> = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| a + b)
            .collect();
        Ok(Matrix {
            data,
            rows: self.rows,
            cols: self.cols,
        })
    }

    pub fn matrix_sub(&self, rhs: &Matrix) -> Result<Matrix, MatrixError> {
        if self.rows != rhs.rows { return Err(MatrixError::NotTheSameNmrOfRows) }
        if self.cols != rhs.cols { return Err(MatrixError::NotTheSameNmrOfCols) }
        let data: Vec<f64> = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| a - b)
            .collect();
        Ok(Matrix {
            data,
            rows: self.rows,
            cols: self.cols,
        })
    }

    pub fn matrix_mul(&self, rhs: &Matrix) -> Result<Matrix, MatrixError> {
        if self.cols != rhs.rows { return Err(MatrixError::ColsNotEqToRows)}
        let mut res = vec![0.0; self.rows * rhs.cols];
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                for k in 0..self.cols {
                    res[i * rhs.cols + j] +=
                        self.data[i * self.cols + k] * rhs.data[k * rhs.cols + j];
                }
            }
        }
        Ok(Matrix {
            data: res,
            rows: self.rows,
            cols: rhs.cols,
        })
    }

    pub fn matrix_add_f64(&self, rhs: f64) -> Matrix {
        let data: Vec<f64> = self.data.iter().map(|a| a + rhs).collect();
        Matrix {
            data,
            rows: self.rows,
            cols: self.cols,
        }
    }

    pub fn matrix_sub_f64(&self, rhs: f64) -> Matrix {
        let data: Vec<f64> = self.data.iter().map(|a| a - rhs).collect();
        Matrix {
            data,
            rows: self.rows,
            cols: self.cols,
        }
    }

    pub fn matrix_mul_f64(&self, rhs: f64) -> Matrix {
        let data: Vec<f64> = self.data.iter().map(|a| a * rhs).collect();
        Matrix {
            data,
            rows: self.rows,
            cols: self.cols,
        }
    }

    pub fn matrix_div_f64(&self, rhs: f64) -> Matrix {
        let data: Vec<f64> = self.data.iter().map(|a| a / rhs).collect();
        Matrix {
            data,
            rows: self.rows,
            cols: self.cols,
        }
    }

}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{:.5} ", self.get(i, j))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
