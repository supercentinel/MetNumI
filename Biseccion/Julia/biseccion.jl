using DataFrames

function diferentSigns(a::Float64, b::Float64)
    if a*b < 0
        return true
    else
        return false
    end
end

function biseccion(f::Function, a::Float64, b::Float64, err::Float64, maxIter::Int64)::DataFrame
    fa = f(a)
    fb = f(b)

    iters::Int64 = 0;

    results = DataFrame(a = Float64[], b = Float64[], fa = Float64[], fb = Float64[], p = Float64[], fp = Float64[])

    #checks if f(a) and f(b) have different signs
    if !diferentSigns(fa, fb)
        throw(ArgumentError("f(a) and f(b) must have different signs"))
    end

    p = (a + b)/2
    fp = f(p)

    push!(results, [a, b, fa, fb, p, fp])

    while abs(fp) > err
        iters += 1
        if diferentSigns(fa, fp)
            b = p
        else
            a = p
        end

        p = (a + b)/2
        fa = f(a)
        fb = f(b)
        fp = f(p)

        push!(results, [a, b, fa, fb, p, fp])

        if iters >= maxIter
            break
        end
    end

    return results
end
