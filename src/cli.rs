use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "Pomo Timer")]
#[command(about = "A Pomodoro timer", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
pub enum SetCommandType {
    Study,
    ShortBreak,
    LongBreak,
    Cycle,
}

/* States the user can reset using the reset command */
#[derive(Clone, ValueEnum)]
pub enum ResetCommandType {
    Timer,
    Cycle,
}