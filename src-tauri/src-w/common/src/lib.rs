pub trait ToStringErr<T> {
    /// Converts a Result<T, E> into a Result<T, String> by converting the error E into a String.
    fn string_err(self) -> Result<T, String>;
}

impl<T, E: std::fmt::Display> ToStringErr<T> for Result<T, E> {
    fn string_err(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}
