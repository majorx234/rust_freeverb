use super::{all_pass::AllPass, comb::Comb};

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
                    AllPass::new(adjust_length(ALLPASS_TUNING_R1, sample_rate)),
                ),
                (
                    AllPass::new(adjust_length(ALLPASS_TUNING_L4, sample_rate)),
                    AllPass::new(adjust_length(ALLPASS_TUNING_L4, sample_rate)),
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
}
