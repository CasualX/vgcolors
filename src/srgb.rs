use core::{cmp, fmt, mem};
use crate::*;

/// Gamma sRGB with `u8` component values.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct sRGB {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
}

#[cfg(feature = "dataview")]
unsafe impl dataview::Pod for sRGB {}

#[inline]
#[allow(non_snake_case)]
pub const fn sRGB(red: u8, green: u8, blue: u8) -> sRGB {
	sRGB { red, green, blue }
}

#[macro_export]
macro_rules! sRGB {
	($name:ident) => {
		$crate::sRGB::$name
	};
	// ($red:expr, $green:expr, $blue:expr) => {
	// 	$crate::sRGB { red: $red, green: $green, blue: $blue }
	// };
}

impl sRGB {
	/// Assigns all components the same value.
	#[inline]
	pub const fn splat(value: u8) -> sRGB {
		sRGB { red: value, green: value, blue: value }
	}
	/// Sets red component.
	#[inline]
	pub const fn red(self, red: u8) -> sRGB {
		sRGB { red, green: self.green, blue: self.blue }
	}
	/// Sets green component.
	#[inline]
	pub const fn green(self, green: u8) -> sRGB {
		sRGB { red: self.red, green, blue: self.blue }
	}
	/// Sets blue component.
	#[inline]
	pub const fn blue(self, blue: u8) -> sRGB {
		sRGB { red: self.red, green: self.green, blue }
	}
	/// Adds alpha channel.
	#[inline]
	pub const fn alpha(self, alpha: u8) -> sRGBA {
		sRGBA { red: self.red, green: self.green, blue: self.blue, alpha }
	}
	/// Packs sRGB value into a u32 as `0xBBGGRR`.
	#[inline]
	pub const fn pack(self) -> u32 {
		self.red as u32 | (self.green as u32) << 8 | (self.blue as u32) << 16
	}
	/// Unpacks `0xBBGGRR` value.
	#[inline]
	pub const fn unpack(value: u32) -> sRGB {
		let red = ((value >> 16) & 0xff) as u8;
		let green = ((value >> 8) & 0xff) as u8;
		let blue = (value & 0xff) as u8;
		sRGB { red, green, blue }
	}
}
impl sRGB {
	#[inline]
	pub const fn luminance(self) -> u8 {
		// Compute luminance in fixed point arith
		const RED_F: u64 = (4294967296.0 * 0.299) as u64;
		const GREEN_F: u64 = (4294967296.0 * 0.587) as u64;
		const BLUE_F: u64 = (4294967296.0 * 0.114) as u64;
		((self.red as u64 * RED_F) >> 32) as u8
			+ ((self.green as u64 * GREEN_F) >> 32) as u8
			+ ((self.blue as u64 * BLUE_F) >> 32) as u8
	}
	#[inline]
	pub const fn grey(self) -> sRGB {
		#[allow(non_snake_case)]
		let Y = self.luminance();
		sRGB { red: Y, green: Y, blue: Y }
	}
	#[inline]
	pub fn min(self, other: sRGB) -> sRGB {
		sRGB {
			red: cmp::min(self.red, other.red),
			green: cmp::min(self.green, other.green),
			blue: cmp::min(self.blue, other.blue),
		}
	}
	#[inline]
	pub fn max(self, other: sRGB) -> sRGB {
		sRGB {
			red: cmp::max(self.red, other.red),
			green: cmp::max(self.green, other.green),
			blue: cmp::max(self.blue, other.blue),
		}
	}
}

impl From<u32> for sRGB {
	#[inline]
	fn from(value: u32) -> Self {
		sRGB::unpack(value)
	}
}
impl From<sRGB> for u32 {
	#[inline]
	fn from(color: sRGB) -> Self {
		color.pack()
	}
}

impl From<[u8; 3]> for sRGB {
	#[inline]
	fn from([red, green, blue]: [u8; 3]) -> Self {
		sRGB { red, green, blue }
	}
}
impl From<sRGB> for [u8; 3] {
	#[inline]
	fn from(sRGB { red, green, blue }: sRGB) -> Self {
		[red, green, blue]
	}
}

impl AsRef<[u8; 3]> for sRGB {
	#[inline]
	fn as_ref(&self) -> &[u8; 3] {
		unsafe { mem::transmute(self) }
	}
}
impl AsRef<sRGB> for [u8; 3] {
	#[inline]
	fn as_ref(&self) -> &sRGB {
		unsafe { mem::transmute(self) }
	}
}

impl AsMut<[u8; 3]> for sRGB {
	#[inline]
	fn as_mut(&mut self) -> &mut [u8; 3] {
		unsafe { mem::transmute(self) }
	}
}
impl AsMut<sRGB> for [u8; 3] {
	#[inline]
	fn as_mut(&mut self) -> &mut sRGB {
		unsafe { mem::transmute(self) }
	}
}

impl fmt::Display for sRGB {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let value = (self.red as u32) << 16 | (self.green as u32) << 8 | self.blue as u32;
		write!(f, "#{:6x}", value)
	}
}
impl fmt::UpperHex for sRGB {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:#8X}", self.pack())
	}
}
impl fmt::LowerHex for sRGB {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:#8x}", self.pack())
	}
}

#[allow(non_upper_case_globals)]
impl sRGB {
	pub const White: sRGB = sRGB(0xff, 0xff, 0xff);
	pub const Silver: sRGB = sRGB(0xc0, 0xc0, 0xc0);
	pub const Gray: sRGB = sRGB(0x80,0x80,0x80);
	pub const Black: sRGB = sRGB(0x00, 0x00, 0x00);
	pub const Red: sRGB = sRGB(0xff, 0x00, 0x00);
	pub const Maroon: sRGB = sRGB(0x80, 0x00, 0x00);
	pub const Yellow: sRGB = sRGB(0xff, 0xff, 0x00);
	pub const Olive: sRGB = sRGB(0x80, 0x80, 0x00);
	pub const Lime: sRGB = sRGB(0x00, 0xff, 0x00);
	pub const Green: sRGB = sRGB(0x00, 0x80, 0x00);
	pub const Aqua: sRGB = sRGB(0x00, 0xff, 0xff);
	pub const Teal: sRGB = sRGB(0x00, 0x80, 0x80);
	pub const Blue: sRGB = sRGB(0x00, 0x00, 0xff);
	pub const Navy: sRGB = sRGB(0x00, 0x00, 0x80);
	pub const Fuchsia: sRGB = sRGB(0xff, 0x00, 0xff);
	pub const Purple: sRGB = sRGB(0x80, 0x00, 0x80);
}
