#include <iostream> //for I/O

typedef unsigned long long ull; //save time typing; this can also be written as `using ull = unsigned long long;`

//since we're only operating over the range [1,20], we're correctly defining our constant limits
//--this avoids erroneously using 0 in the range, or other undesired behavior.
const ull RANGE_MIN = 1;
const ull RANGE_MAX = 20;

// Function to calculate GCD using the Euclidean algorithm
ull gcd(ull a, ull b) {
    while (b != 0) {
        ull temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

// Function to calculate LCM using GCD
ull lcm(ull a, ull b) {
    return (a * b) / gcd(a, b);
}

int main ()
{
  ull range_lcm = RANGE_MIN;
  for (ull i = RANGE_MIN; i <= RANGE_MAX ; i++) {
      range_lcm = lcm(range_lcm, i); //fold LCM over range
  }
  std::cout << "LCM of range: " << range_lcm << std::endl;
  return 0;
}
