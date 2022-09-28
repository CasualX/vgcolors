use core::{fmt, mem};
use crate::sRGB;

/// Gamma sRGBA with `u8` component values.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct sRGBA {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
	pub alpha: u8,
}

#[cfg(feature = "dataview")]
unsafe impl dataview::Pod for sRGBA {}

#[inline]
#[allow(non_snake_case)]
pub const fn sRGBA(red: u8, green: u8, blue: u8, alpha: u8) -> sRGBA {
	sRGBA { red, green, blue, alpha }
}

#[macro_export]
macro_rules! sRGBA {
	($name:ident) => {{
		let $crate::sRGB { red, green, blue } = $crate::sRGB::$name;
		$crate::sRGBA { red, green, blue, alpha: 255 }
	}};
	($name:ident, $alpha:expr) => {{
		let $crate::sRGB { red, green, blue } = $crate::sRGB::$name;
		$crate::sRGBA { red, green, blue, alpha: $alpha }
	}};
	// ($red:expr, $green:expr, $blue:expr) => {
	// 	$crate::sRGBA { red: $red, green: $green, blue: $blue, alpha: 255 }
	// };
	// ($red:expr, $green:expr, $blue:expr, $alpha:expr) => {
	// 	$crate::sRGBA { red: $red, green: $green, blue: $blue, alpha: $alpha }
	// };
}

impl sRGBA {
	pub const TRANSPARENT: sRGBA = sRGBA { red: 0, green: 0, blue: 0, alpha: 0 };

	/// Drops alpha channel.
	#[inline]
	pub const fn rgb(self) -> sRGB {
		sRGB {
			red: self.red,
			green: self.green,
			blue: self.blue,
		}
	}
	/// Packs sRGBA value into a u32 as `0xAABBGGRR`.
	#[inline]
	pub const fn pack(self) -> u32 {
		self.red as u32 | (self.green as u32) << 8 | (self.blue as u32) << 16 | (self.alpha as u32) << 24
	}
	/// Unpacks `0xAABBGGRR` value.
	#[inline]
	pub const fn unpack(value: u32) -> sRGBA {
		let alpha = ((value >> 24) & 0xff) as u8;
		let blue = ((value >> 16) & 0xff) as u8;
		let green = ((value >> 8) & 0xff) as u8;
		let red = (value & 0xff) as u8;
		sRGBA { red, green, blue, alpha }
	}
}

impl From<sRGB> for sRGBA {
	#[inline]
	fn from(sRGB { red, green, blue }: sRGB) -> Self {
		sRGBA { red, green, blue, alpha: 255 }
	}
}
impl From<(sRGB, u8)> for sRGBA {
	#[inline]
	fn from((sRGB { red, green, blue }, alpha): (sRGB, u8)) -> Self {
		sRGBA { red, green, blue, alpha }
	}
}

impl From<[u8; 3]> for sRGBA {
	#[inline]
	fn from([red, green, blue]: [u8; 3]) -> Self {
		sRGBA { red, green, blue, alpha: 255 }
	}
}
impl From<([u8; 3], u8)> for sRGBA {
	#[inline]
	fn from(([red, green, blue], alpha): ([u8; 3], u8)) -> Self {
		sRGBA { red, green, blue, alpha }
	}
}

impl From<u32> for sRGBA {
	#[inline]
	fn from(value: u32) -> Self {
		sRGBA::unpack(value)
	}
}
impl From<sRGBA> for u32 {
	#[inline]
	fn from(color: sRGBA) -> Self {
		color.pack()
	}
}

impl From<[u8; 4]> for sRGBA {
	#[inline]
	fn from([red, green, blue, alpha]: [u8; 4]) -> Self {
		sRGBA { red, green, blue, alpha }
	}
}
impl From<sRGBA> for [u8; 4] {
	#[inline]
	fn from(sRGBA { red, green, blue, alpha }: sRGBA) -> Self {
		[red, green, blue, alpha]
	}
}

impl AsRef<[u8; 4]> for sRGBA {
	#[inline]
	fn as_ref(&self) -> &[u8; 4] {
		unsafe { mem::transmute(self) }
	}
}
impl AsMut<[u8; 4]> for sRGBA {
	#[inline]
	fn as_mut(&mut self) -> &mut [u8; 4] {
		unsafe { mem::transmute(self) }
	}
}

impl fmt::Display for sRGBA {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let value = (self.alpha as u32) << 24 | (self.red as u32) << 16 | (self.green as u32) << 8 | self.blue as u32;
		write!(f, "#{:8x}", value)
	}
}
impl fmt::UpperHex for sRGBA {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:#10X}", self.pack())
	}
}
impl fmt::LowerHex for sRGBA {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:#10x}", self.pack())
	}
}
