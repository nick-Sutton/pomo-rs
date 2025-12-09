use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration};

use crate::msgs;

/*
 * Thread running the pomo timer
 */
pub fn timer_thread(cmd_rx: Receiver<msgs::TimerMessage>, event_tx: Sender<msgs::TimerEvent>, init_duration: u32) {
    let mut is_running: bool = false;
    let mut remaining_time: u32 = init_duration;

    loop {
        if !is_running {
            // Wait for command
            match cmd_rx.recv() {
                Ok(msgs::TimerMessage::Resume) => {
                    is_running = true;
                    event_tx.send(msgs::TimerEvent::Resumed).unwrap();
                }
                Ok(msgs::TimerMessage::Set(duration)) => {
                    remaining_time = duration;
                }
                _ => continue,
            }
        } else {
           // Check if the timer has reached 0
           if remaining_time == 0 {
            event_tx.send(msgs::TimerEvent::Completed).unwrap();
            is_running = false;
           } else {
            event_tx.send(msgs::TimerEvent::Tick(remaining_time)).unwrap();
            
            match cmd_rx.recv_timeout(Duration::from_secs(1)) {
                Ok(msgs::TimerMessage::Pause) => {
                    is_running = false;
                    event_tx.send(msgs::TimerEvent::Paused).unwrap();
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    // decrement count
                    remaining_time -= 1;
                }
                _ => {}
            }
           }

        }
    }
}