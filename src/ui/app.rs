/* Struct storing the state of the TUI Application */
pub struct App {
    pomo_state: PomoState,
    input_buffer: String,
    command_history: Vec<String>,
    status_message: Option<String>,
    should_quit: bool,
}

impl App {
    fn handle_input(&mut self, key: KeyEvent, timer_tx: &Sender<TimerCommand>) {
        match key.code {
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Enter => {
                self.execute_command(timer_tx);
            }
            KeyCode::Esc => {
                self.input_buffer.clear();
            }
            _ => {}
        }
    }
    
    fn execute_command(&mut self, timer_tx: &Sender<TimerCommand>) {
        let input = self.input_buffer.trim();
        if input.is_empty() {
            return;
        }
        
        // Parse command using your existing CLI parser
        // Split input into args: "set study 1500" -> ["set", "study", "1500"]
        let args: Vec<&str> = input.split_whitespace().collect();
        
        match Cli::try_parse_from(std::iter::once("pomo").chain(args.iter().copied())) {
            Ok(cli) => {
                // Reuse your existing update logic!
                self.pomo_state.update(cli.command, timer_tx);
                self.status_message = Some(format!("✓ Executed: {}", input));
                self.command_history.push(input.to_string());
            }
            Err(e) => {
                self.status_message = Some(format!("✗ Error: {}", e));
            }
        }
        
        self.input_buffer.clear();
    }
}