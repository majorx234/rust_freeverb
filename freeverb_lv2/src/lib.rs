use freeverb::Freeverb;
use itertools::izip;
use lv2::prelude::*;

#[derive(PortCollection)]
struct Ports {
    dampening: InputPort<Control>,
    width: InputPort<Control>,
    room_size: InputPort<Control>,
    freeze: InputPort<Control>,
    dry: InputPort<Control>,
    wet: InputPort<Control>,
    input_l: InputPort<Audio>,
    input_r: InputPort<Audio>,
    output_l: OutputPort<Audio>,
    output_r: OutputPort<Audio>,
}

#[uri("urn:freeverb_lv2")]
struct FreeverbLv2 {
    freeverb: Freeverb,
}

impl Plugin for FreeverbLv2 {
    type Ports = Ports;
    type InitFeatures = ();
    type AudioFeatures = ();

    fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
        let sample_rate = 48000;
        Some(Self {
            freeverb: Freeverb::new(sample_rate),
        })
    }

    fn run(&mut self, ports: &mut Ports, _features: &mut (), _some_u32: u32) {
        self.freeverb.set_dampening(*(ports.dampening) as f64);
        self.freeverb.set_width(*(ports.width) as f64);
        self.freeverb.set_room_size(*(ports.room_size) as f64);
        self.freeverb.set_freeze(*(ports.freeze) > 0.0);
        self.freeverb.set_dry(*(ports.dry) as f64);
        self.freeverb.set_wet(*(ports.wet) as f64);

        for (in_vec_l, in_vec_r, out_vec_l, out_vec_r) in izip!(
            ports.input_l.iter(),
            ports.input_r.iter(),
            ports.output_l.iter_mut(),
            ports.output_r.iter_mut()
        ) {
            let result = self.freeverb.tick((*in_vec_l as f64, *in_vec_r as f64));
            *out_vec_l = result.0 as f32;
            *out_vec_r = result.1 as f32;
        }
    }
}
lv2_descriptors!(FreeverbLv2);
