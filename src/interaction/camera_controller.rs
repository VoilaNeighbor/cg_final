use glutin::event::{DeviceEvent, VirtualKeyCode, WindowEvent};
use nalgebra::{Matrix3, Matrix4, Vector3, Vector4};
use crate::Camera;

pub struct CameraController {
	camera: Camera,
	speed: f32,
	angular_speed: f32,
}

impl Default for CameraController {
	fn default() -> Self {
		Self {
			camera: Camera::default(),
			speed: 0.1,
			angular_speed: -0.1,
		}
	}
}

impl CameraController {
	pub fn on_window_event(&mut self, event: &WindowEvent) {
		match event {
			WindowEvent::KeyboardInput { input, .. } => {
				match input.virtual_keycode {
					Some(VirtualKeyCode::W) => self.camera.position += self.camera.direction() * self.speed,
					Some(VirtualKeyCode::S) => self.camera.position -= self.camera.direction() * self.speed,
					Some(VirtualKeyCode::D) => self.camera.position += self.camera.right() * self.speed,
					Some(VirtualKeyCode::A) => self.camera.position -= self.camera.right() * self.speed,
					_ => {}
				}
			}
			_ => {},
		}
	}

	pub fn on_device_event(&mut self, event: &DeviceEvent) {
		if let DeviceEvent::MouseMotion { delta: (x, y), .. } = event {
			let yaw = *x as f32 * self.angular_speed;
			let pitch = *y as f32 * self.angular_speed;
			let yaw = Matrix4::new_rotation(Vector3::new(0.0, yaw, 0.0));
			let pitch = Matrix4::new_rotation(Vector3::new(pitch, 0.0, 0.0));
			let d = self.camera.direction();
			self.camera.set_direction((yaw * pitch * Vector4::new(d.x, d.y, d.z, 1.0)).xyz());
		}
	}

	pub fn camera(&self) -> &Camera {
		&self.camera
	}
}