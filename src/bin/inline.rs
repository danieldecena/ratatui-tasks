use color_eyre::Result;
use ratatui::{
    backend::CrosstermBackend,
    Terminal, TerminalOptions, Viewport,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Borders, BorderType, Paragraph},
};
use std::io::stdout;

fn main() -> Result<()> {
    color_eyre::install()?;

    // 1. Tell Ratatui to render INLINE instead of taking over the screen
    // We request exactly 5 lines of height in the current terminal scrollback
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Inline(5),
        },
    )?;

    // 2. Draw our card exactly once
    terminal.draw(|frame| {
        let area = frame.area();

        let text = " \n 🔘 \x1b]8;;agy://mark-done/T08\x1b\\Click to Mark Done (OSC 8)\x1b]8;;\x1b\\";

        let block = Block::default()
            .title(" Card Inside Chat Stream ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan));

        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Left);

        frame.render_widget(paragraph, area);
    })?;

    // 3. We do NOT enter an event loop, we just exit!
    // The card is printed to the chat stream permanently.
    Ok(())
}
