use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    buffer::Buffer,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use ratatui::style::{Color, Style};
use ratatui_textarea::{TextArea, Input};
use ratatui::widgets::Borders;
use tokio::time::Duration;
use tokio::sync::mpsc::{channel, Sender, Receiver};
use crate::tcp_message::TcpMessage;

#[derive(Debug)]
pub struct App {
    pub counter: u8,
    pub exit: bool,
    pub text: TextArea<'static>,
    pub lines: Vec<Line<'static>>,
}

impl Default for App {
    fn default() -> Self {
        let block = Block::default().borders(Borders::ALL).title("Input");
        let mut text = TextArea::default();
        text.set_block(block);
        text.set_cursor_line_style(
            Style::default().fg(Color::Green),
        );

        Self {
            counter: 0,
            exit: false,
            text,
            lines: Vec::new(),
        }
    }
}
impl App {

    /// runs the application's main loop until the user quits
    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        mut server_to_tui_rx: Receiver<TcpMessage>,
        tui_to_server: Sender<TcpMessage>
    ) -> std::io::Result<()> {
        while !self.exit {
            while let Ok(msg) =server_to_tui_rx.try_recv() {
                self.lines.push(Line::from(msg.to_string()));
            }
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                },
                _ => {}
            };
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.code == KeyCode::Esc && key_event.kind == KeyEventKind::Press {
            self.exit();
        } else {
            self.text.input(key_event);
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Min(0),
                Constraint::Length(3)
            ])
            .split(area);

        let output_title = Line::from("Output".bold());
        let output_block = Block::default().borders(Borders::ALL).title(output_title);

        let input_title = Line::from("Input".bold());
        let input_block = Block::default().borders(Borders::ALL).title(input_title);
        self.text
            .render(layout[1], buf);

        Paragraph::new(self.lines.clone())
            .block(output_block)
            .render(layout[0], buf);
    }
}