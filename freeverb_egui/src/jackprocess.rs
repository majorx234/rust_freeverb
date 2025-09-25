extern crate jack;
use audio_module::{AudioModule, AudioProcessor, Command, CommandHandler};
use bus::Bus;
use crossbeam_channel::{bounded, Sender};
use std::{thread, time::Duration};

pub fn start_jack_thread<Module: AudioModule>(
    _sample_rate: usize,
) -> (std::thread::JoinHandle<()>, Sender<Command>, Bus<bool>) {
    let (tx_command, rx_command) = bounded::<Command>(1024);
    let mut tx_close = Bus::<bool>::new(1);
    let mut rx_close = tx_close.add_rx();

    (
        std::thread::spawn(move || {
            let (client, _status) =
                jack::Client::new(&Module::name(), jack::ClientOptions::NO_START_SERVER)
                    .expect("No Jack server running\n");
            let sample_rate = client.sample_rate();

            let mut processor = Module::create_processor(sample_rate);
            // register ports
            let in_l = client
                .register_port("freeverb_in_l", jack::AudioIn)
                .unwrap();
            let in_r = client
                .register_port("freeverb_in_r", jack::AudioIn)
                .unwrap();
            let mut out_l = client
                .register_port("freeverb_out_l", jack::AudioOut)
                .unwrap();
            let mut out_r = client
                .register_port("freeverb_out_r", jack::AudioOut)
                .unwrap();

            let process_callback =
                move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
                    if let Ok(rx_command_msg) = rx_command.try_recv() {
                        let command_msg: Command = rx_command_msg;

                        processor.handle_command(command_msg);
                    };
                    processor.process_stereo(
                        in_l.as_slice(ps),
                        in_r.as_slice(ps),
                        out_l.as_mut_slice(ps),
                        out_r.as_mut_slice(ps),
                    );
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
        }),
        tx_command,
        tx_close,
    )
}
