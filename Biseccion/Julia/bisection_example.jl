include("biseccion.jl")

function f(x::Float64)::Float64
    return x^2 - 2
end

println(biseccion(f, -2.0, -1.0, 0.0005, 32))
