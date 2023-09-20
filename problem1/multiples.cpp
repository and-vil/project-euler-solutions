#include <iostream>

int main() {
    unsigned int sum = 0;
    for (short i = 0; i < 1000; i++) {
        sum += (i%3 == 0 || i%5 == 0)? i: 0;
    }
    std::cout << sum << std::endl;
    return 0;
}
