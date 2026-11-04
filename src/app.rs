use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::{action::Action, bench::BenchmarkState, config::Config, ui};

#[derive(Default)]
pub struct App {
    pub exit: bool,
    pub bench: BenchmarkState,
    pub config: Config,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        ui::render(self, frame.area(), frame.buffer_mut());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                if self.config.is_rebinding() {
                    self.config.handle_key(key.code);
                } else {
                    let action = self.map_key_to_action(key);
                    self.update(action);
                }
            }
        } else {
            self.update(Action::Tick);
        }
        Ok(())
    }

    fn map_key_to_action(&self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => Action::Quit,
            KeyCode::Enter => Action::StartBenchmark,
            KeyCode::Char('r') => Action::Reset,
            KeyCode::Char('h') => Action::DecreaseTarget,
            KeyCode::Char('l') => Action::IncreaseTarget,
            KeyCode::Char('k') => Action::BindsSelection,
            KeyCode::Char(c) if c == self.config.key_primary || c == self.config.key_secondary => {
                Action::RegisterClick(c)
            }
            _ => Action::None,
        }
    }

    fn update(&mut self, action: Action) {
        match action {
            Action::Quit => self.exit = true,
            Action::BindsSelection => self.config.selection(),
            Action::StartBenchmark => self.bench.start(self.config.target_clicks),
            Action::IncreaseTarget => self.config.increase_target(),
            Action::DecreaseTarget => self.config.decrease_target(),
            Action::RegisterClick(k) => self.bench.register_click(k),
            Action::Reset => self.bench.reset(),
            Action::Tick | Action::None => {}
        }
    }
}
