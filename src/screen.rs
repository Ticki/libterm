//! Managing switching between main and alternate screen buffers.
//!
//! Note that this implementation uses xterm's new escape sequences for screen switching and thus
//! only works for xterm compatible terminals (which should be most terminals nowadays).
//!
//! # Example
//!
//! ```rust
//! use termion::screen::IntoAlternateScreen;
//! use std::io::{Write, stdout};
//!
//! fn main() {
//!     {
//!         let mut screen = stdout().into_alternate_screen().unwrap();
//!         write!(screen, "Writing to alternate screen!").unwrap();
//!         screen.flush().unwrap();
//!     }
//!     println!("Writing to main screen.");
//! }
//! ```

use std::io::{self, Write};
use std::ops;
use std::fmt;

/// Switch to the main screen buffer of the terminal.
pub struct ToMainScreen;

impl fmt::Display for ToMainScreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, csi!("?1049l"))
    }
}

/// Switch to the alternate screen buffer of the terminal.
pub struct ToAlternateScreen;

impl fmt::Display for ToAlternateScreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, csi!("?1049h"))
    }
}

/// A terminal restorer, which wraps a type implementing Write, and causes all writes to be written
/// to an alternate screen.
///
/// This is achieved by switching the terminal to the alternate screen on creation and
/// automatically switching it back to the original screen on drop.
pub struct AlternateScreen<W: Write> {
    /// The output target.
    output: W,
}

/// Extension trait for writers, providing the `into_alternate_screen` function.
pub trait IntoAlternateScreen: Write + Sized {
    /// Switch the terminal controlled by this writer to use the alternate screen. The terminal will be
    /// restored to the main screen when the `AlternateScreen` returned by this function is
    /// dropped.
    fn into_alternate_screen(mut self) -> io::Result<AlternateScreen<Self>> {
        write!(self, "{}", ToAlternateScreen)?;
        Ok(AlternateScreen { output: self })
    }
}

impl<W: Write> IntoAlternateScreen for W {}

impl<W: Write> Drop for AlternateScreen<W> {
    fn drop(&mut self) {
        let _ = write!(self, "{}", ToMainScreen);
    }
}

impl<W: Write> ops::Deref for AlternateScreen<W> {
    type Target = W;

    fn deref(&self) -> &W {
        &self.output
    }
}

impl<W: Write> ops::DerefMut for AlternateScreen<W> {
    fn deref_mut(&mut self) -> &mut W {
        &mut self.output
    }
}

impl<W: Write> Write for AlternateScreen<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}
