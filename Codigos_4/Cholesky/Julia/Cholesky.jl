function Cholesky(A::Array{Float64}, X::Array{Float64})
    L = zeros(size(A))

    # Factorization
    for i ∈ 1:size(A)[1]
        for j ∈ 1:i
            if i == j
                L[i, j] = sqrt(A[i, j] - sum(L[i, 1:j-1].^2))
            else
                L[i, j] = (A[i, j] - sum(L[i, 1:j-1] .* L[j, 1:j-1])) / L[j, j]
            end
        end
    end

    # Forward substitution
    Z = zeros(size(X))

    for i ∈ 1:size(X)[1]
        Z[i] = (X[i] - sum(L[i, 1:i-1] .* Z[1:i-1])) / L[i, i]
    end

    # Backward substitution
    LT = transpose(L)
    Solution = zeros(size(X))

    for i ∈ size(X)[1]:-1:1
        Solution[i] = (Z[i] - sum(LT[i, i+1:size(X)[1]] .* Solution[i+1:size(X)[1]])) / LT[i, i]
    end


    return Solution
end
