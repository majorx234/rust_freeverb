mod jackprocess;
use crate::jackprocess::start_jack_thread;
use audio_module::{widget::Widget, AudioModule, Command};
use crossbeam_channel::{bounded, Receiver, Sender};
use eframe::egui;
use freeverb_lib::FreeverbEguiApp;
use freeverb_module::FreeverbModule;

fn main() {
    main_run::<FreeverbModule>();
}

fn main_run<Module: AudioModule>() {
    let (tx_command, rx_command) = bounded::<Command>(1024);
    let options = eframe::NativeOptions::default();

    let _ = eframe::run_native(
        "FreeverbEgui App",
        options,
        Box::new(|_cc| Ok(Box::new(FreeverbEguiApp::default()))),
    );
}
