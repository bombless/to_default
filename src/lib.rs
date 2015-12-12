use ::std::default::Default;

pub trait ToDefault: Default {
	fn to_default(&mut self) -> Self {
		::std::mem::replace(self, Default::default())
	}
}

impl<T: Default> ToDefault for T {}
