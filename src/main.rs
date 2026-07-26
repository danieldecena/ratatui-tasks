use color_eyre::Result;
use ratatui::{
    crossterm::event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEventKind,
    },
    crossterm::execute,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    DefaultTerminal,
};
use std::fs;
use std::io::stdout;

const DB_PATH: &str = "/Users/home/Claude Desktop/personal-tasks/tasks-db.md";

struct Task {
    id: String,
    title: String,
    is_done: bool,
    click_area: Rect,
}

struct App {
    exit: bool,
    tasks: Vec<Task>,
    scroll_offset: usize,
}

impl App {
    fn new() -> Self {
        Self {
            exit: false,
            tasks: Self::load_tasks(),
            scroll_offset: 0,
        }
    }

    fn load_tasks() -> Vec<Task> {
        let content = fs::read_to_string(DB_PATH).unwrap_or_default();
        let mut tasks = Vec::new();
        
        for line in content.lines() {
            // Looking for table rows that start with the ID column
            if line.starts_with("| T") {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 11 {
                    let id = parts[1].trim().to_string();
                    let title = parts[2].trim().to_string();
                    let status = parts[10].trim();
                    let is_done = status.eq_ignore_ascii_case("done");
                    
                    tasks.push(Task {
                        id,
                        title,
                        is_done,
                        click_area: Rect::default(),
                    });
                }
            }
        }
        tasks
    }

    fn toggle_task(&mut self, task_index: usize) {
        let task = &mut self.tasks[task_index];
        task.is_done = !task.is_done;
        
        let new_status = if task.is_done { " Done " } else { " Active " };
        let task_id = task.id.clone();
        
        // Update the file
        if let Ok(content) = fs::read_to_string(DB_PATH) {
            let mut new_lines = Vec::new();
            for line in content.lines() {
                if line.starts_with(&format!("| {} ", task_id)) || line.starts_with(&format!("| {}|", task_id)) {
                    let mut parts: Vec<&str> = line.split('|').collect();
                    if parts.len() >= 11 {
                        parts[10] = new_status;
                        new_lines.push(parts.join("|"));
                    } else {
                        new_lines.push(line.to_string());
                    }
                } else {
                    new_lines.push(line.to_string());
                }
            }
            let _ = fs::write(DB_PATH, new_lines.join("\n") + "\n");
        }
    }

    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut ratatui::Frame) {
        let area = frame.area();

        // Let's cap the number of tasks we draw to what fits on the screen
        let available_height = area.height.saturating_sub(4); // minus margins and header
        let max_visible_tasks = (available_height / 4) as usize;
        let visible_tasks = std::cmp::min(max_visible_tasks, self.tasks.len());

        let mut constraints = vec![Constraint::Length(2)]; // Header
        for _ in 0..visible_tasks {
            constraints.push(Constraint::Length(4)); // Task height
        }
        constraints.push(Constraint::Min(0)); // Fill the rest

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .horizontal_margin(4)
            .constraints(constraints)
            .split(area);

        // Header
        let header = Paragraph::new(Line::from(vec![
            Span::styled("My Live Tasks ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled("(Syncs with tasks-db.md - Click circles to toggle!)", Style::default().fg(Color::DarkGray)),
        ]));
        frame.render_widget(header, chunks[0]);

        // Draw tasks
        for (i, task) in self.tasks.iter_mut().take(visible_tasks).enumerate() {
            let task_area = chunks[i + 1];

            let (icon, color) = if task.is_done {
                ("✓", Color::Green)
            } else {
                ("○", Color::Red)
            };

            let text = vec![
                Line::from(vec![
                    Span::styled(format!(" {} ", icon), Style::default().fg(color).add_modifier(Modifier::BOLD)),
                    Span::styled(format!("{} - {}", task.id, task.title), Style::default().fg(Color::White)),
                ])
            ];

            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if task.is_done { Color::DarkGray } else { Color::White }));

            let paragraph = Paragraph::new(text).block(block);
            frame.render_widget(paragraph, task_area);

            // Hitbox
            task.click_area = Rect {
                x: task_area.x,
                y: task_area.y,
                width: 5,
                height: task_area.height,
            };
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                        self.exit = true;
                    }
                }
                Event::Mouse(mouse) => {
                    if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                        let mut task_to_toggle = None;
                        
                        for (index, task) in self.tasks.iter().enumerate() {
                            let in_x = mouse.column >= task.click_area.x && mouse.column < task.click_area.x + task.click_area.width;
                            let in_y = mouse.row >= task.click_area.y && mouse.row < task.click_area.y + task.click_area.height;
                            
                            if in_x && in_y {
                                task_to_toggle = Some(index);
                                break;
                            }
                        }
                        
                        if let Some(index) = task_to_toggle {
                            self.toggle_task(index);
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    execute!(stdout(), EnableMouseCapture)?;
    let terminal = ratatui::init();
    
    let mut app = App::new();
    let result = app.run(terminal);
    
    let _ = execute!(stdout(), DisableMouseCapture);
    ratatui::restore();
    
    result
}
