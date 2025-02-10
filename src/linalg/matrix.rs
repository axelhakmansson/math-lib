#[derive(Clone, Debug)]
pub struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

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
    pub fn inverse(&self) -> Self {
        assert_eq!(self.rows, self.cols);
        assert_ne!(self.det(), 0.0);
        let mut res_old = self.transpose() * (1.0 / (self.col_norm() * self.row_norm()));
        let mut res_new = Self::zeros(self.rows, self.cols);
        let mut diff = 1.0;
        while diff > 1e-6 {
            res_new = &res_old * &(Self::identity_matrix(self.rows) * 2.0 - self * &res_old);
            diff = (&res_old - &res_new).f_norm() / res_old.f_norm();
            res_old = res_new.clone();
        }
        res_new
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
    pub fn det(&self) -> f64 {
        assert_eq!(self.rows, self.cols);
        if self.rows == 2 {
            return self.det_2x2();
        } else if self.rows == 3 {
            return self.det_3x3();
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
                res += self.get(0, i) * minor.det() * if i % 2 == 0 { 1.0 } else { -1.0 };
            }
            res
        }
    }
}

impl std::ops::Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);
        let data: Vec<f64> = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| a + b)
            .collect();
        Self {
            data,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl std::ops::Add<f64> for Matrix {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        let data: Vec<f64> = self.data.iter().map(|a| a + rhs).collect();
        Self {
            data,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl std::ops::Sub for Matrix {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);
        let data: Vec<f64> = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| a - b)
            .collect();
        Self {
            data,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl std::ops::Sub<&Matrix> for &Matrix {
    type Output = Matrix;

    fn sub(self, rhs: &Matrix) -> Self::Output {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);
        let data: Vec<f64> = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| a - b)
            .collect();
        Matrix {
            data,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl std::ops::Sub<f64> for Matrix {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        let data: Vec<f64> = self.data.iter().map(|a| a - rhs).collect();
        Self {
            data,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl std::ops::Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.cols, rhs.rows);
        let mut res = vec![0.0; self.rows * rhs.cols];
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                for k in 0..self.cols {
                    res[i * rhs.cols + j] +=
                        self.data[i * self.cols + k] * rhs.data[k * rhs.cols + j];
                }
            }
        }
        Self {
            data: res,
            rows: self.rows,
            cols: rhs.cols,
        }
    }
}

impl std::ops::Mul<&Matrix> for Matrix {
    type Output = Self;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        assert_eq!(self.cols, rhs.rows);
        let mut res = vec![0.0; self.rows * rhs.cols];
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                for k in 0..self.cols {
                    res[i * rhs.cols + j] +=
                        self.data[i * self.cols + k] * rhs.data[k * rhs.cols + j];
                }
            }
        }
        Self {
            data: res,
            rows: self.rows,
            cols: rhs.cols,
        }
    }
}

impl std::ops::Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        assert_eq!(self.cols, rhs.rows);
        let mut res = vec![0.0; self.rows * rhs.cols];
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                for k in 0..self.cols {
                    res[i * rhs.cols + j] +=
                        self.data[i * self.cols + k] * rhs.data[k * rhs.cols + j];
                }
            }
        }
        Matrix {
            data: res,
            rows: self.rows,
            cols: rhs.cols,
        }
    }
}

impl std::ops::Mul<f64> for Matrix {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let data: Vec<f64> = self.data.iter().map(|a| a * rhs).collect();
        Self {
            data,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl std::ops::Mul<f64> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
        let data: Vec<f64> = self.data.iter().map(|a| a * rhs).collect();
        Matrix {
            data,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl std::ops::Div<f64> for Matrix {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let data: Vec<f64> = self.data.iter().map(|a| a / rhs).collect();
        Self {
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
