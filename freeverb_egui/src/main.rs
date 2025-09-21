mod jackprocess;
use crate::jackprocess::start_jack_thread;
use audio_module::AudioModule;
use eframe::egui;
use freeverb_lib::FreeverbEguiApp;
use freeverb_module::FreeverbModule;

fn main() {
    main_run::<FreeverbModule>();
}

fn main_run<Module: AudioModule>() {
    let options = eframe::NativeOptions::default();

    let samplerate = 48000;
    let (jack_thread_hdl, tx_command, tx_close) = start_jack_thread::<Module>(samplerate);
    let freeverb_egui_app =
        FreeverbEguiApp::new::<Module>(Some(jack_thread_hdl), Some(tx_command), Some(tx_close));

    let _ = eframe::run_native(
        "FreeverbEgui App",
        options,
        Box::new(|_cc| Ok(Box::new(freeverb_egui_app))),
    );
}
