pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),      // Timer display
            Constraint::Length(3),   // Status bar
            Constraint::Length(3),   // Input box
            Constraint::Length(5),   // Help text
        ])
        .split(f.size());
    
    // Timer display (big numbers, progress bar)
    render_timer_display(f, chunks[0], &app.pomo_state);
    
    // Status message
    if let Some(msg) = &app.status_message {
        let status = Paragraph::new(msg.as_str())
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(status, chunks[1]);
    }
    
    // Input box with cursor
    let input = Paragraph::new(app.input_buffer.as_str())
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Command"))
        .style(Style::default().fg(Color::White));
    f.render_widget(input, chunks[2]);
    
    // Show cursor in input box
    f.set_cursor(
        chunks[2].x + app.input_buffer.len() as u16 + 1,
        chunks[2].y + 1
    );
    
    // Help text
    let help = Paragraph::new(
        "Commands: start | pause | resume | set study 1500 | set cycle 4 | quit\n\
         Press Enter to execute | Esc to clear | Type 'help' for more"
    )
    .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(help, chunks[3]);
}