pub trait Parameter {
    fn name(&self) -> String;
    fn default_user_value(&self) -> f32;
}

pub trait ParameterProvider {
    fn parameter_count() -> usize;
    fn parameter(id: usize) -> Box<dyn Parameter>;
}

pub struct BoolParameter {
    pub name: String,
    pub default_user_value: bool,
}

impl BoolParameter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            default_user_value: false,
        }
    }

    pub fn default_user_value(mut self, default: bool) -> Self {
        self.default_user_value = default;
        self
    }
}

impl Parameter for BoolParameter {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn default_user_value(&self) -> f32 {
        if self.default_user_value {
            1.0
        } else {
            0.0
        }
    }
}

pub struct FloatParameter {
    pub name: String,
    pub unit: String,
    pub default_user_value: f32,
}

impl FloatParameter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            unit: String::default(),
            default_user_value: 0.0,
        }
    }

    pub fn default_user_value(mut self, default: f32) -> Self {
        self.default_user_value = default;
        self
    }
}

impl Parameter for FloatParameter {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn default_user_value(&self) -> f32 {
        self.default_user_value
    }
}
