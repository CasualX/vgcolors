use core::mem;
use crate::*;

/// Linear RGB with `f32` component values.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct RGB {
	pub red: f32,
	pub green: f32,
	pub blue: f32,
}

#[cfg(feature = "dataview")]
unsafe impl dataview::Pod for RGB {}

#[inline]
#[allow(non_snake_case)]
pub const fn RGB(red: f32, green: f32, blue: f32) -> RGB {
	RGB { red, green, blue }
}

#[macro_export]
macro_rules! RGB {
	($name:ident) => {
		<$crate::RGB as From<$crate::sRGB>>::from($crate::sRGB::$name)
	};
	// ($red:expr, $green:expr, $blue:expr) => {
	// 	$crate::RGB { red: $red, green: $green, blue: $blue }
	// };
}

impl RGB {
	/// Assigns all components the same value.
	#[inline]
	pub const fn splat(value: f32) -> RGB {
		RGB { red: value, green: value, blue: value }
	}
	/// Sets red component.
	#[inline]
	pub const fn red(self, red: f32) -> RGB {
		RGB { red, green: self.green, blue: self.blue }
	}
	/// Sets green component.
	#[inline]
	pub const fn green(self, green: f32) -> RGB {
		RGB { red: self.red, green, blue: self.blue }
	}
	/// Sets blue component.
	#[inline]
	pub const fn blue(self, blue: f32) -> RGB {
		RGB { red: self.red, green: self.green, blue }
	}
	/// Adds alpha channel.
	#[inline]
	pub const fn alpha(self, alpha: f32) -> RGBA {
		RGBA { red: self.red, green: self.green, blue: self.blue, alpha }
	}
}
impl RGB {
	#[inline]
	pub fn luminance(self) -> f32 {
		0.2126 * self.red + 0.7152 * self.green + 0.0722 * self.blue
	}
	#[inline]
	pub fn grey(self) -> RGB {
		#[allow(non_snake_case)]
		let Y = self.luminance();
		RGB { red: Y, green: Y, blue: Y }
	}
	/// Clamps the component values to `[0.0, 1.0]`.
	#[inline]
	pub fn clamp(self) -> RGB {
		RGB {
			red: self.red.min(1.0).max(0.0),
			green: self.green.min(1.0).max(0.0),
			blue: self.blue.min(1.0).max(0.0),
		}
	}
	#[inline]
	pub fn mix(self, other: RGB, alpha: f32) -> RGB {
		RGB {
			red: self.red + (other.red - self.red) * alpha,
			green: self.green + (other.green - self.green) * alpha,
			blue: self.blue + (other.blue - self.blue) * alpha,
		}
	}
	#[inline]
	pub fn min(self, other: RGB) -> RGB {
		RGB {
			red: f32::min(self.red, other.red),
			green: f32::min(self.green, other.green),
			blue: f32::min(self.blue, other.blue),
		}
	}
	#[inline]
	pub fn max(self, other: RGB) -> RGB {
		RGB {
			red: f32::max(self.red, other.red),
			green: f32::max(self.green, other.green),
			blue: f32::max(self.blue, other.blue),
		}
	}
	#[inline]
	pub fn multiply(self, other: RGB) -> RGB {
		RGB {
			red: self.red * other.red,
			green: self.green * other.green,
			blue: self.blue * other.blue,
		}
	}
	#[inline]
	pub fn screen(self, other: RGB) -> RGB {
		RGB {
			red: 1.0 - (1.0 - self.red) * (1.0 - other.red),
			green: 1.0 - (1.0 - self.green) * (1.0 - other.green),
			blue: 1.0 - (1.0 - self.blue) * (1.0 - other.blue),
		}
	}
	#[inline]
	pub fn add(self, other: RGB) -> RGB {
		RGB {
			red: self.red + other.red,
			green: self.green + other.green,
			blue: self.blue + other.blue,
		}
	}
}

impl From<[f32; 3]> for RGB {
	#[inline]
	fn from([red, green, blue]: [f32; 3]) -> RGB {
		RGB { red, green, blue }
	}
}
impl From<RGB> for [f32; 3] {
	#[inline]
	fn from(RGB { red, green, blue }: RGB) -> [f32; 3] {
		[red, green, blue]
	}
}

impl From<sRGB> for RGB {
	#[inline]
	fn from(sRGB { red, green, blue }: sRGB) -> RGB {
		RGB {
			red: tables::srgb2linear(red),
			green: tables::srgb2linear(green),
			blue: tables::srgb2linear(blue),
		}
	}
}
impl From<RGB> for sRGB {
	#[inline]
	fn from(RGB { red, green, blue }: RGB) -> sRGB {
		sRGB {
			red: tables::linear2srgb(red),
			green: tables::linear2srgb(green),
			blue: tables::linear2srgb(blue),
		}
	}
}

impl AsRef<[f32; 3]> for RGB {
	#[inline]
	fn as_ref(&self) -> &[f32; 3] {
		unsafe { mem::transmute(self) }
	}
}
impl AsRef<RGB> for [f32; 3] {
	#[inline]
	fn as_ref(&self) -> &RGB {
		unsafe { mem::transmute(self) }
	}
}

impl AsMut<[f32; 3]> for RGB {
	#[inline]
	fn as_mut(&mut self) -> &mut [f32; 3] {
		unsafe { mem::transmute(self) }
	}
}
impl AsMut<RGB> for [f32; 3] {
	#[inline]
	fn as_mut(&mut self) -> &mut RGB {
		unsafe { mem::transmute(self) }
	}
}
