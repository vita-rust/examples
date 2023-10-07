#[cfg(test)]
mod tests {
    use std::panic;

    #[test]
    pub fn test_unwind() {
        let result = panic::catch_unwind(|| std::hint::black_box(()));
        assert!(result.is_ok(), "panic shouldn't happen");

        let result = panic::catch_unwind(|| {
            panic!("oh no!");
        });

        assert!(result.is_err(), "panic should be handled");
    }
}
