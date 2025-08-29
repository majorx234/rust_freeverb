pub mod parameters;
pub mod widget;

use crate::parameters::ParameterProvider;

pub trait CommandHandler {}

pub trait AudioProcessor: CommandHandler {
    fn process_stereo(&mut self, input: &[f32], output: &[f32]);
}

pub trait AudioModule: ParameterProvider {
    type Processor: AudioProcessor;
}
