use audio_module::{
    parameters::{BoolParameter, FloatParameter, Parameter, ParameterProvider},
    AudioModule, AudioProcessor, Command, CommandHandler,
};
use freeverb::Freeverb;
use num_enum::{FromPrimitive, IntoPrimitive};
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(FromPrimitive, IntoPrimitive, EnumCountMacro)]
#[repr(usize)]
pub enum FreeverbParameters {
    #[num_enum(default)]
    Dampening,
    Width,
    RoomSize,
    Freeze,
    Dry,
    Wet,
}

pub struct FreeverbProcessor {
    freeverb: Freeverb,
}

impl FreeverbProcessor {
    fn new(sample_rate: usize) -> Self {
        Self {
            freeverb: Freeverb::new(sample_rate),
        }
    }
}

impl CommandHandler for FreeverbProcessor {
    fn handle_command(&mut self, command: Command) {
        match command {
            Command::SetParameter(id, value) => match FreeverbParameters::from(id) {
                FreeverbParameters::Dampening => {
                    self.freeverb.set_dampening(value as f64);
                }
                FreeverbParameters::Width => {
                    self.freeverb.set_width(value as f64);
                }
                FreeverbParameters::RoomSize => {
                    self.freeverb.set_room_size(value as f64);
                }
                FreeverbParameters::Freeze => {
                    self.freeverb.set_freeze(value != 0.0);
                }
                FreeverbParameters::Dry => {
                    self.freeverb.set_dry(value as f64);
                }
                FreeverbParameters::Wet => {
                    self.freeverb.set_wet(value as f64);
                }
            },
        }
    }
}

impl AudioProcessor for FreeverbProcessor {
    fn process_stereo(&mut self, input: &[f32], output: &mut [f32]) {
        assert!(input.len() == output.len());

        for (in_vec, out_vec) in input.chunks(2).zip(output.chunks_mut(2)) {
            let result = self.freeverb.tick((in_vec[0] as f64, in_vec[1] as f64));
            out_vec[0] = result.0 as f32;
            out_vec[1] = result.1 as f32;
        }
    }
}

struct FreeverbModule {}

impl ParameterProvider for FreeverbModule {
    fn parameter_count() -> usize {
        FreeverbParameters::COUNT
    }

    fn parameter(id: usize) -> Box<dyn Parameter> {
        match FreeverbParameters::from(id) {
            FreeverbParameters::Dampening => {
                Box::new(FloatParameter::new("Dampening").default_user_value(0.5))
            }
            FreeverbParameters::Width => {
                Box::new(FloatParameter::new("Width").default_user_value(0.5))
            }
            FreeverbParameters::RoomSize => {
                Box::new(FloatParameter::new("Room Size").default_user_value(0.5))
            }
            FreeverbParameters::Freeze => Box::new(BoolParameter::new("Freeze")),
            FreeverbParameters::Dry => Box::new(FloatParameter::new("Dry").default_user_value(0.0)),
            FreeverbParameters::Wet => Box::new(FloatParameter::new("Wet").default_user_value(1.0)),
        }
    }
}

impl AudioModule for FreeverbModule {
    type Processor = FreeverbProcessor;

    fn create_processor(sample_rate: usize) -> Self::Processor {
        FreeverbProcessor::new(sample_rate)
    }
}
