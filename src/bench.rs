use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct BenchmarkState {
    pub is_active: bool,
    pub current_clicks: u32,
    pub target_clicks: u32,
    pub timestamps: Vec<Instant>,
    pub bpm: u16,
    pub ur: u16,
    pub last_key: Option<char>,
}

impl Default for BenchmarkState {
    fn default() -> Self {
        Self {
            is_active: false,
            current_clicks: 0,
            target_clicks: 20,
            timestamps: Vec::new(),
            bpm: 0,
            ur: 0,
            last_key: None,
        }
    }
}

impl BenchmarkState {
    pub fn start(&mut self, target: u32) {
        self.is_active = true;
        self.current_clicks = 0;
        self.target_clicks = target;
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
            self.bpm();
            self.is_active = false;
        }
    }

    fn bpm(&mut self) {
        if self.timestamps.len() < 2 {
            self.bpm = 0;
            return;
        };

        let mut deltas: Vec<_> = self.timestamps.windows(2).map(|w| w[1] - w[0]).collect();

        deltas.sort_unstable();

        let len = deltas.len();
        let mid = len / 2;

        let median_dt_sec = if len % 2 == 0 {
            (deltas[mid - 1].as_secs_f32() + deltas[mid].as_secs_f32()) / 2.0
        } else {
            deltas[mid].as_secs_f32()
        };

        self.bpm = if median_dt_sec > 0.0 {
            (15.0 / median_dt_sec).round() as u16
        } else {
            0
        };

        self.ur(deltas);
    }

    fn ur(&mut self, deltas: Vec<Duration>) {
        let ms: Vec<f64> = deltas.iter().map(|d| d.as_secs_f64() * 1000.0).collect();
        let n = ms.len() as f64;

        let mean = ms.iter().sum::<f64>() / n;
        let variance = ms.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
        let std = variance.sqrt();

        self.ur = (std / std::f64::consts::SQRT_2 * 10.0).round() as u16;
    }

    pub fn reset(&mut self) {
        self.is_active = false;
        self.current_clicks = 0;
        self.timestamps.clear();
        self.last_key = None;
    }
}
