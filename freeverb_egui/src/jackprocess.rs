extern crate jack;
use audio_module::{AudioModule, Command, CommandHandler};
use bus::BusReader;
use crossbeam_channel::Receiver;

use std::{process::exit, thread, time::Duration};

pub fn start_jack_thread<Module: AudioModule>(
    _sample_rate: usize,
    rx_command: Receiver<Command>,
    mut rx_close: BusReader<bool>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let (client, _status) = jack::Client::new("freeverb", jack::ClientOptions::NO_START_SERVER)
            .expect("No Jack server running\n");
        let sample_rate = client.sample_rate();

        let mut processor = Module::create_processor(sample_rate);
        // register ports
        let mut in_a = client
            .register_port("freeverb_in_l", jack::AudioIn)
            .unwrap();
        let mut in_b = client
            .register_port("freeverb_in_r", jack::AudioIn)
            .unwrap();
        let mut out_a = client
            .register_port("freeverb_out_l", jack::AudioOut)
            .unwrap();
        let mut out_b = client
            .register_port("freeverb_out_r", jack::AudioOut)
            .unwrap();

        let process_callback = move |_: &jack::Client, _ps: &jack::ProcessScope| -> jack::Control {
            if let Ok(rx_command_msg) = rx_command.try_recv() {
                let command_msg: Command = rx_command_msg;
                processor.handle_command(command_msg);
            };

            jack::Control::Continue
        };

        let process = jack::ClosureProcessHandler::new(process_callback);
        let active_client = client.activate_async((), process).unwrap();
        let mut run: bool = true;
        while run {
            thread::sleep(Duration::from_millis(100));
            match rx_close.recv() {
                Ok(running) => run = running,
                Err(_) => run = false,
            }
        }
        let _ = active_client.deactivate();
    })
}
