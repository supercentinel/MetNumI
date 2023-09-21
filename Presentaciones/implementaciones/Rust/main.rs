mod matrix;

use std::io;
use matrix::Matrix;


fn main() {
  println!("Inverse of a Matrix and Sistem equation solver");
  println!("1. Calculate the inverse of the matrix in the file Inverse.txt");
  println!("2. Solve the system equation for the matrices SisEq.txt with the b.txt vector");

  let mut input = String::new();

  io::stdin().read_line(&mut input)
      .expect("failed to read line");

  let option: i32 = input.trim().parse().expect("Number");


  if option == 1 {

    let a = Matrix::read_from_file("./matrices/Inverse2.txt");

    println!("Matrix A:");
    a.print();
    println!("Matrix A^-1:");
    a.inv().print();
    println!("A * A^-1 = I: ");
    a.prod(&a.inv()).print();
  } else if option == 2 {

    let sis_eq = Matrix::read_from_file("./matrices/SisEq.txt");
    let b = Matrix::read_from_file("./matrices/b.txt");

    let solution = sis_eq.inv().prod(&b);

    println!("Coeficents matrix: ");
    sis_eq.print();
    println!("b: ");
    b.print();
    println!("Solution :");
    solution.print()

  } else {
    println!("Choose one or two!");
  }
}
