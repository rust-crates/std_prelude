//! `import prelude::*` so you can be ready to code!

// traits
pub use std::ascii::AsciiExt; // to_ascii_uppercase(), etc
pub use std::clone::Clone;
pub use std::convert::AsRef;
pub use std::default::Default;
pub use std::fmt::{Debug, Write as FmtWrite};
pub use std::io::{Read, Seek, SeekFrom, Write};
pub use std::iter::{FromIterator, Iterator};
pub use std::str::FromStr;
pub use std::ops::{Deref, DerefMut};

// structs
pub use std::collections::{HashMap, HashSet};
pub use std::ffi::OsString;
pub use std::path::{Path, PathBuf};
pub use std::rc::Rc;
pub use std::sync::Arc;
