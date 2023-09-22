#include <algorithm>
#include <iostream>
#include <vector>

std::vector<std::vector<unsigned int>> matrixMult2D(const std::vector<std::vector<unsigned int>>& A, const std::vector<std::vector<unsigned int>>& B) {
  // Check if the matrices have compatible dimensions
  if (A[0].size() != B.size()) {
    return {};
  }

  // Create the result matrix
  std::vector<std::vector<unsigned int>> C(A.size(), std::vector<unsigned int>(B[0].size()));

  // Multiply the matrices
  for (int i = 0; i < A.size(); i++) {
    for (int j = 0; j < B[0].size(); j++) {
      for (int k = 0; k < A[0].size(); k++) {
        C[i][j] += A[i][k] * B[k][j];
      }
    }
  }

  return C;
}

std::vector<std::vector<unsigned int>> createFibonacciBasisMatrix() {
    std::vector<std::vector<unsigned int>> fibonacciBasis = {
        {1, 1},
        {1, 0}
    };
    return fibonacciBasis;
}

int main() {
    std::vector<std::vector<unsigned int>> fibn_basis = createFibonacciBasisMatrix();
    std::vector<std::vector<unsigned int>> fibn_iter = createFibonacciBasisMatrix();
    unsigned int sum = 0;
    while (fibn_iter[0][0] < 4000000) {
        fibn_iter = matrixMult2D(fibn_basis, fibn_iter);
        sum += (fibn_iter[0][0]%2 == 0) ? fibn_iter[0][0] : 0;
    }
    std::cout << sum << std::endl;
    return 0;
}
