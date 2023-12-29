#include <iostream>
#include <vector>

std::vector<int> generate_fibonacci_sequence(int n) {
    std::vector<int> sequence;
    if (n >= 1) {
        sequence.push_back(0);
    }
    if (n >= 2) {
        sequence.push_back(1);
    }

    for (int i = 2; i < n; ++i) {
        sequence.push_back(sequence[i - 1] + sequence[i - 2]);
    }

    return sequence;
}

// Example usage:
int main() {
    int n = 10;
    std::vector<int> sequence = generate_fibonacci_sequence(n);
    std::cout << "The Fibonacci sequence up to the " << n << "-th number is: ";
    for (int num : sequence) {
        std::cout << num << " ";
    }
    std::cout << std::endl;
    return 0;
}
