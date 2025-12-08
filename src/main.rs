use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
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
}

/* State that the user can change using the set command */
#[derive(Clone, ValueEnum)]
enum SetCommandType {
    Study,
    ShortBreak,
    LongBreak,
    Cycle,
}

/* Message sent fromthe main thread to the timer thread */
enum TimerMessage {
    Set(SetCommandType, u32),    // Set the duration of the timer
    Resume,                      // Start the time again 
    Pause,                       // Stop the timer
    Quit,                        // Terminate the thread
}

/* States the timer can be in */
enum TimerState {
    Study,
    Break,
    Paused,
}

/* Struct storing the state of the Pomo timer */
struct PomoState {
    study_duration: u32,
    short_break_duration: u32,
    long_break_duration: u32,
    pomo_cycle: u32,
    cycle_count: u32,
    timer_state: TimerState,
}

/*
 * Update the pomo state struct
 */
fn update_pomo_state(pomo_state: &PomoState, msg: TimerMessage) {

}

/*
 * Thread running the pomo timer
 */
fn timer_thread(rx: Receiver<TimerMessage>) {
    let mut is_running = false; // Track if the timer should be runnijg
    let mut remaining_time = 30; // default duration
    let start_time = Instant::now();

    // Run the timer
    while remaining_time > 0 {
        let elapsed_time = start_time.elapsed();
        println!("Time remaining: {} ({}s)", remaining_time, elapsed_time.as_secs_f64());

        // Pause the thread for 1 second
        thread::sleep(Duration::from_secs(1));
        remaining_time -= 1;

    }
}

/*
 * Main entry point for the program
 */
fn main() {

    // Notes:
    //  Main thread handles updating state
    //  Timer thread is sent updates by the main thread

    // Set up communication
    let (tx, rx) = mpsc::channel::<TimerMessage>();

    // Init state struct
    let pomo_state = PomoState {
        study_duration: DEFAULT_STUDY_DURATION,
        short_break_duration: DEFAULT_SHORT_BREAK_DURATION,
        long_break_duration: DEFAULT_LONG_BREAK_DURATION,
        pomo_cycle: DEFAULT_POMO_CYCLE,
        cycle_count: 0,
        timer_state: TimerState::Paused,
    };

    // Create timer thread
    let handle = thread::spawn(move || {
        timer_thread(rx);
    });

    // Set up cli parsing
    let cli = Cli::parse();
    match cli.command {
        Commands::Set { set_cmd_type, duration } => {
            // Send message to timer thread here
            // tx.send(TimerMessage::Set(set_cmd_type, duration)).unwrap();
        }
        Commands::Pause => {
            println!("Pausing timer");
            // Send pause message
            // tx.send(TimerMessage::Pause).unwrap();
        }
        Commands::Resume => {
            println!("Resuming timer");
            // Send resume message
            // tx.send(TimerMessage::Resume).unwrap();
        }
    }

    // Join the spawned thread
    handle.join().unwrap();

}
