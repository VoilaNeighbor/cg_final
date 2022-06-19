use nalgebra::{Matrix4, Point3, Vector3};

pub struct Camera {
	pub position: Point3<f32>,
	pub direction: Vector3<f32>,
}

impl Default for Camera {
	fn default() -> Self {
		let position = Point3::new(0.0, 0.0, 3.0);
		let direction = (Vector3::zeros() - position.coords).normalize();
		Self { position, direction }
	}
}

impl Camera {
	pub fn look_at(&self) -> Matrix4<f32> {
		Matrix4::look_at_rh(&self.position, &(self.position + self.direction), &self.up())
	}

	pub fn set_target(&mut self, target: Point3<f32>) {
		self.direction = (target - self.position).normalize();
	}

	pub fn up(&self) -> Vector3<f32> {
		let right = self.direction.cross(&Vector3::y());
		right.cross(&self.direction)
	}
}
