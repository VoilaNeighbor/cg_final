use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct Locator {
	objects: HashMap<TypeId, Box<dyn Any>>,
}

impl Locator {
	pub fn add<T: Any>(&mut self, object: T) {
		self.objects.insert(object.type_id(), Box::new(object));
	}

	pub fn find<T: Any>(&self) -> Option<&T> {
		self.objects.get(&TypeId::of::<T>()).and_then(|o| o.downcast_ref())
	}

	pub fn find_mut<T: Any>(&mut self) -> Option<&mut T> {
		self.objects.get_mut(&TypeId::of::<T>()).and_then(|o| o.downcast_mut())
	}
}
