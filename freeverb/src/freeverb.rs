use super::{all_pass::AllPass, comb::Comb};

static FIXED_GAIN: f64 = 0.015;
static SCALE_WET: f64 = 3.0;

const SCALE_DAMPENING: f64 = 0.4;

const SCALE_ROOM: f64 = 0.28;
const OFFSET_ROOM: f64 = 0.7;

static STEREOSPREAD: usize = 23;

// These values assume 44.1KHz sample rate
// they will probably be OK for 48KHz sample rate
// but would need scaling for 96KHz (or other) sample rates.
// The values were obtained by listening tests.

static COMB_TUNING_L1: usize = 1116;
static COMB_TUNING_R1: usize = 1116 + STEREOSPREAD;
static COMB_TUNING_L2: usize = 1188;
static COMB_TUNING_R2: usize = 1188 + STEREOSPREAD;
static COMB_TUNING_L3: usize = 1277;
static COMB_TUNING_R3: usize = 1277 + STEREOSPREAD;
static COMB_TUNING_L4: usize = 1356;
static COMB_TUNING_R4: usize = 1356 + STEREOSPREAD;
static COMB_TUNING_L5: usize = 1422;
static COMB_TUNING_R5: usize = 1422 + STEREOSPREAD;
static COMB_TUNING_L6: usize = 1491;
static COMB_TUNING_R6: usize = 1491 + STEREOSPREAD;
static COMB_TUNING_L7: usize = 1557;
static COMB_TUNING_R7: usize = 1557 + STEREOSPREAD;
static COMB_TUNING_L8: usize = 1617;
static COMB_TUNING_R8: usize = 1617 + STEREOSPREAD;

static ALLPASS_TUNING_L1: usize = 556;
static ALLPASS_TUNING_R1: usize = 556 + STEREOSPREAD;
static ALLPASS_TUNING_L2: usize = 441;
static ALLPASS_TUNING_R2: usize = 441 + STEREOSPREAD;
static ALLPASS_TUNING_L3: usize = 341;
static ALLPASS_TUNING_R3: usize = 341 + STEREOSPREAD;
static ALLPASS_TUNING_L4: usize = 225;
static ALLPASS_TUNING_R4: usize = 225 + STEREOSPREAD;

fn adjust_length(length: usize, sample_rate: usize) -> usize {
    (length as f64 * sample_rate as f64 / 44100.0) as usize
}

pub struct Freeverb {
    combs: [(Comb, Comb); 8],
    allpasses: [(AllPass, AllPass); 4],
    wet_gains: (f64, f64),
    wet: f64,
    width: f64,
    dry: f64,
    input_gain: f64,
    dampening: f64,
    room_size: f64,
    frozen: bool,
}

impl Freeverb {
    pub fn new(sample_rate: usize) -> Self {
        Freeverb {
            combs: [
                (
                    Comb::new(adjust_length(COMB_TUNING_L1, sample_rate)),
                    Comb::new(adjust_length(COMB_TUNING_R1, sample_rate)),
                ),
                (
                    Comb::new(adjust_length(COMB_TUNING_L2, sample_rate)),
                    Comb::new(adjust_length(COMB_TUNING_R2, sample_rate)),
                ),
                (
                    Comb::new(adjust_length(COMB_TUNING_L3, sample_rate)),
                    Comb::new(adjust_length(COMB_TUNING_R3, sample_rate)),
                ),
                (
                    Comb::new(adjust_length(COMB_TUNING_L4, sample_rate)),
                    Comb::new(adjust_length(COMB_TUNING_R4, sample_rate)),
                ),
                (
                    Comb::new(adjust_length(COMB_TUNING_L5, sample_rate)),
                    Comb::new(adjust_length(COMB_TUNING_R5, sample_rate)),
                ),
                (
                    Comb::new(adjust_length(COMB_TUNING_L6, sample_rate)),
                    Comb::new(adjust_length(COMB_TUNING_R6, sample_rate)),
                ),
                (
                    Comb::new(adjust_length(COMB_TUNING_L7, sample_rate)),
                    Comb::new(adjust_length(COMB_TUNING_R7, sample_rate)),
                ),
                (
                    Comb::new(adjust_length(COMB_TUNING_L8, sample_rate)),
                    Comb::new(adjust_length(COMB_TUNING_R8, sample_rate)),
                ),
            ],
            allpasses: [
                (
                    AllPass::new(adjust_length(ALLPASS_TUNING_L1, sample_rate)),
                    AllPass::new(adjust_length(ALLPASS_TUNING_R1, sample_rate)),
                ),
                (
                    AllPass::new(adjust_length(ALLPASS_TUNING_L2, sample_rate)),
                    AllPass::new(adjust_length(ALLPASS_TUNING_R2, sample_rate)),
                ),
                (
                    AllPass::new(adjust_length(ALLPASS_TUNING_L3, sample_rate)),
                    AllPass::new(adjust_length(ALLPASS_TUNING_R3, sample_rate)),
                ),
                (
                    AllPass::new(adjust_length(ALLPASS_TUNING_L4, sample_rate)),
                    AllPass::new(adjust_length(ALLPASS_TUNING_R4, sample_rate)),
                ),
            ],
            wet_gains: (0.0, 0.0),
            wet: 1.0,
            dry: 0.0,
            input_gain: 0.0,
            width: 0.5,
            dampening: 0.5,
            room_size: 0.5,
            frozen: false,
        }
    }
    pub fn set_wet(&mut self, value: f64) {
        self.wet = value * SCALE_WET;
        self.update_wet_gains();
    }

    pub fn set_width(&mut self, value: f64) {
        self.width = value;
        self.update_wet_gains();
    }

    fn update_wet_gains(&mut self) {
        self.wet_gains = (
            self.wet * (self.width / 2.0 + 0.5),
            self.wet * ((1.0 - self.width) / 2.0),
        )
    }

    pub fn set_dampening(&mut self, value: f64) {
        self.dampening = value * SCALE_DAMPENING;
        self.update_combs();
    }

    pub fn set_freeze(&mut self, frozen: bool) {
        self.frozen = frozen;
        self.update_combs();
    }

    fn set_frozen(&mut self, frozen: bool) {
        self.frozen = frozen;
        self.input_gain = if frozen { 0.0 } else { 1.0 };
        self.update_combs();
    }

    pub fn set_room_size(&mut self, value: f64) {
        self.room_size = value * SCALE_ROOM + OFFSET_ROOM;
        self.update_combs();
    }

    fn update_combs(&mut self) {
        let (feedback, dampening) = if self.frozen {
            (1.0, 0.0)
        } else {
            (self.room_size, self.dampening)
        };

        for combs in self.combs.iter_mut() {
            combs.0.set_feedback(feedback);
            combs.1.set_feedback(feedback);

            combs.0.set_dampening(dampening);
            combs.1.set_dampening(dampening);
        }
    }

    pub fn set_dry(&mut self, value: f64) {
        self.dry = value;
    }

    pub fn tick(&mut self, input: (f64, f64)) -> (f64, f64) {
        let input_mixed = (input.0 + input.1) * FIXED_GAIN * self.input_gain;
        let mut out = (0.0, 0.0);
        for comb in self.combs.iter_mut() {
            out.0 += comb.0.tick(input_mixed);
            out.1 += comb.1.tick(input_mixed);
        }
        for allpass in self.allpasses.iter_mut() {
            out.0 = allpass.0.tick(out.0);
            out.1 = allpass.1.tick(out.1);
        }

        (
            out.0 * self.wet_gains.0 + out.1 * self.wet_gains.1 + input.0 * self.dry,
            out.1 * self.wet_gains.0 + out.0 * self.wet_gains.1 + input.1 * self.dry,
        )
    }
}
