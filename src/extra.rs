//use clap::Parser;
//use tokio::time::{self, Duration};
use crossterm::event::{self, Event};
use ratatui::{text::Text, Frame};

/*
#[derive(Debug, Clone, clap::ValueEnum)]
enum Command {
    Set,
    Start,
    Stop,
    Pause,
}

enum State {
    Study,
    Break,
    Paused,
}

#[derive(Parser, Debug)]
#[command(name = "pomo")]
#[command(about = "A pomodoro timer", long_about = None)]
struct CommandLine {
    /// The command to execute
    #[arg(value_enum)]
    cmd: Command,
    
    /// Time parameter
    time: u32,
}


struct PomoTimer {
    state: State,
    max_time: u32,
    remaining_time: u32,
}

impl PomoTimer {
    fn set_timer(&mut self, time: u32) {
        self.max_time = time;
        self.remaining_time = time;
    }

    fn start_timer(&mut self) {

    }

    fn stop_timer(&mut self) {

    }

    fn pause_timer(&mut self) {

    }
}*/

fn main() {
    // Parse cmd args
    //let args = CommandLine::parse();

    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();

    /* 
    // Create a PomoTimer 
    let mut pt = PomoTimer {
        state: State::Paused,
        max_time: 0,
        remaining_time: 0,
    };

    // Interval that ticks every second
    let mut interval = time::interval(Duration::from_secs(1));

    // Select command
    match args.cmd {
        Command::Set=>pt.set_timer(),
        Command::Start=>pt.start_timer(),
        Command::Pause=>pt.pause_timer(),
        Command::Stop=>pt.stop_timer(),
    }

    for _ in 0..pt.max_time {
        interval.tick().await;

        // Decrement time
        pt.remaining_time -= 1;

        // Update time in tui

    }
    */
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
}

// pomo start 5s
// pomo start 10m
// pomo start 1:30:00

use std::thread;
use std::time::{Duration, Instant};

//use clap::Parser;
//use tokio::time::{self, Duration};
//use crossterm::event::{self, Event};
//use ratatui::{
//    layout::{Constraint, Layout},
//    widgets::Block,
//    Frame,
//};

fn fmain() {
    // Parse cmd args
    //let args = CommandLine::parse();

    //let mut terminal = ratatui::init();
    //loop {
    //    terminal.draw(render).expect("failed to draw frame");
    //    if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
    //        break;
    //    }
    //}
    //ratatui::restore();

    let time_limit = 30;

    // User entered a command to set the time -> Create a thread for the timer
    let handle = thread::spawn(move || {
        let mut curr_count = time_limit;
        let start_time = Instant::now();

        // When user enters the start command we start the timer

        while curr_count > 0 {
            let elapsed_time = start_time.elapsed();
            println!("Time remaining: {} ({}s)", curr_count, elapsed_time.as_secs_f64());

            // Pause the thread for 1 second
            thread::sleep(Duration::from_secs(1));
            curr_count -= 1;

            // Listen for stop command
        }
    });

    // Join the spawned thread
    handle.join().unwrap();

}


// pomo start 5s
// pomo start 10m
// pomo start 1:30:00

