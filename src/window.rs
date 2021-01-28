//! Window manipulation

use std::fmt;
use numtoa::NumToA;

// TODO: add restoring for minimization and maximization respectively
derive_csi_sequence!("Minimize the terminal window.", Minimize, "2t");
derive_csi_sequence!("Minimize the terminal window.", Maxmize, "9;1t");

/// Resize the terminal text area in character size unit
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Resize(pub u16, pub u16);

impl From<Resize> for String {
    fn from(this: Resize) -> String {
        let (mut x, mut y) = ([0u8; 20], [0u8; 20]);
        ["\x1B[8;", this.1.numtoa_str(10, &mut x), ";", this.0.numtoa_str(10, &mut y), "t"].concat()
    }
}

impl Default for Resize {
    fn default() -> Resize {
        use sys::size::terminal_size;
        let (col, row) = terminal_size().unwrap();
        Resize(col, row)
    }
}

impl fmt::Display for Resize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&String::from(*self))
    }
}

/// Move the terminal window
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Move(pub u16, pub u16);

impl From<Move> for String {
    fn from(this: Move) -> String {
        let (mut x, mut y) = ([0u8; 20], [0u8; 20]);
        ["\x1B[3;", this.0.numtoa_str(10, &mut x), ";", this.1.numtoa_str(10, &mut y), "t"].concat()
    }
}

impl Default for Move {
    fn default() -> Move {
        Move(0, 0) // TODO: use current terminal size
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&String::from(*self))
    }
}


