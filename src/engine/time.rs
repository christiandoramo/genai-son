pub struct TimeState {
    pub last_frame: std::time::Instant,
    pub delta: f32,
    pub elapsed: f32,
}

impl TimeState {
    pub fn new() -> Self {
        Self {
            last_frame: std::time::Instant::now(),
            delta: 0.0,
            elapsed: 0.0,
        }
    }

    pub fn update(&mut self) {
        let now = std::time::Instant::now();
        self.delta = now.duration_since(self.last_frame).as_secs_f32().min(0.05);
        self.last_frame = now;
        self.elapsed += self.delta;
    }
}