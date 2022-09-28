use core::mem;
use crate::*;

/// Linear RGBA with `f32` component values.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct RGBA {
	pub red: f32,
	pub green: f32,
	pub blue: f32,
	pub alpha: f32,
}

#[cfg(feature = "dataview")]
unsafe impl dataview::Pod for RGBA {}

#[inline]
#[allow(non_snake_case)]
pub const fn RGBA(red: f32, green: f32, blue: f32, alpha: f32) -> RGBA {
	RGBA { red, green, blue, alpha }
}

#[macro_export]
macro_rules! RGBA {
	($name:ident) => {{
		let RGB { red, green, blue } = $crate::RGB!($name);
		RGBA { red, green, blue, alpha: 1.0 }
	}};
	($name:ident, $alpha:expr) => {{
		let RGB { red, green, blue } = $crate::RGB!($name);
		RGBA { red, green, blue, alpha: $alpha }
	}};
	// ($red:expr, $green:expr, $blue:expr) => {
	// 	$crate::RGBA { red: $red, green: $green, blue: $blue, alpha: 1.0 }
	// };
	// ($red:expr, $green:expr, $blue:expr, $alpha:expr) => {
	// 	$crate::RGBA { red: $red, green: $green, blue: $blue, alpha: $alpha }
	// };
}

impl RGBA {
	pub const TRANSPARENT: RGBA = RGBA { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 };

	#[inline]
	pub const fn rgb(self) -> RGB {
		RGB {
			red: self.red,
			green: self.green,
			blue: self.blue,
		}
	}
	#[inline]
	pub const fn red(self, red: f32) -> RGBA {
		RGBA { red, green: self.green, blue: self.blue, alpha: self.alpha }
	}
	#[inline]
	pub const fn green(self, green: f32) -> RGBA {
		RGBA { red: self.red, green, blue: self.blue, alpha: self.alpha }
	}
	#[inline]
	pub const fn blue(self, blue: f32) -> RGBA {
		RGBA { red: self.red, green: self.green, blue, alpha: self.alpha }
	}
	#[inline]
	pub const fn alpha(self, alpha: f32) -> RGBA {
		RGBA { red: self.red, green: self.green, blue: self.blue, alpha }
	}
}

impl From<RGB> for RGBA {
	#[inline]
	fn from(RGB { red, green, blue }: RGB) -> RGBA {
		RGBA { red, green, blue, alpha: 1.0 }
	}
}
impl From<(RGB, f32)> for RGBA {
	#[inline]
	fn from((RGB { red, green, blue }, alpha): (RGB, f32)) -> RGBA {
		RGBA { red, green, blue, alpha }
	}
}

impl From<[f32; 3]> for RGBA {
	#[inline]
	fn from([red, green, blue]: [f32; 3]) -> RGBA {
		RGBA { red, green, blue, alpha: 1.0 }
	}
}
impl From<([f32; 3], f32)> for RGBA {
	#[inline]
	fn from(([red, green, blue], alpha): ([f32; 3], f32)) -> RGBA {
		RGBA { red, green, blue, alpha }
	}
}

impl From<[f32; 4]> for RGBA {
	#[inline]
	fn from([red, green, blue, alpha]: [f32; 4]) -> RGBA {
		RGBA { red, green, blue, alpha }
	}
}
impl From<RGBA> for [f32; 4] {
	#[inline]
	fn from(RGBA { red, green, blue, alpha }: RGBA) -> [f32; 4] {
		[red, green, blue, alpha]
	}
}

impl From<sRGBA> for RGBA {
	#[inline]
	fn from(sRGBA { red, green, blue, alpha }: sRGBA) -> RGBA {
		RGBA {
			red: tables::srgb2linear(red),
			green: tables::srgb2linear(green),
			blue: tables::srgb2linear(blue),
			alpha: alpha as f32 / 255.0,
		}
	}
}
impl From<RGBA> for sRGBA {
	#[inline]
	fn from(RGBA { red, green, blue, alpha }: RGBA) -> sRGBA {
		sRGBA {
			red: tables::linear2srgb(red),
			green: tables::linear2srgb(green),
			blue: tables::linear2srgb(blue),
			alpha: (alpha * 255.0) as u8,
		}
	}
}

impl AsRef<[f32; 4]> for RGBA {
	#[inline]
	fn as_ref(&self) -> &[f32; 4] {
		unsafe { mem::transmute(self) }
	}
}
impl AsRef<RGBA> for [f32; 4] {
	#[inline]
	fn as_ref(&self) -> &RGBA {
		unsafe { mem::transmute(self) }
	}
}

impl AsMut<[f32; 4]> for RGBA {
	#[inline]
	fn as_mut(&mut self) -> &mut [f32; 4] {
		unsafe { mem::transmute(self) }
	}
}
impl AsMut<RGBA> for [f32; 4] {
	#[inline]
	fn as_mut(&mut self) -> &mut RGBA {
		unsafe { mem::transmute(self) }
	}
}
