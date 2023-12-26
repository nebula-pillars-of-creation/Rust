/// Calculates the factorial of a number.
///
/// Given a non-negative integer `n`, this function returns the factorial of `n`,
/// which is the product of all positive integers less than or equal to `n`.
///
/// # Examples
///
/// ```
/// let five_factorial = factorial(5);
/// assert_eq!(five_factorial, 120);
/// ```
///
/// # Arguments
///
/// * `n` - A u64 non-negative integer
///
/// # Returns
///
/// * A u64 representing the factorial of `n`

fn factorial(n: u64) -> u64 {
  if n <= 1 {
      1
  } else {
      n * factorial(n - 1)
  }
}

fn main() {
  let num = 5;
  println!("The factorial of {} is {}", num, factorial(num));
}
