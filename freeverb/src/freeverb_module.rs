use audio_module::parameters::{BoolParameter, FloatParameter, Parameter, ParameterProvider};
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
