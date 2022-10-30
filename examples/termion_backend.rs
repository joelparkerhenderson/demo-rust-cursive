//! termion-backend shows how to use the termion backend, 
//! which is pure Rust, and runs on Unix (not Windows).
//!
//! Available only on crate feature `termion-backend`.
//! 
//! Example section of file `Cargo.toml`:
//! 
//! ```tomly
//! [dependencies]
//! cursive = { version = "*", features = ["termion-backend"] }
//! ```

pub fn main() {
    let mut c = cursive::termion();
    c.run();
}
