use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration};
use clap::Parser;

mod cli;
mod timer;
mod msgs;
mod state;

/*
 * Main entry point for the program
 */
fn main() {

    // Notes:
    //  Main thread handles updating state
    //  Timer thread is sent updates by the main thread

    // You can only pause if the timer is running
    // You can only resume of the timer is paused

    // Init state struct
    let mut pomo_state = state::PomoState::new();

    // Set up thread communication channels
    let (cmd_tx, cmd_rx) = mpsc::channel::<msgs::TimerMessage>();
    let (event_tx, event_rx) = mpsc::channel::<msgs::TimerEvent>();


    // Create timer thread
    let handle = thread::spawn(move || {
        timer::timer_thread(cmd_rx, event_tx, pomo_state.study_duration);
    });

    // Set up TUI

    // Parse commands
    loop {
        // Set up cli parsing
        let cli = cli::Cli::parse();

        // Update state based on command
        pomo_state.update(cli.command, &cmd_tx);

        let event = event_rx.recv_timeout(Duration::from_millis(100)).unwrap();

        pomo_state.handle_event(event, &cmd_tx);
    }



    // Join the spawned thread
    handle.join().unwrap();

}
