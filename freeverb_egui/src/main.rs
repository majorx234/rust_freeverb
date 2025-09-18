mod jackprocess;
use crate::jackprocess::start_jack_thread;
use audio_module::AudioModule;
use bus::Bus;
use eframe::egui;
use freeverb_lib::FreeverbEguiApp;
use freeverb_module::FreeverbModule;

fn main() {
    main_run::<FreeverbModule>();
}

fn main_run<Module: AudioModule>() {
    let mut tx_close = Bus::<bool>::new(1);
    let rx1_close = tx_close.add_rx();

    let options = eframe::NativeOptions::default();

    let samplerate = 48000;
    let (jack_thread_hdl, tx_command) = start_jack_thread::<Module>(samplerate, rx1_close);

    let freeverb_egui_app =
        FreeverbEguiApp::new::<Module>(Some(jack_thread_hdl), Some(tx_close), Some(tx_command));
    let _ = eframe::run_native(
        "FreeverbEgui App",
        options,
        Box::new(|_cc| Ok(Box::new(freeverb_egui_app))),
    );
}
