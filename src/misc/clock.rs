use std::time::Instant;

pub struct Clock {
	start: Instant,
	time: f32,
	prev_time: f32,
}

impl Default for Clock {
	fn default() -> Self {
		Self {
			start: Instant::now(),
			time: 0.0,
			prev_time: 0.0,
		}
	}
}

impl Clock {
	pub fn time(&self) -> f32 {
		self.time
	}

	pub fn delta_time(&self) -> f32 {
		self.time - self.prev_time
	}

	pub fn update(&mut self) {
		self.prev_time = self.time;
		self.time = self.start.elapsed().as_secs_f32();
	}
}