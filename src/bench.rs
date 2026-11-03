use std::time::Instant;

#[derive(Debug, Clone)]
pub struct BenchmarkState {
    pub is_active: bool,
    pub target_clicks: u32,
    pub current_clicks: u32,
    pub timestamps: Vec<Instant>,
    pub result: u16,
    pub last_key: Option<char>,
}

impl Default for BenchmarkState {
    fn default() -> Self {
        Self {
            is_active: false,
            target_clicks: 20,
            current_clicks: 0,
            timestamps: Vec::new(),
            result: 0,
            last_key: None,
        }
    }
}

impl BenchmarkState {
    pub fn start(&mut self) {
        self.is_active = true;
        self.current_clicks = 0;
        self.timestamps.clear();
        self.last_key = None;
    }

    pub fn register_click(&mut self, key: char) {
        if !self.is_active {
            return;
        }

        self.current_clicks += 1;
        self.timestamps.push(Instant::now());
        self.last_key = Some(key);

        if self.current_clicks >= self.target_clicks {
            self.result();
            self.is_active = false;
        }
    }

    fn result(&mut self) {
        if self.timestamps.len() < 2 {
            self.result = 0;
            return;
        };

        let mut deltas: Vec<_> = self.timestamps
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect();

        deltas.sort_unstable();

        let len = deltas.len();
        let mid = len / 2;

        let median_dt_sec =  if len % 2 == 0 {
            (deltas[mid -1].as_secs_f32() + deltas[mid].as_secs_f32()) / 2.0
        } else {
            deltas[mid].as_secs_f32()
        };

        self.result = if median_dt_sec > 0.0 {
            (15.0 / median_dt_sec).round() as u16
        } else {
                0
            };

    }

    pub fn reset(&mut self) {
        self.is_active = false;
        self.current_clicks = 0;
        self.timestamps.clear();
        self.last_key = None;
    }
}
