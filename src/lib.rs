/*!
Video game colors library.
 */

mod srgb;
mod srgba;
mod rgb;
mod rgba;
mod hsl;

mod tables;

pub use self::srgb::*;
pub use self::srgba::*;

pub use self::rgb::*;
pub use self::rgba::*;

pub use self::hsl::*;
