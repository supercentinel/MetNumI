function Crout(A::Array{Float64}, X::Array{Float64})
    n = size(A)[1]

    L = zeros(size(A))
    U = zeros(size(A))

    for i in 1:n
    U[i, i] = 1.0
    end

    for j ∈ 1:n
        # L Factorization
        for i ∈ j:n
            ∑_L = 0.0

            for k ∈ 1:j
                ∑_L += L[i, k] * U[k, j]
            end

            L[i, j] = A[i, j] - ∑_L
        end

        # U Factorization
        for i ∈ j+1:n
            ∑_U = 0.0

            for k ∈ 1:j
                ∑_U += L[j, k] * U[k, i]
            end

            U[j, i] = (A[j, i] - ∑_U) / L[j, j]
        end

    end


    # Forward Substitution
    Z = zeros(size(A)[1])

    for i ∈ 1:size(X)[1]
        Z[i] = (X[i] - sum(L[i, 1:i-1] .* Z[1:i-1])) / L[i, i]
    end

    # Backward Substitution
    Sol = zeros(size(A)[1])

    for i ∈ size(A)[1]:-1:1
        Sol[i] = (Z[i] - sum(U[i, i+1:size(A)[1]] .* Sol[i+1:size(A)[1]])) / U[i, i]
    end

    return Sol
end
