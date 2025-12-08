use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration};
use clap::{Parser, Subcommand, ValueEnum};

/* Defualt study timer duration (Minutes) */
const DEFAULT_STUDY_DURATION: u32 = 25;

/* Default break timer duration (Minutes) */
const DEFAULT_SHORT_BREAK_DURATION: u32 = 5;

/* Defult long break cycle */
const DEFAULT_POMO_CYCLE: u32 = 4;

/* Defualt long break duration (Minutes) */
const DEFAULT_LONG_BREAK_DURATION: u32 = 30;

#[derive(Parser)]
#[command(name = "Pomo Timer")]
#[command(about = "A Pomodoro timer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Set {
        set_cmd_type: SetCommandType,

        duration: u32,
    },
    Pause,
    Resume,
    Reset {
        reset_cmd_type: ResetCommandType,
    },
}

/* State that the user can change using the set command */
#[derive(Clone, ValueEnum)]
enum SetCommandType {
    Study,
    ShortBreak,
    LongBreak,
    Cycle,
}

/* States the user can reset using the reset command */
#[derive(Clone, ValueEnum)]
enum ResetCommandType {
    Timer,
    Cycle,
}

/* Message sent fromthe main thread to the timer thread */
enum TimerMessage {
    Set(u32),
    Resume,
    Pause,
    Quit,
}

/* States the timer can be in */
enum TimerState {
    Study,
    ShortBreak,
    LongBreak,
}

/* Struct storing the state of the Pomo timer */
struct PomoState {
    is_running: bool,
    study_duration: u32,
    short_break_duration: u32,
    long_break_duration: u32,
    pomo_cycle: u32,
    cycle_count: u32,
    curr_state: TimerState,
}

impl PomoState {
    
    /* Create a new PomoState Struct */
    fn new() -> Self {
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
    fn update(&mut self, command: Commands, timer_tx: &Sender<TimerMessage>) {
        match command {
            // Change the value of the state specified in the set command
            Commands::Set { set_cmd_type, duration } => {
                match set_cmd_type {
                    SetCommandType::Study => {
                        self.study_duration = duration;

                         // Send update to timer thread
                        timer_tx.send(TimerMessage::Set(duration)).unwrap();
                        println!("Study duration set to {}s", duration);
                    }
                    SetCommandType::ShortBreak => {
                        self.short_break_duration = duration;

                        // Send update to timer thread
                        timer_tx.send(TimerMessage::Set(duration)).unwrap();
                        println!("Short break duration set to {}s", duration);
                    }
                    SetCommandType::LongBreak => {
                        self.long_break_duration = duration;

                        // Send update to timer thread
                        timer_tx.send(TimerMessage::Set(duration)).unwrap();
                        println!("Long break duration set to {}s", duration);
                    }
                    SetCommandType::Cycle => {
                        self.pomo_cycle = duration;
                        println!("Pomodoro cycle set to {}", duration);
                    }
                }
            }
            // Pause the timer
            Commands::Pause => {
                if self.is_running {
                    timer_tx.send(TimerMessage::Pause).unwrap();
                    println!("Pausing timer...");
                } else {
                    println!("No active timer to pause");
                }
            }
            // Resume the timer
            Commands::Resume => {
                if !self.is_running{
                    timer_tx.send(TimerMessage::Resume).unwrap();
                    println!("Resuming");
                } else {
                    println!("No paused timer to resume");
                }
            }
            Commands::Reset { reset_cmd_type} => {
                match reset_cmd_type {
                    ResetCommandType::Timer => {
                        // TODO: Might need to ensure this is only done if the timer is paused
                        // Current Reset timer
                        if let TimerState::Study = self.curr_state {
                            timer_tx.send(TimerMessage::Set(self.study_duration)).unwrap();
                        }

                        if let TimerState::ShortBreak = self.curr_state {
                            timer_tx.send(TimerMessage::Set(self.short_break_duration)).unwrap();
                        }

                        if let TimerState::LongBreak = self.curr_state {
                            timer_tx.send(TimerMessage::Set(self.long_break_duration)).unwrap();
                        }
                    }
                    ResetCommandType::Cycle => {
                        self.cycle_count = 0;
                    }
                }
            }
        }
        
    }
}

#[derive(Debug)]
enum TimerEvent {
    Tick(u32),           // Remaining seconds
    Completed,           // Timer finished
    Paused,
    Resumed,
}

/*
 * Thread running the pomo timer
 */
fn timer_thread(cmd_rx: Receiver<TimerMessage>, event_tx: Sender<TimerEvent>, init_duration: u32) {
    let mut is_running: bool = false;
    let mut remaining_time: u32 = init_duration;

    loop {
        if !is_running {
            // Wait for command
            match cmd_rx.recv() {
                Ok(TimerMessage::Resume) => {
                    is_running = true;
                    event_tx.send(TimerEvent::Resumed);
                }
                Ok(TimerMessage::Set(duration)) => {
                    remaining_time = duration;
                }
                _ => continue,
            }
        } else {
           // Check if the timer has reached 0
           if remaining_time == 0 {
            event_tx.send(TimerEvent::Completed).unwrap();
            is_running = false;
           } else {
            event_tx.send(TimerEvent::Tick(remaining_time)).unwrap();
            
            match cmd_rx.recv_timeout(Duration::from_secs(1)) {
                Ok(TimerMessage::Pause) => {
                    is_running = false;
                    event_tx.send(TimerEvent::Paused).unwrap();
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
    let mut pomo_state = PomoState::new();

    // Set up thread communication channels
    let (cmd_tx, cmd_rx) = mpsc::channel::<TimerMessage>();
    let (event_tx, event_rx) = mpsc::channel::<TimerEvent>();


    // Create timer thread
    let handle = thread::spawn(move || {
        timer_thread(cmd_rx, event_tx, pomo_state.study_duration);
    });

    // Set up TUI

    // Set up cli parsing
    let cli = Cli::parse();

    // Update state based on command
    pomo_state.update(cli.command, &cmd_tx);

    // Parse commands
    loop {
        match event_rx.recv_timeout(Duration::from_millis(100)) {
            Ok(TimerEvent::Completed) => {
                // swap state
                match pomo_state.curr_state {
                    TimerState::Study => {
                        // Check and update cycle count
                        if pomo_state.cycle_count == pomo_state.pomo_cycle {
                            pomo_state.curr_state = TimerState::LongBreak;
                            pomo_state.cycle_count = 0;

                            // Send long break duration to timer
                            cmd_tx.send(TimerMessage::Set(pomo_state.long_break_duration)).unwrap();
                        } else {
                            pomo_state.curr_state = TimerState::ShortBreak;
                            pomo_state.cycle_count += 1;

                            // Send short break duration to timer
                            cmd_tx.send(TimerMessage::Set(pomo_state.short_break_duration)).unwrap();
                        }
                    }
                    TimerState::ShortBreak | TimerState::LongBreak => {
                        pomo_state.curr_state = TimerState::Study;

                        // Send study duration to timer
                        cmd_tx.send(TimerMessage::Set(pomo_state.study_duration)).unwrap();

                    }
                }

                // Update UI
            }
            Ok(TimerEvent::Paused) => {
                pomo_state.is_running = false;
                // Update UI
            }
            Ok(TimerEvent::Resumed) => {
                pomo_state.is_running = true;
                // Update UI
            }
            Ok(TimerEvent::Tick(remaining)) => {
                //Update UI
            }
            _ => continue,
        }
    }



    // Join the spawned thread
    handle.join().unwrap();

}
