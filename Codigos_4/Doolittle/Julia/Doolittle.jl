function Doolittle(matrix::Array{Float64}, X::Array{Float64})::Array{Float64}
    L = zeros(size(matrix))
    U = zeros(size(matrix))

    #LU Factorization
    for i ∈ 1:size(matrix)[1]
        for j ∈ 1:i
            if i >= 2
                ∑_1 = 0
                ∑_2 = 0

                #∑_1
                for k ∈ 1:i-2
                    ∑_1 += L[i,k]*U[k,j]
                end

                #∑_2
                for k ∈ 1:i-1
                    ∑_2 += L[i,k]*U[k,j]
                end

                L[i,j] = (A[i,j] - ∑_1)/U[j,j]
                U[j,i] = (A[i,j] - ∑_2)
            end

            U[1,j] = A[1,j]
            L[j,j] = 1
        end
    end

    #Forward Substitution
    Z = zeros(size(X))

    for i ∈ 1:size(Z)[1]
        ∑ = 0
        for j ∈ 1:i-1
            ∑ += L[i,j]*Z[j]
        end
        Z[i] = (X[i] - ∑)/L[i,i]
    end

    #Backward Substitution
    Solution = zeros(size(X))

    for i ∈ size(Solution)[1]:-1:1
        ∑ = 0
        for j ∈ i+1:size(Solution)[1]
            ∑ += U[i,j]*Solution[j]
        end
        Solution[i] = (Z[i] - ∑)/U[i,i]
    end

    return Solution
end
