// iterators4.rs



pub fn factorial(num: u64) -> u64 {
<<<<<<< HEAD
    let array: Vec<u64> = (1..=num).collect();
    array.into_iter().fold(1,|a, b|a * b)
    // Complete this function to return factorial of num
=======
    // Complete this function to return the factorial of num
>>>>>>> 9f61db5dbe38538cf06571fcdd5f806e7901e83a
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
