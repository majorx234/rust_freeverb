extern crate jack;
use audio_module::{AudioModule, AudioProcessor, Command, CommandHandler};
use bus::BusReader;
use crossbeam_channel::Receiver;

use std::{process::exit, thread, time::Duration};

pub fn start_jack_thread(
    sample_rate: usize,
    rx_command: crossbeam_channel::Receiver<Command>,
    mut rx_close: BusReader<bool>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let mut run: bool = true;
        let (client, _status) = jack::Client::new("freeverb", jack::ClientOptions::NO_START_SERVER)
            .expect("No Jack server running\n");

        let sample_rate = client.sample_rate();
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

        let process_callback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
            jack::Control::Continue
        };

        let process = jack::ClosureProcessHandler::new(process_callback);
        let active_client = client.activate_async((), process).unwrap();

        let mut run = true;
        while run {
            thread::sleep(Duration::from_millis(100));
            match rx_close.recv() {
                Ok(running) => run = running,
                Err(_) => run = false,
            }
        }
    })
}
