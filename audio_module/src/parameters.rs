use crate::widget::Widget;

pub struct Scale {
    pub lower: f32,
    pub upper: f32,
    pub step_size: f32,
}

pub trait Parameter {
    fn name(&self) -> String;
    fn default_user_value(&self) -> f32;
    fn widget(&self) -> Widget {
        Widget::Slider
    }
    fn scale(&self) -> Scale;
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

    fn widget(&self) -> Widget {
        Widget::Button
    }

    fn scale(&self) -> Scale {
        Scale {
            lower: 0.0f32,
            upper: 1.0f32,
            step_size: 1.0f32,
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

    fn widget(&self) -> Widget {
        Widget::Slider
    }

    fn scale(&self) -> Scale {
        Scale {
            lower: 0.0,
            upper: 1.0,
            step_size: 0.01,
        }
    }
}

pub struct CurveParameter {
    pub name: String,
    pub curve: Vec<f32>,
    pub default_user_value: Vec<f32>,
}

impl CurveParameter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            curve: Vec::new(),
            default_user_value: Vec::new(),
        }
    }

    pub fn default_user_value(mut self, default: Vec<f32>) -> Self {
        self.default_user_value.copy_from_slice(&default);
        self
    }
}

impl Parameter for CurveParameter {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn default_user_value(&self) -> f32 {
        if self.default_user_value.is_empty() {
            0.0
        } else {
            1.0
        }
    }
    fn widget(&self) -> Widget {
        Widget::Graph
    }

    fn scale(&self) -> Scale {
        Scale {
            lower: 0.0,
            upper: 1.0,
            step_size: 0.01,
        }
    }
}
