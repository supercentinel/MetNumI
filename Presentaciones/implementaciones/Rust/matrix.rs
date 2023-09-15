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

    fn scalar_mul(&self, scalar: &f64) -> Matrix {
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

                // Horrible nesting
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

        if self.rows > 3 {
            det = 0.0;
        }

        return det;
    }

//     fn transpose(&self) -> Matrix {
//     }

//     fn inv(&self) -> Matrix {
//     }
}

fn main() {
    let mut a = Matrix::new(3, 3);
    let mut b = Matrix::new(2, 2);

    a.data[0][0] = 1.0;
    a.data[0][1] = 2.0;
    a.data[0][2] = 3.0;
    a.data[1][0] = 4.0;
    a.data[1][1] = 5.0;
    a.data[1][2] = 6.0;
    a.data[2][0] = 8.0;
    a.data[2][1] = 10.0;
    a.data[2][2] = 10.0;


    b.data[0][0] = 20.0;
    b.data[0][1] = 50.0;
    b.data[1][0] = 15.0;
    b.data[1][1] = 90.0;

    println!("Matrix A:");
    a.print();
    println!("M_1,1 of A");
    a.minor(2, 2).print();
}
