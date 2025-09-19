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
    let (mut freeverb_egui_app, rx_close) =
        FreeverbEguiApp::new::<Module>(None, None);
    let (jack_thread_hdl, tx_command) = start_jack_thread::<Module>(samplerate, rx_close);
    freeverb_egui_app.set_tx_command(Some(tx_command));
    freeverb_egui_app.set_jack_thread(Some(jack_thread_hdl));

    let _ = eframe::run_native(
        "FreeverbEgui App",
        options,
        Box::new(|_cc| Ok(Box::new(freeverb_egui_app))),
    );
}
