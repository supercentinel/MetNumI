include("Cholesky.jl")

A = [8.0 3.0 -2.0 1.0;
     3.0 9.0 1.0 4.0;
     -2.0 1.0 9.0 3.0;
     1.0 4.0 3.0 10.0]

X = [-2.0;
     11.0;
     -22.0;
     20.0]

display(Cholesky(A, X))
