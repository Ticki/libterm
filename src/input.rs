use std::io::{self, Read, Write};

use IntoRawMode;

#[cfg(feature = "nightly")]
use std::io::{Chars, CharsError};

/// A key.
#[derive(Debug)]
pub enum Key {
    /// Backspace.
    Backspace,
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// Normal character.
    Char(char),
    /// Alt modified character.
    Alt(char),
    /// Ctrl modified character.
    ///
    /// Note that certain keys may not be modifiable with `ctrl`, due to limitations of terminals.
    Ctrl(char),
    /// Invalid character code.
    Invalid,
    /// IO error.
    Error(io::Error),
    /// Null byte.
    Null,

    #[allow(missing_docs)]
    #[doc(hidden)]
    __IsNotComplete
}

/// An iterator over input keys.
#[cfg(feature = "nightly")]
pub struct Keys<I> {
    chars: I,
}

#[cfg(feature = "nightly")]
impl<I: Iterator<Item = Result<char, CharsError>>> Iterator for Keys<I> {
    type Item = Key;

    fn next(&mut self) -> Option<Key> {
        match self.chars.next() {
            Some(Ok('\x1B')) => Some(match self.chars.next() {
                Some(Ok('[')) => match self.chars.next() {
                    Some(Ok('D')) => Key::Left,
                    Some(Ok('C')) => Key::Right,
                    Some(Ok('A')) => Key::Up,
                    Some(Ok('B')) => Key::Down,
                    _ => Key::Invalid,
                },
                Some(Ok(c)) => Key::Alt(c),
                Some(Err(_)) | None => Key::Invalid,
            }),
            Some(Ok('\n')) | Some(Ok('\r')) => Some(Key::Char('\n')),
            Some(Ok('\t')) => Some(Key::Char('\t')),
            Some(Ok('\x7F')) => Some(Key::Backspace),
            Some(Ok(c @ '\x01' ... '\x1A')) => Some(Key::Ctrl((c as u8 - 0x1  + b'a') as char)),
            Some(Ok(c @ '\x1C' ... '\x1F')) => Some(Key::Ctrl((c as u8 - 0x1C + b'4') as char)),
            None => None,
            Some(Ok('\0')) => Some(Key::Null),
            Some(Ok(c)) => Some(Key::Char(c)),
            Some(Err(e)) => Some(Key::Error(io::Error::new(io::ErrorKind::InvalidData, e))),
        }
    }
}

/// Extension to `Read` trait.
pub trait TermRead {
    /// An iterator over key inputs.
    #[cfg(feature = "nightly")]
    fn keys(self) -> Keys<Chars<Self>> where Self: Sized;

    /// Read a password.
    ///
    /// EOT and ETX will abort the prompt, returning `None`. Newline or carriage return will
    /// complete the password input.
    fn read_passwd<W: Write>(&mut self, writer: &mut W) -> io::Result<Option<String>>;
}

impl<R: Read> TermRead for R {
    #[cfg(feature = "nightly")]
    fn keys(self) -> Keys<Chars<R>> {
        Keys {
            chars: self.chars(),
        }
    }

    fn read_passwd<W: Write>(&mut self, writer: &mut W) -> io::Result<Option<String>> {
        let _raw = try!(writer.into_raw_mode());
        let mut passbuf = Vec::with_capacity(30);

        for c in self.bytes() {
            match c {
                Err(e) => return Err(e),
                Ok(0) | Ok(3) | Ok(4) => return Ok(None),
                Ok(b'\n') | Ok(b'\r') => break,
                Ok(c) => passbuf.push(c),
            }
        }

        let passwd = try!(String::from_utf8(passbuf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)));

        Ok(Some(passwd))
    }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "nightly")]
    #[test]
    fn test_keys() {
        use {TermRead, Key};

        let mut i = b"\x1Bayo\x7F\x1B[D".keys();

        assert_eq!(i.next(), Some(Key::Alt('a')));
        assert_eq!(i.next(), Some(Key::Char('y')));
        assert_eq!(i.next(), Some(Key::Char('o')));
        assert_eq!(i.next(), Some(Key::Backspace));
        assert_eq!(i.next(), Some(Key::Left));
        assert_eq!(i.next(), None);
    }
}
