using DataFrames

function secant(f::Function, x_0::Float64, x_1::Float64, iterations::Int64, absolute_error::Float64)::DataFrame

    x_km1 = x_0
    x_k = x_1
    x_kp1 = 0.0
    fx_k = 0.0
    fx_km1 = 0.0
    aError = 0.0
    rError = 0.0
    pError = 0.0

    iters::Int64 = 0

    results = DataFrame(x_km1 = Float64[],
                        fx_km1 = Float64[],
                        x_k = Float64[],
                        fx_k = Float64[],
                        x_kp1 = Float64[],
                        absolute_error = Float64[],
                        relative_error = Float64[],
                        porcentual_error = Float64[])

    while true

        fx_k = f(x_k)
        fx_km1 = f(x_km1)

        x_kp1 = x_k - (fx_k * (x_km1 - x_k))/(fx_km1 - fx_k)

        aError = abs(x_k - x_km1)
        rError = aError / abs(x_k)
        pError = rError * 100

        push!(results, [x_km1, fx_km1, x_k, fx_k, x_kp1, aError, rError, pError])

        if iters >= iterations
            break
        end
        if aError <= absolute_error
            break
        end

        iters += 1

        x_km1 = x_k
        x_k = x_kp1
    end

    return results
end
