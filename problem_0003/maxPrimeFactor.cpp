#include <algorithm> //for std::max_element
#include <iostream>  //for I/O
#include <vector>    //for std::vector<T>

unsigned long long maxPrimeFactor(unsigned long long n)
{
    std::vector<unsigned long long> factors;
    //iterate from 1st prime at 2 up to sqrt(n).
    //Note: postfix increment is needed if 2 is a prime factor, and is also ISO CPP std.
    for (unsigned long long p = 2; p * p <= n; p++) {
        while (n % p == 0) {
            factors.push_back(p);
            n /= p;
        }
    }
    if (n > 1) {
        factors.push_back(n); //if anything remains after division iterations, it's the last remaining factor.
    }
    //Note: it may be possible to coerce factors into smaller data types (e.g. unsigned long, unsigned int) but isn't necessary.
    return *std::max_element(std::begin(factors), std::end(factors)); //deref and return max elem. in vector.
}

int main ()
{
  unsigned long long number = 600851475143ULL;
  unsigned long long factor = maxPrimeFactor(number);
  std::cout << "Max factor of " << number << ": " << factor << std::endl;
  return 0;
}
