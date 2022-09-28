use core::fmt;
use crate::*;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct HSL {
	pub hue: f32,
	pub sat: f32,
	pub light: f32,
}

#[cfg(feature = "dataview")]
unsafe impl dataview::Pod for HSL {}

#[macro_export]
macro_rules! HSL {
	($hue:literal, $sat:literal %, $light:literal %) => {
		$crate::HSL { hue: $hue, sat: $sat, light: $light }
	};
}

impl From<sRGB> for HSL {
	fn from(sRGB { red, green, blue }: sRGB) -> HSL {
		let red = red as f32 / 255.0;
		let green = green as f32 / 255.0;
		let blue = blue as f32 / 255.0;

		let c_max = f32::max(f32::max(red, green), blue);
		let c_min = f32::min(f32::min(red, green), blue);
		let delta = c_max - c_min;

		let hue =
			if delta == 0.0 {
				0.0
			}
			else if c_max == red {
				60.0 * (green - blue) / delta
			}
			else if c_max == green {
				60.0 * ((blue - red) / delta + 2.0)
			}
			else/* if c_max == blue*/ {
				60.0 * ((red - green) / delta + 4.0)
			};

		let light = (c_max + c_min) * 0.5;

		let sat =
			if delta == 0.0 {
				0.0
			}
			else {
				delta / (1.0 - (2.0 * light - 1.0).abs())
			};

		HSL { hue, sat, light }
	}
}

impl From<HSL> for sRGB {
	fn from(HSL { hue, sat, light }: HSL) -> sRGB {
		let hue = hue.rem_euclid(360.0);
		let sat = sat.min(1.0).max(0.0);
		let light = light.min(1.0).max(0.0);

		let c = (1.0 - (2.0 * light - 1.0).abs()) * sat;
		let x = c * (1.0 - ((hue / 60.0).rem_euclid(2.0) - 1.0).abs());
		let m = light - c / 2.0;
		let (r, g, b) =
			if hue < 60.0 {
				(c, x, 0.0)
			}
			else if hue < 120.0 {
				(x, c, 0.0)
			}
			else if hue < 180.0 {
				(0.0, c, x)
			}
			else if hue < 240.0 {
				(0.0, x, c)
			}
			else if hue < 300.0 {
				(x, 0.0, c)
			}
			else/* if hue < 360.0*/ {
				(c, 0.0, x)
			};
		let red = ((r + m) * 255.0) as u8;
		let green = ((g + m) * 255.0) as u8;
		let blue = ((b + m) * 255.0) as u8;
		sRGB { red, green, blue }
	}
}

impl fmt::Display for HSL {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({:.0}°, {:.0}%, {:.0}%)", self.hue, self.sat * 100.0, self.light * 100.0)
	}
}
