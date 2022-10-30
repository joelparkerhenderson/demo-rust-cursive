//! blt-backend shows how to use the BearLibTerminal backend, 
//! such as for rouge-like games and python-based installation.
//!
//! Available only on crate feature `blt-backend`.
//! 
//! Example section of file `Cargo.toml`:
//! 
//! ```tomly
//! [dependencies]
//! cursive = { version = "*", features = ["blt-backend"] }
//! ```

pub fn main() {
    let mut c = cursive::blt();
    c.run();
}
