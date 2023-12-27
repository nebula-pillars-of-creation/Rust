def generate_fibonacci_sequence(n: int) -> list:
    """Generates the Fibonacci sequence up to the n-th number.

    Args:
        n: The length of the Fibonacci sequence.

    Returns:
        A list containing the Fibonacci sequence up to the n-th number.
    """
    if n <= 0:
        return []
    sequence = [0, 1]
    while len(sequence) < n:
        sequence.append(sequence[-1] + sequence[-2])
    return sequence[:n]

# Example usage:
n = 10
print(f"The Fibonacci sequence up to the {n}-th number is: {generate_fibonacci_sequence(n)}")
