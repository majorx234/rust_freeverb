pub mod parameters;
pub mod widget;

pub use parameters::ParameterProvider;
pub use widget::Widget;

pub enum Command {
    SetParameter(usize, f32),
}

pub trait CommandHandler {
    fn handle_command(&mut self, command: Command);
}

pub trait AudioProcessor: CommandHandler + Send + Sync + 'static {
    fn process_stereo(
        &mut self,
        input_l: &[f32],
        input_r: &[f32],
        output_l: &mut [f32],
        output_r: &mut [f32],
    );
}

pub trait AudioModule: ParameterProvider {
    type Processor: AudioProcessor;

    fn create_processor(sample_rate: usize) -> Self::Processor;
    fn name() -> String;
}
