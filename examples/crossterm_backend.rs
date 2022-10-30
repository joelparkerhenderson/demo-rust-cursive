//! crossterm-backend shows how to use the Crossterm backend, 
//! which is pure Rust and cross-platform Unix and Windows.
//!
//! Available only on crate feature `crossterm-backend`.
//! 
//! Example section of file `Cargo.toml`:
//! 
//! ```tomly
//! [dependencies]
//! cursive = { version = "*", features = ["crossterm-backend"] }
//! ```

pub fn main() {
    let mut c = cursive::crossterm();
    c.run();
}

