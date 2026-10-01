use std::time::Instant;

pub struct AnimationClock {
    started_at: Instant,
}

impl AnimationClock {
    pub fn new() -> Self {
        Self {
            started_at: Instant::now(),
        }
    }

    pub fn seconds(&self) -> f32 {
        self.started_at.elapsed().as_secs_f32()
    }
}

pub fn bobbing_height(time: f32, phase: f32, amplitude: f32, speed: f32) -> f32 {
    (time * speed + phase).sin() * amplitude
}
