using DataFrames

function diferentSigns(a::Float64, b::Float64)::Bool
    if a*b < 0
        return true
    else
        return false
    end
end

function falseP(f::Function, x_0::Float64, x_1::Float64, err::Float64, maxIter::Int64)

    results = DataFrame(x_0 = Float64[], x_1 = Float64[], fx_0 = Float64[], fx_1 = Float64[], x_2 = Float64[], f_x_2 = Float64[])

    fx_0 = f(x_0)
    fx_1 = f(x_1)

    iters::Int64 = 0

    #checks if the root is between x_0 and x_1
    if !diferentSigns(fx_0, fx_1)
        throw(ArgumentError("f(x_0) and f(x_1) must have different signs"))
    end

    x_2 = x_1 - (fx_1 * ((x_0 - x_1)/(fx_0 - fx_1)))
    fx_2 = f(x_2)

    push!(results, [x_0, x_1, fx_0, fx_1, x_2, fx_2])

    while abs(fx_2) > err
        iters += 1

        if diferentSigns(fx_0, fx_2)
            x_1 = x_2
            fx_1 = fx_2
        else
            x_0 = x_2
            fx_0 = fx_2
        end

        x_2 = x_1 - (fx_1 * ((x_0 - x_1)/(fx_0 - fx_1)))
        fx_2 = f(x_2)

        push!(results, [x_0, x_1, fx_0, fx_1, x_2, fx_2])

        if iters > maxIter
            break
        end
    end

    return results
end
