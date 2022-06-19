pub mod window_info_tracker;

pub const fn as_bytes<T: Sized>(p: &[T]) -> &[u8] {
	let len = std::mem::size_of_val(p);
	let p = p.as_ptr() as *const u8;
	// Safety: The raw parts were from a slice.
	unsafe { std::slice::from_raw_parts(p, len) }
}
