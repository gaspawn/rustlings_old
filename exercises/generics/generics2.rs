// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

<<<<<<< HEAD

struct Wrapper<T> {
    value: T
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
=======
// I AM NOT DONE
struct Wrapper {
    value: u32
}

impl Wrapper {
    pub fn new(value: u32) -> Self {
>>>>>>> 9f61db5dbe38538cf06571fcdd5f806e7901e83a
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value,  42);
    }

    #[test]
    fn store_str_in_wrapper() {
<<<<<<< HEAD
        // TODO: Delete this assert and uncomment the one  below once you have  finished the exercise.
        //assert!(false);
         assert_eq!(Wrapper::new("Foo").value, "Foo");
=======
        assert_eq!(Wrapper::new("Foo").value, "Foo");
>>>>>>> 9f61db5dbe38538cf06571fcdd5f806e7901e83a
    }
}