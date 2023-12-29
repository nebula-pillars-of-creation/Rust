def reverse_string(s: str) -> str:
    """Reverses a given string.

    Args:
        s: A string to be reversed.

    Returns:
        A string that is the reverse of the input.

    Raises:
        ValueError: If the input is not a string.
    """
    if not isinstance(s, str):
        raise ValueError("Input must be a string.")

    return s[::-1]

# Example usage:
try:
    original_string = "hello"
    reversed_string = reverse_string(original_string)
    print(f"Reversed string: {reversed_string}")
except ValueError as e:
    print(e)

#####################################################################

def reverse_string(s: str) -> str:
    """Reverses a given string without using standard libraries.

    Args:
        s: A string to be reversed.

    Returns:
        A string that is the reverse of the input.
    """
    reversed_string = ""
    for i in range(len(s) - 1, -1, -1):
        reversed_string += s[i]
    return reversed_string

# Example usage:
original_string = "hello"
reversed_string = reverse_string(original_string)
print(f"Reversed string: {reversed_string}")
