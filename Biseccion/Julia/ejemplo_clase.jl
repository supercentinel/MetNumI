using Plots

include("biseccion.jl")

function f(x::Float64)
    return x^4 + 7*x^3 - 7
end

function f_2(x::Float64)
    return x^4 + 3*x^2 - 2
end

results = biseccion(f_2, 0.0, 1.0, 0.0005, 100)

println(results)


plot(f_2, -1, 1, label="f(x)", show=true, xlabel="x", ylabel="y")
ylims!(-2, 2)
vline!([0], label="x axis", line=:dash, color="black")
hline!([0], label="y axis", line=:dash, color="black")
scatter!([results.p[size(results.p, 1)]], [0], label="root", color="red")

readline()

#println(biseccion(f, 0.0, 1.0, 0.005, 100))

