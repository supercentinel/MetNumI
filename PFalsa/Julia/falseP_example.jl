include("pfalsa.jl")

function f(x::Float64)::Float64
    return x^3 + x^2 - 4x - 1
end

println(falseP(f, 1.0, 2.0, 0.0005, 100))
