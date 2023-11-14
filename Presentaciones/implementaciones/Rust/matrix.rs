#![allow(dead_code)]
use std::fs;

#[derive(Debug)]
pub struct Matrix {
    pub data: Vec<Vec<f64>>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            data: vec![vec![0.0; cols]; rows],
            rows,
            cols,
        }
    }

    pub fn print(&self) {
        for rows in &self.data {
            for cols in rows {
                print!("{} ", cols);
            }
            println!();
        }
    }

    //Stolen form https://github.com/robertfeliciano/linear-rustgebra/blob/main/src/lib.rs
    pub fn read_from_file(path: &str) -> Matrix {
        let content: String = fs::read_to_string(path).unwrap_or_else(|_e| panic!("{}", "{e}"));
        let mut matrix: Vec<Vec<f64>> = Vec::new();

        for rows in content.lines() {
            let mut row: Vec<f64> = Vec::new();
            let entries: Vec<&str> = rows.split_whitespace().collect();

            for entry in entries {
                row.push(entry.parse::<f64>().unwrap());
            }

            matrix.push(row);
        }

        let rows = matrix.len();
        let cols = matrix[0].len();

        return Matrix {
            data: matrix,
            rows,
            cols,
        };
    }

    pub fn scalar_prod(&self, scalar: &f64) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] * scalar;
            }
        }

        return result;
    }

    pub fn sum(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.rows {
            panic!("The Matrices need to be of the same dimensions");
        }

        let mut sum = Matrix::new(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                sum.data[i][j] = self.data[i][j] + other.data[i][j]
            }
        }

        return sum;
    }

    pub fn prod(&self, other: &Matrix) -> Matrix {
        let mut prod = Matrix::new(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    prod.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }

        return prod;
    }

    pub fn transpose(&self) -> Matrix {
        let mut transpose = Matrix::new(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                transpose.data[j][i] = self.data[i][j];
            }
        }

        return transpose;
    }

    //TODO Add check for square matrix
    pub fn minor(&self, _i: usize, _j: usize) -> Matrix {
        if self.cols == 1 || self.rows == 1 {
            panic!("The 1x1 matrix does not have minors");
        }

        let mut minor_matrix = Matrix::new(self.cols - 1, self.rows - 1);

        for i in 0..self.rows {
            if i == _i-1 {
                continue;
            }

            for j in 0..self.cols {
                if j == _j-1 {
                    continue;
                }

                // Horrible nesting. written by Copilot
                if i < _i-1 && j < _j-1 {
                    minor_matrix.data[i][j] = self.data[i][j];
                } else {
                    if i < _i-1 && j > _j-1 {
                        minor_matrix.data[i][j-1] = self.data[i][j];
                    } else {
                        if i > _i-1 && j < _j-1 {
                            minor_matrix.data[i-1][j] = self.data[i][j];
                        } else {
                            if i > _i-1 && j > _j-1 {
                                minor_matrix.data[i-1][j-1] = self.data[i][j];
                            }
                        }
                    }
                }
            }
        }

        return minor_matrix;
    }

    pub fn det(&self) -> f64 {
        let mut det = 0.0;

        if self.rows != self.cols {
            panic!("The Matrix needs to be square");
        }
        //TODO Refactor this to a switch statement
        if self.rows == 1 {
            det = self.data[0][0];
        }

        if self.rows == 2 {
            det = self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0];
        }

        if self.rows == 3 {
            det = (self.data[0][0] * self.minor(1, 1).det())
                - (self.data[0][1] * self.minor(1, 2).det())
                + (self.data[0][2] * self.minor(1, 3).det());
        }

        if self.rows > 3 {
            for i in 0..self.cols {
                det += self.data[0][i] * self.cofactor(1, i+1);
            }
        }

        return det;
    }

    pub fn cofactor(&self, _i: usize, _j: usize) -> f64 {
        let cofactor: f64;

        if (_i + _j) % 2 == 0 {
            cofactor = self.minor(_i, _j).det();
        } else {
            cofactor = -self.minor(_i, _j).det();
        }

        return cofactor;
    }

    pub fn cofactor_matrix(&self) -> Matrix {
        let mut cofactor_matrix = Matrix::new(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                cofactor_matrix.data[i][j] = self.cofactor(i+1, j+1);
            }
        }

        return cofactor_matrix;
    }

    pub fn inv(&self) -> Matrix {
        let inv: Matrix;

        let det = self.det();

        if det == 0.0 {
            panic!("The Matrix is not invertible");
        }

        inv = self.cofactor_matrix().transpose().scalar_prod(&(1.0/det));

        return inv;
    }
}
