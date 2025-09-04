pub mod parameters;
pub mod widget;

use crate::parameters::ParameterProvider;

pub enum Command {
    SetParameter(usize, f32),
}

pub trait CommandHandler {
    fn handle_command(&mut self, command: Command);
}

pub trait AudioProcessor: CommandHandler {
    fn process_stereo(&mut self, input: &[f32], output: &mut [f32]);
}

pub trait AudioModule: ParameterProvider {
    type Processor: AudioProcessor;

    fn create_processor(sample_rate: usize) -> Self::Processor;
}
