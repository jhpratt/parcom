mod any_byte;
pub mod ascii;
mod byte;
mod end_of_input;
mod first_success;
mod int;
mod n_bytes;
mod success;
mod trait_def;
mod utf8_char;
mod verbatim;

pub use utf8_char::utf8_char;

pub use self::any_byte::any_byte;
pub use self::byte::byte;
pub use self::end_of_input::end_of_input;
pub use self::first_success::first_success;
pub use self::int::{int_be, int_le, int_ne};
pub use self::n_bytes::n_bytes;
pub use self::success::success;
pub use self::trait_def::Parser;
pub use self::verbatim::verbatim;

pub type ParserResult<'input, Output, Error> = Result<crate::ParsedItem<'input, Output>, Error>;
