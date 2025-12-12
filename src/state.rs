use std::sync::mpsc::{Sender};
use crate::{cli, msgs::{self, TimerEvent}};

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
    pub fn update(&mut self, command: cli::Commands) -> Result<Option<msgs::TimerMessage>, msgs::PomoErr>{
        match command {
            // Change the value of the state specified in the set command
            cli::Commands::Set { set_cmd_type, duration } => {
                match set_cmd_type {
                    cli::SetCommandType::Study => {
                        self.study_duration = duration;

                         // Send update to timer thread
                        return Ok(Some(msgs::TimerMessage::Set(duration)));
                    }
                    cli::SetCommandType::ShortBreak => {
                        self.short_break_duration = duration;

                        // Send update to timer thread
                        return Ok(Some(msgs::TimerMessage::Set(duration)));
                    }
                    cli::SetCommandType::LongBreak => {
                        self.long_break_duration = duration;

                        // Send update to timer thread
                        return Ok(Some(msgs::TimerMessage::Set(duration)));
                    }
                    cli::SetCommandType::Cycle => {
                        self.pomo_cycle = duration;
                        return Ok(None);
                    }
                }
            }
            // Pause the timer
            cli::Commands::Pause => {
                if self.is_running {
                    self.is_running = false;

                    return Ok(Some(msgs::TimerMessage::Pause));
                } else {
                    return Err(msgs::PomoErr::TimerAlreadyPaused);
                }
            }
            // Resume the timer
            cli::Commands::Resume => {
                if !self.is_running{
                    self.is_running = true;
                    return Ok(Some(msgs::TimerMessage::Resume));
                } else {
                    return Err(msgs::PomoErr::TimerAlreadyActive);
                }
            }
            cli::Commands::Reset { reset_cmd_type} => {
                match reset_cmd_type {
                    cli::ResetCommandType::Timer => {
                        // TODO: Might need to ensure this is only done if the timer is paused
                        let duration = match self.curr_state {
                            TimerState::Study => self.study_duration,
                            TimerState::ShortBreak => self.short_break_duration,
                            TimerState::LongBreak => self.long_break_duration,
                        };
                        return Ok(Some(msgs::TimerMessage::Set(duration)));
                    }
                    cli::ResetCommandType::Cycle => {
                        self.cycle_count = 0;
                        return Ok(None);
                    }
                }
            }
        }
        
    }

    pub fn handle_event(&mut self, event: TimerEvent, cmd_tx: &Sender<msgs::TimerMessage>) {
        match event {
            TimerEvent::Completed => {
                // swap state
                match self.curr_state {
                    TimerState::Study => {
                        // Check and update cycle count
                        if self.cycle_count == self.pomo_cycle {
                            self.curr_state = TimerState::LongBreak;
                            self.cycle_count = 0;

                            // Send long break duration to timer
                            cmd_tx.send(msgs::TimerMessage::Set(self.long_break_duration)).unwrap();

                            // Update UI
                        } else {
                            self.curr_state = TimerState::ShortBreak;
                            self.cycle_count += 1;

                            // Send short break duration to timer
                            cmd_tx.send(msgs::TimerMessage::Set(self.short_break_duration)).unwrap();

                            //Update UI
                        }
                    }
                    TimerState::ShortBreak | TimerState::LongBreak => {
                        self.curr_state = TimerState::Study;

                        // Send study duration to timer
                        cmd_tx.send(msgs::TimerMessage::Set(self.study_duration)).unwrap();

                        // Update UI

                    }
                }

            }
            TimerEvent::Paused => {
                self.is_running = false;
                // Update UI
            }
            TimerEvent::Resumed => {
                self.is_running = true;
                // Update UI
            }
            TimerEvent::Tick(remaining) => {
                // Update UI
            }
            TimerEvent::Set => {
                // Update UI
            }
            _ => return,
        }
    }
}