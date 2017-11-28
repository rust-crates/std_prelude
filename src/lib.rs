//! `import std_prelude::*` so you can be ready to code!
//!
//! # Traits
//! Not having common traits imported is one of the most annoying gotchas in rust when coming from
//! other languages. When you are used to the language, you expect the methods you are used to to
//! always work... not so. Using `Vec::from_iter` is extremely common, but you must first import
//! the `FromIterator` trait.
//!
//! The following are the traits that are exported and why:
//!
//! - `std::ascii::AsciiExt`: adds the `to_ascii_uppercase` onto `&str` types.
//! - `std::fmt::Debug`: allows you to define Debug manually.
//! - `std::fmt::Write as FmtWrite`: adds `write_str` onto byte buffers (such as `String`). Renamed
//!   `FmtWrite` to avoid conflict with `std::io::Write`
//! - `std::io::BufRead`: the `BufRead` trait allows you to use the methods associated with
//!   the `BufReader` struct.
//! - `std::io::Read`: allows you to use `file.read()`
//! - `std::io::Seek`: allows you to use `file.seek()`
//! - `std::io::Write as IoWrite`: allows you to use `file.write()` and `file.write_all()`. Renamed
//!   `IoWrite` to avoid conflict with `std::fmt::Write`
//! - `std::ops::{Deref, DerefMut}`: allows deref through `*v` and also enables Deref coercions
//! - `std::str::FromStr`: allows you to use `type::from_str` constructor for several types. This
//!   is what is implicitly called with `str::parse<_>()`
//!
//! # structs
//! These are extremely commonly used types and it is annoying to have to reimport them all the
//! time.
//!
//! - `std::collections::{BTreeMap, HashMap, HashSet}`: ordered-dict, dict and set
//! - `std::ffi::OsString`: os agnostic (non unicode) string type `std::path::PathBuf` uses this.
//! - `std::fs::File`: for opening files.
//!     - `File::open` to open a file for reading
//!     - `File::write` to open a file for writing
//! - `std::fs::OpenOptions` for more file opening options
//! - `std::fs::ReadDir`: to iterate over the entries in a directory.
//! - `std::io::BufReader`: the BufRead struct wraps `io::Read` using a buffer reducing the number
//!   of OS calls and giving helpful methods
//!   - `read_line()`: read a single line
//!   - `lines()`: return an iterator over all lines.
//!   - `split(byte: u8)`: return an iterator which splits at the chosen byte.
//! - `std::io::BufWriter`: similar to `BufReader`, buffers writes to reduce the number of calls to
//!   the OS. Provides no new methods.
//! - `std::path::{Path, PathBuf}`: specifies an os path.
//! - `std::rc::Rc`: reference counted pointer
//! - `std::sync::Arc`: atomically reference counted pointer
//! - `std::sync::Mutex`: mutual exclusion primitive for threading.
//! - `std::sync::atomic::{AtomicBool, AtomicIsize, AtomicUsize}`: basic atomic types. Good for
//!   unique ids and lots of other use cases.
//! - `std::sync::atomic::Ordering`: necessary for performing operations on atomic types. For
//!   incrementing a counter use `val.fetch_add(1, Ordering::SeqCst)`.
//! - `std::sync::atomic::ATOMIC_USIZE_INIT`: initialized `AtomicUsize` of 0. Use with `static
//!   COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;`
//! - `std::time::Duration`: an amount of time, used for `std::thread::sleep`.
//!
//! # functions
//! These are mostly just "nice to have" functions and it is *really* unlikely that they would ever
//! be overriden.
//!
//! - `std::mem::{size_of, size_of_val}`: get the size of a type. This is safe and common enough
//!   that it should be always available.
//! - `std::thread::sleep`: put the thread to sleep for a `Duration`.
//! - `std::thread::spawn`: spawn a function in a new thread. In rust this is memory safe, so it is
//!   nice to have it always available.

// traits
pub use std::ascii::AsciiExt;
pub use std::fmt::{Debug, Write as FmtWrite};
pub use std::io::{
    BufRead,
    Read, Seek, SeekFrom,
    Write as IoWrite,
};
pub use std::iter::FromIterator;
pub use std::ops::{Deref, DerefMut};
pub use std::str::FromStr;

// structs
pub use std::collections::{BTreeMap, HashMap, HashSet};
pub use std::ffi::OsString;
pub use std::fs::{
    File,
    OpenOptions,
    ReadDir,
};
pub use std::io::{
    BufReader, BufWriter,
};
pub use std::path::{Path, PathBuf};
pub use std::rc::Rc;
pub use std::sync::{Arc, Mutex};
pub use std::sync::atomic::{
    AtomicBool,
    AtomicIsize,
    AtomicUsize,
    Ordering,
    ATOMIC_USIZE_INIT,
};
pub use std::time::Duration;

// functions
pub use std::mem::{size_of, size_of_val};
pub use std::thread::{sleep, spawn};
