pub trait Validate {
    fn validate(&self) -> Result<(), Vec<&'static str>>;
}
