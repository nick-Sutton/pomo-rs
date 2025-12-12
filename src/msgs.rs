/* Message sent fromthe main thread to the timer thread */
pub enum TimerMessage {
    Set(u32),
    Resume,
    Pause,
    Quit,
}
#[derive(Debug)]
pub enum TimerEvent {
    Tick(u32),           // Remaining seconds
    Completed,           // Timer finished
    Paused,
    Resumed,
    Set,
}

pub enum PomoErr {
    InvalidDuration(u32),
    TimerAlreadyActive,
    TimerAlreadyPaused,
    ChannelSendFailed,
}