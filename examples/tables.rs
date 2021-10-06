use std::{env, fs};
use std::path::Path;
use std::io::*;

fn srgb2linear(c: f64) -> f64 {
	if c <= (12.92 * 0.0031308) {
		c / 12.92
	}
	else {
		((c + 0.055) / 1.055).powf(2.4)
	}
}
fn linear2srgb(c: f64) -> f64 {
	if c <= 0.0031308 {
		12.92 * c
	} else {
		1.055 * c.powf(1f64 / 2.4) - 0.055
	}
}

fn print<F: Fn(f64) -> f64>(w: &mut dyn Write, name: &str, f: F) {
	let len = 256;
	let _ = write!(w, "static {}: [f32; {}] = [", name, len);

	for i in 0..len {
		let c = i as f64 / 255.0;
		let x = f(c);
		let y = (x * 255.0).round() as u8;

		let _ = write!(w, "{},", x as f32);
	}

	let _ = writeln!(w, "];");
}

fn main() {
	let stdout = std::io::stdout();
	let mut stdout = stdout.lock();

	print(&mut stdout, "SRGB2LINEAR", srgb2linear);
	print(&mut stdout, "LINEAR2SRGB", linear2srgb);
}
