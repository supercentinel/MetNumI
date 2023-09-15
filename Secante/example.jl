include("secant.jl")

using Plots

function f(x::Float64)::Float64
    return x^5 - 3*x^4 + 2
end

results = secant(f, 0.8, 1.2, 10, 0.0001)
root = results.x_kp1[size(results.x_kp1, 1)]

println(results)
println("Root: ", root)

plot(f, -1.0, 5.0, label="f(x)", show=true, xlabel="x", ylabel="y", size=(800, 600))
ylims!(-10, 10)
vline!([0], label="x-axis", line=:dash, color=:black)
hline!([0], label="y-axis", line=:dash, color=:black)
scatter!([results.x_kp1[size(results.x_kp1, 1)]], [0], label="secant method root", color="red")

readline()
