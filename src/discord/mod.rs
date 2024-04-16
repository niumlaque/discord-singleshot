pub mod error;
mod send_once;

pub use error::{Error, Result};
pub use send_once::SendOnce;

/// Provides a function to split a string
pub trait StringSplitter {
    /// Returns a slice of the split strings
    fn get(&self) -> &[String];
}

/// Splits the string and retains it in the specified character count
pub struct LengthBasedSplitter {
    splitted: Vec<String>,
}

impl LengthBasedSplitter {
    pub fn new(s: impl AsRef<str>, len: usize) -> Self {
        let s = s.as_ref();
        let splitted = s
            .chars()
            .collect::<Vec<_>>()
            .chunks(len)
            .map(|x| x.iter().collect())
            .collect();
        Self { splitted }
    }
}

impl StringSplitter for LengthBasedSplitter {
    fn get(&self) -> &[String] {
        &self.splitted
    }
}
