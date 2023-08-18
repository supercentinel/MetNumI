function diferentSigns(a::Float64, b::Float64)
    if a*b < 0
        return true
    else
        return false
    end
end

function biseccion(f::Function, a::Float64, b::Float64, err::Float64, maxIter::Int64)::Float64
    fa = f(a)
    fb = f(b)

    iters::Int64 = 0;

    #checks if f(a) and f(b) have different signs
    if !diferentSigns(fa, fb)
        throw(ArgumentError("f(a) and f(b) must have different signs"))
    end

    p = (a + b)/2
    fp = f(p)

    while abs(fp) > err
        iters += 1
        if diferentSigns(fa, fp)
            b = p
        else
            a = p
        end

        p = (a + b)/2
        fp = f(p)

        if iters >= maxIter
            break
        end
    end

    println("Iterations: ", iters)
    println("Error: ", fp)
    return p
end

function f(x::Float64)::Float64
    return x^2 - 2
end

println(biseccion(f, -2.0, -1.0, 0.0005, 32))

