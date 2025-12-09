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

    // Set up cli parsing
    let cli = cli::Cli::parse();

    // Update state based on command
    pomo_state.update(cli.command, &cmd_tx);

    // Parse commands
    loop {
        match event_rx.recv_timeout(Duration::from_millis(100)) {
            Ok(msgs::TimerEvent::Completed) => {
                // swap state
                match pomo_state.curr_state {
                    state::TimerState::Study => {
                        // Check and update cycle count
                        if pomo_state.cycle_count == pomo_state.pomo_cycle {
                            pomo_state.curr_state = state::TimerState::LongBreak;
                            pomo_state.cycle_count = 0;

                            // Send long break duration to timer
                            cmd_tx.send(msgs::TimerMessage::Set(pomo_state.long_break_duration)).unwrap();

                            // Update UI
                        } else {
                            pomo_state.curr_state = state::TimerState::ShortBreak;
                            pomo_state.cycle_count += 1;

                            // Send short break duration to timer
                            cmd_tx.send(msgs::TimerMessage::Set(pomo_state.short_break_duration)).unwrap();

                            //Update UI
                        }
                    }
                    state::TimerState::ShortBreak | state::TimerState::LongBreak => {
                        pomo_state.curr_state = state::TimerState::Study;

                        // Send study duration to timer
                        cmd_tx.send(msgs::TimerMessage::Set(pomo_state.study_duration)).unwrap();

                        // Update UI

                    }
                }

            }
            Ok(msgs::TimerEvent::Paused) => {
                pomo_state.is_running = false;
                // Update UI
            }
            Ok(msgs::TimerEvent::Resumed) => {
                pomo_state.is_running = true;
                // Update UI
            }
            Ok(msgs::TimerEvent::Tick(remaining)) => {
                //Update UI
            }
            _ => continue,
        }
    }



    // Join the spawned thread
    handle.join().unwrap();

}
