#include <iostream>;

/**
* Calculates the factorial of a number.
*
* @param n A non-negative integer to calculate the factorial of.
* @return The factorial of `n`.
*
* Example:
*   auto result = factorial(5); // result is 120
*/

unsigned long long factorial(unsigned long long n) {
    if (n == 0 || n == 1) return 1;
    else return n * factorial(n - 1);
}

int main() {
    unsigned long long num = 5;
    std::cout << "The factorial of " << num << " is " << factorial(num) << std::endl;
    return 0;
}
