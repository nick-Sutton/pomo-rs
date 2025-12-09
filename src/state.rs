use std::sync::mpsc::{Sender};
use crate::{cli, msgs};

/* Defualt study timer duration (Minutes) */
const DEFAULT_STUDY_DURATION: u32 = 25;

/* Default break timer duration (Minutes) */
const DEFAULT_SHORT_BREAK_DURATION: u32 = 5;

/* Defult long break cycle */
const DEFAULT_POMO_CYCLE: u32 = 4;

/* Defualt long break duration (Minutes) */
const DEFAULT_LONG_BREAK_DURATION: u32 = 30;

/* States the timer can be in */
pub enum TimerState {
    Study,
    ShortBreak,
    LongBreak,
}

/* Struct storing the state of the Pomo timer */
pub struct PomoState {
    pub is_running: bool,
    pub study_duration: u32,
    pub short_break_duration: u32,
    pub long_break_duration: u32,
    pub pomo_cycle: u32,
    pub cycle_count: u32,
    pub curr_state: TimerState,
}

impl PomoState {
    
    /* Create a new PomoState Struct */
    pub fn new() -> Self {
        PomoState {
            is_running: false,
            study_duration: DEFAULT_STUDY_DURATION,
            short_break_duration: DEFAULT_SHORT_BREAK_DURATION,
            long_break_duration: DEFAULT_LONG_BREAK_DURATION,
            pomo_cycle: DEFAULT_POMO_CYCLE,
            cycle_count: 0,
            curr_state: TimerState::Study,
        }
    }

    /* Update state based on command */
    pub fn update(&mut self, command: cli::Commands, timer_tx: &Sender<msgs::TimerMessage>) {
        match command {
            // Change the value of the state specified in the set command
            cli::Commands::Set { set_cmd_type, duration } => {
                match set_cmd_type {
                    cli::SetCommandType::Study => {
                        self.study_duration = duration;

                         // Send update to timer thread
                        timer_tx.send(msgs::TimerMessage::Set(duration)).unwrap();
                        println!("Study duration set to {}s", duration);
                    }
                    cli::SetCommandType::ShortBreak => {
                        self.short_break_duration = duration;

                        // Send update to timer thread
                        timer_tx.send(msgs::TimerMessage::Set(duration)).unwrap();
                        println!("Short break duration set to {}s", duration);
                    }
                    cli::SetCommandType::LongBreak => {
                        self.long_break_duration = duration;

                        // Send update to timer thread
                        timer_tx.send(msgs::TimerMessage::Set(duration)).unwrap();
                        println!("Long break duration set to {}s", duration);
                    }
                    cli::SetCommandType::Cycle => {
                        self.pomo_cycle = duration;
                        println!("Pomodoro cycle set to {}", duration);
                    }
                }
            }
            // Pause the timer
            cli::Commands::Pause => {
                if self.is_running {
                    self.is_running = false;
                    timer_tx.send(msgs::TimerMessage::Pause).unwrap();
                    println!("Pausing timer...");
                } else {
                    println!("No active timer to pause");
                }
            }
            // Resume the timer
            cli::Commands::Resume => {
                if !self.is_running{
                    self.is_running = true;
                    timer_tx.send(msgs::TimerMessage::Resume).unwrap();
                    println!("Resuming");
                } else {
                    println!("No paused timer to resume");
                }
            }
            cli::Commands::Reset { reset_cmd_type} => {
                match reset_cmd_type {
                    cli::ResetCommandType::Timer => {
                        // TODO: Might need to ensure this is only done if the timer is paused
                        // Current Reset timer
                        if let TimerState::Study = self.curr_state {
                            timer_tx.send(msgs::TimerMessage::Set(self.study_duration)).unwrap();
                        }

                        if let TimerState::ShortBreak = self.curr_state {
                            timer_tx.send(msgs::TimerMessage::Set(self.short_break_duration)).unwrap();
                        }

                        if let TimerState::LongBreak = self.curr_state {
                            timer_tx.send(msgs::TimerMessage::Set(self.long_break_duration)).unwrap();
                        }
                    }
                    cli::ResetCommandType::Cycle => {
                        self.cycle_count = 0;
                    }
                }
            }
        }
        
    }
}