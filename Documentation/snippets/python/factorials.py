def factorial(n):
    """Calculate the factorial of a number.

    Args:
        n (int): A non-negative integer to calculate the factorial of.
    Returns:
        int: The factorial of `n`.
    Examples:
        >>> factorial(5)
        120
    """
    if n == 0 or n == 1:
        return 1
    else:
        return n * factorial(n - 1)

num = 5
print(f"The factorial of {num} is {factorial(num)}")
