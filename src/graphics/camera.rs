use nalgebra::{Matrix4, Point3, Vector3};

pub struct Camera {
	position: Point3<f32>,
	direction: Vector3<f32>,
	up: Vector3<f32>,
}

impl Default for Camera {
	fn default() -> Self {
		let position = Point3::new(0.0, 0.0, 3.0);
		let direction = (Vector3::zeros() - position.coords).normalize();
		let right = direction.cross(&Vector3::y());
		let up = right.cross(&direction).normalize();
		Self { position, direction, up }
	}
}

impl Camera {
	pub fn look_at(&self) -> Matrix4<f32> {
		Matrix4::look_at_rh(&self.position, &(self.position + self.direction), &self.up)
	}
}
