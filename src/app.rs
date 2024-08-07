use std::io;

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    Frame,
};

use crate::tui;

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frames(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frames(&self, frame: &mut Frame) {
        // let layout = Layout::default()
        //     .direction(Direction::Vertical)
        //     .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        //     .split(frame.area());
        // frame.render_widget(self, layout[0]);
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('Q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Pomodoro Timer ".red().bold());
        let instructions = Title::from(Line::from(vec![
            " Start ".bold().red(),
            "<S>".bold(),
            " Stop ".bold().red(),
            "<T>".bold(),
            " Restart ".bold().red(),
            "<R>".bold(),
            " Break ".bold().red(),
            "<B> ".bold(),
        ]));
        let block_instructions = Block::bordered()
            .title(
                title
                    .alignment(ratatui::layout::Alignment::Center)
                    .position(Position::Top),
            )
            .title(
                instructions
                    .alignment(ratatui::layout::Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::DOUBLE);
        let text = Text::from(vec![Line::from(vec![
            " Text -> ".into(),
            self.exit.to_string().yellow(),
        ])]);
        Paragraph::new(text)
            .centered()
            .block(block_instructions)
            .render(area, buf);
    }
}
