struct Matrix {
    pub data: Vec<Vec<f64>>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            data: vec![vec![0.0; cols]; rows],
            rows,
            cols,
        }
    }

    fn print(&self) {
        for rows in &self.data {
            for cols in rows {
                print!("{} ", cols);
            }
            println!();
        }
    }

    fn scalar_prod(&self, scalar: &f64) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] * scalar;
            }
        }

        return result;
    }

    fn sum(&self, other: &Matrix) -> Matrix {
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

    fn prod(&self, other: &Matrix) -> Matrix {
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

    fn transpose(&self) -> Matrix {
        let mut transpose = Matrix::new(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                transpose.data[j][i] = self.data[i][j];
            }
        }

        return transpose;
    }

    fn minor(&self, _i: usize, _j: usize) -> Matrix {
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

    fn det(&self) -> f64 {
        let mut det = 0.0;

        if self.rows != self.cols {
            panic!("The Matrix needs to be square");
        }

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

    fn cofactor(&self, _i: usize, _j: usize) -> f64 {
        let mut cofactor = 0.0;

        if (_i + _j) % 2 == 0 {
            cofactor = self.minor(_i, _j).det();
        } else {
            cofactor = -self.minor(_i, _j).det();
        }

        return cofactor;
    }

    fn cofactor_matrix(&self) -> Matrix {
        let mut cofactor_matrix = Matrix::new(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                cofactor_matrix.data[i][j] = self.cofactor(i+1, j+1);
            }
        }

        return cofactor_matrix;
    }

    fn inv(&self) -> Matrix {
        let mut inv = Matrix::new(self.rows, self.cols);

        let det = self.det();

        if det == 0.0 {
            panic!("The Matrix is not invertible");
        }

        inv = self.cofactor_matrix().transpose().scalar_prod(&(1.0/det));

        return inv;
    }
}

fn main() {
    let mut a = Matrix::new(4, 4);
    let mut b = Matrix::new(2, 2);

    a.data[0][0] = 1.0;
    a.data[0][1] = -3.0;
    a.data[0][2] = 5.0;
    a.data[0][3] = 6.0;
    a.data[1][0] = 2.0;
    a.data[1][1] = 4.0;
    a.data[1][2] = 0.0;
    a.data[1][3] = 3.0;
    a.data[2][0] = 1.0;
    a.data[2][1] = 5.0;
    a.data[2][2] = 9.0;
    a.data[2][3] = -2.0;
    a.data[3][0] = 4.0;
    a.data[3][1] = 0.0;
    a.data[3][2] = 2.0;
    a.data[3][3] = 7.0;

    b.data[0][0] = 20.0;
    b.data[0][1] = 50.0;
    b.data[1][0] = 15.0;
    b.data[1][1] = 90.0;

    print!("Matrix B:");
    println!(" {} ", a.minor(3, 2).det());

    println!("Matrix A:");
    a.print();
    println!("Det(A): ");
    println!("{}", a.det());
    println!("M_3,2 of A");
    a.minor(2, 4).print();
    println!("C_1,1 of A");
    println!("{}", a.cofactor(2, 4));
    println!("Cofactor Matrix of A");
    a.cofactor_matrix().print();
    println!("Inverse of A");
    a.inv().print();
    println!("A * A^-1 = I");
    a.prod(&a.inv()).print();
}
