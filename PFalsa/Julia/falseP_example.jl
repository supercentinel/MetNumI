using Plots

include("pfalsa.jl")

function f(x::Float64)::Float64
    return x^3 + x^2 - 4x - 1
end

results = falseP(f, 1.0, 2.0, 0.0005, 100)

println(results)


plot(f, -2, 2, label="f(x) = x^3 + x^2 - 4x - 1", legend=:topleft, show=true)
ylims!(-2, 2)
vline!([0], color=:black, line=:dash)
hline!([0], color=:black, line=:dash)
scatter!([results.x_2[size(results.x_2, 1)]], [0], label="Root", color=:red)


readline()

println()
