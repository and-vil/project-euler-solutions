#include <iostream>
#include <vector>

//could use some `typedef`s or `using`s to save typing on unsigned shorts and longs, but didn't think to do it here

unsigned long int countPaths(unsigned short int n) {
    // Create a 2D vector to represent the grid
    std::vector<std::vector<unsigned long int>> grid(n, std::vector<unsigned long int>(n, 0));

    // Initialize first row and first column to 1
    for (unsigned short int i = 0; i < n; i++) {
        grid[i][0] = 1;
        grid[0][i] = 1;
    }

    // Calculate number of paths per cell
    for (unsigned short int i = 1; i < n; i++) {
        for (unsigned short int j = 1; j < n; j++) {
            grid[i][j] = grid[i - 1][j] + grid[i][j - 1];
        }
    }

    // The bottom-right cell contains total path count
    return grid[n - 1][n - 1];
}

int main() {
    unsigned short int n = 20; // desired grid size
    unsigned long int paths = countPaths(n+1); //account for actual grid size with +1 offset

    std::cout << "Number of paths in a " << n << "x" << n << " grid: " << paths << std::endl;

    return 0;
}
