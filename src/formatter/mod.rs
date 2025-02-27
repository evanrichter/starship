pub mod model;
mod parser;
pub mod string_formatter;
mod version;

pub use model::{StyleVariableHolder, VariableHolder};
pub use string_formatter::StringFormatter;
pub use version::VersionFormatter;

#[cfg(fuzzing)]
pub fn parse(data: &str) {
    _ = parser::parse(data);
}
