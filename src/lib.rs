//! `use std_prelude::*` so you can be ready to code!
//!
//! # Traits
//! Not having common traits imported is one of the most annoying gotchas in rust when coming from
//! other languages. When you are used to the language, you expect the commonly used methods to
//! always work... not so in rust! Using `Vec::from_iter` is extremely common, but you must first
//! import the `FromIterator` trait.
//!
//! The following are the traits that are exported and why:
//!
//! - **std::ascii::[AsciiExt](trait.AsciiExt.html)**: adds the `to_ascii_uppercase` onto `&str`
//!   types.
//! - **std::borrow::[Borrow](trait.Borrow.html)**: for manually defining the `Borrow` trait.
//! - **std::cmp::{[Ord](enum.Ord.html), [PartialOrd](enum.PartialOrd.html)}**: for manually
//!   defining the Ordering traits and using them in trait bounds.
//! - **std::fmt::[Debug](trait.Debug.html)**: allows you to define Debug manually and use in trait
//!   bounds.
//! - **std::hash::{[Hash](trait.Hash.html), [Hasher](trait.Hasher.html)}**: allows you to
//!   define Hash manually and use in trait bounds.
//! - **std::fmt::[Write as FmtWrite](trait.FmtWrite.html)**: adds `write_str` onto byte buffers
//!   (such as `String`). Renamed `FmtWrite` to avoid conflict with `std::io::Write`
//! - **std::io::[BufRead](trait.BufRead.html)**: the `BufRead` trait allows you to use the methods
//!   associated with the `BufReader` struct (also imported).
//! - **std::io::[Read](trait.Read.html)**: allows you to use `file.read()`
//! - **std::io::[Seek](trait.Seek.html)**: allows you to use `file.seek()`
//! - **std::io::[Write as IoWrite](trait.IoWrite.html)**: allows you to use `file.write()` and
//!   `file.write_all()`. Renamed `IoWrite` to avoid conflict with `std::fmt::Write`
//! - **std::ops::{[Deref](trait.Deref.html), [DerefMut](trait.DerefMut.html)}**: allows deref
//!   through `*v` and also enables Deref coercions
//! - **std::str::[FromStr](trait.FromStr.html)**: allows you to use `type::from_str` constructor
//!   for several types. This is what is implicitly called with `str::parse<_>()`
//!
//! # Structs
//! These are extremely commonly used types and it is annoying to have to reimport them all the
//! time.
//!
//! - **std::borrow::[Cow](struct.Cow.html)**: A clone-on-write smart pointer (often called copy on
//!   write in other languages). This is used by many libraries to be as efficient as possible when
//!   returned data is identical to the data passed in, among other uses.
//! - **std::collections::{[BTreeMap](struct.BTreeMap.html), [HashMap](struct.HashMap.html),
//!   [HashSet](struct.HashSet.html)}**: ordered-dict, dict and set
//! - **std::cmp::[Ordering](enum.Ordering.html)**: the enum type used in the Ordering traits.
//! - **std::ffi::[OsString](struct.OsString.html)**: os agnostic (non unicode) string type.
//!   `std::path::PathBuf` uses this.
//! - **std::fs::[File](struct.File.html)**: for opening files.
//!     - `File::open` to open a file for reading
//!     - `File::write` to open a file for writing
//! - **std::fs::[OpenOptions](struct.OpenOptions.html)**: file opening options.
//! - **std::fs::[ReadDir](struct.ReadDir.html)**: to iterate over the entries in a directory.
//! - **std::io::[BufReader](struct.BufReader.html)**: the BufRead struct wraps `io::Read` using a
//!   buffer reducing the number of OS calls and giving helpful methods
//!   - `read_line()`: read a single line
//!   - `lines()`: return an iterator over all lines.
//!   - `split(byte: u8)`: return an iterator which splits at the chosen byte.
//! - **std::io::[BufWriter](struct.BufWriter.html)**: similar to `BufReader`, buffers writes to
//!   reduce the number of calls to the OS. Provides no new methods.
//!
//! - **std::path::{[Path](struct.Path.html), [PathBuf](struct.PathBuf.html)}**: specifies an os
//!   path.
//! - **std::rc::[Rc](struct.Rc.html)**: reference counted pointer
//! - **std::sync::[Arc](struct.Arc.html)**: atomically reference counted pointer
//! - **std::sync::[Mutex](struct.Mutex.html)**: mutual exclusion primitive for threading.
//! - **std::sync::atomic::{AtomicBool, AtomicIsize, AtomicUsize}**: basic atomic types. Good for
//!   unique ids and lots of other use cases.
//! - **std::sync::atomic::[Ordering as AtomicOrdering](struct.AtomicOrdering.html)**: necessary
//!   for performing operations on atomic types. For incrementing a counter use `val.fetch_add(1,
//!   AtomicOrdering::SeqCst)`. Renamed to not conflict with `std::cmp::Ordering`.
//! - **std::sync::atomic::ATOMIC_USIZE_INIT**: initialized `AtomicUsize` of 0. Use with `static
//!   COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;`
//! - **std::time::[Duration](struct.Duration.html)**: an amount of time, used for
//!   `std::thread::sleep`.
//!
//! # Functions
//! These are mostly just "nice to have" functions and it is *really* unlikely that they would ever
//! be overriden.
//!
//! - **std::cmp::{[max](fn.max.html), [min](fn.min.html)}**: get the max or min of two comparable integers.
//! - **std::mem::{[size_of](fn.size_of.html), [size_of_val](fn.size_of_val.html)}**: get the size
//!   of a type. This is safe and common enough that it should be always available.
//! - **std::thread::[sleep](fn.sleep.html)**: put the thread to sleep for a `Duration`.
//! - **std::thread::[spawn](fn.spawn.html)**: spawn a function in a new thread. In rust this is
//!   memory safe, so it is nice to have it always available.
//!
//! # Modules (_primitive type_ only)
//! The following modules are imported so that it is easy to access their relevant constants and
//! constructors.
//!
//! - **u8 u16 u64 usize**: unsigned integer modules with `MAX` and `MIN`
//! - **i8 i16 i64 isize**: signed integer modules with `MAX` and `MIN`
//! - **f32 f64**: floating point modules with not just `MAX` and `MIN` but also `NAN`, `INFINITY`,
//!   etc as well as a [f32::consts](f32/consts/index.html) and [f64::consts](f64/consts/index.html)
//!   modules with basic mathematical constants like `PI` and `E`.
//! - [**str**](str/index.html): core string type with [`from_utf8`](str/fn.from_utf8.html)
//!   function.

// ------------------------------
// ----------- TRAITS -----------
pub use std::ascii::AsciiExt;
pub use std::borrow::Borrow;
pub use std::cmp::{
    Ord, PartialOrd, Ordering
};
pub use std::fmt::{Debug, Write as FmtWrite};
pub use std::hash::{Hash, Hasher};
pub use std::io::{
    BufRead,
    Read, Seek, SeekFrom,
    Write as IoWrite,
};
pub use std::iter::FromIterator;
pub use std::ops::{Deref, DerefMut};
pub use std::str::FromStr;


// -------------------------------
// ----------- STRUCTS -----------
pub use std::borrow::Cow;
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
    Ordering as AtomicOrdering,
    ATOMIC_USIZE_INIT,
};
pub use std::time::Duration;


// ---------------------------------
// ----------- FUNCTIONS -----------
pub use std::mem::{size_of, size_of_val};
pub use std::thread::{sleep, spawn};


// ---------------------------------
// ----------- MODULES -----------

pub use std::u8;
pub use std::u16;
pub use std::u64;
pub use std::usize;
pub use std::i8;
pub use std::i16;
pub use std::i64;
pub use std::isize;
pub use std::f32;
pub use std::f64;
pub use std::str;
