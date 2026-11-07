use crossterm::event::KeyCode;

#[derive(Default, PartialEq, Eq)]
pub enum BindState {
    #[default]
    Idle,
    WaitingFirst,
    WaitingSecond,
}

pub struct Config {
    pub key_primary: char,
    pub key_secondary: char,
    pub target_clicks: u32,
    pub state: BindState,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            key_primary: 'z',
            key_secondary: 'x',
            target_clicks: 20,
            state: BindState::Idle,
        }
    }
}

impl Config {
    pub fn selection(&mut self) {
        self.state = BindState::WaitingFirst;
    }

    pub fn is_rebinding(&self) -> bool {
        self.state != BindState::Idle
    }

    pub fn handle_key(&mut self, code: KeyCode) {
        if let KeyCode::Char(c) = code {
            match self.state {
                BindState::WaitingFirst => {
                    self.key_primary = c;
                    self.state = BindState::WaitingSecond;
                }
                BindState::WaitingSecond => {
                    self.key_secondary = c;
                    self.state = BindState::Idle;
                }
                BindState::Idle => {}
            }
        }
    }

    pub fn increase_target(&mut self) {
        self.target_clicks = (self.target_clicks + 5).min(500);
    }

    pub fn decrease_target(&mut self) {
        self.target_clicks = self.target_clicks.saturating_sub(5).max(5);
    }
}
