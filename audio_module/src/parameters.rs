pub trait Parameters {
    fn name(&self) -> String;
}

pub trait ParameterProvider {
    fn parameter_count() -> usize;
    fn parameter(id: usize) -> Box<dyn Parameters>;
}
