#include "StringReversal.h"

std::string reverse_string(const std::string& s) {
    if (s.empty()) {
        throw std::invalid_argument("Input string must not be empty.");
    }

    std::string reversed(s.rbegin(), s.rend());
    return reversed;
}

// Example usage:
int main() {
    try {
        std::string original_string = "hello";
        std::string reversed_string = reverse_string(original_string);
        std::cout << "Reversed string: " << reversed_string << std::endl;
    } catch (const std::invalid_argument& e) {
        std::cerr << e.what() << std::endl;
    }

    return 0;
}

/////////////////////////////////////////////////////////////////////
#include <iostream>

void reverse_string(char *s) {
    // Calculate the length of the string.
    int length = 0;
    while (s[length] != '\0') {
        ++length;
    }

    // Reverse the string in place.
    for (int i = 0; i < length / 2; ++i) {
        char temp = s[i];
        s[i] = s[length - i - 1];
        s[length - i - 1] = temp;
    }
}

// Example usage:
int main() {
    char original_string[] = "hello";
    reverse_string(original_string);
    std::cout << "Reversed string: " << original_string << std::endl;
    return 0;
}
