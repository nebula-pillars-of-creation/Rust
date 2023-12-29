#ifndef STRING_REVERSAL_H
#define STRING_REVERSAL_H

#include <string>
#include <stdexcept>

/**
 * Reverses a given string.
 *
 * @param s A string to be reversed.
 * @return A string that is the reverse of the input.
 * @throw std::invalid_argument If the input is not valid.
 */
std::string reverse_string(const std::string& s);

#endif // STRING_REVERSAL_H
