# extended stdlib prelude
To use, add to your Crates.io and write
```
extern crate prelude;
use prelude::*;

// you now have access to HashMap, io::Write, str::FromStr, etc.
```

For a full list of the items that are imported, see [lib.rs](src/lib.rs)

This library is to add several "often used" traits, structs and methods
that (in the opinionated version of this author) should have been
included in the stdlib all along. The author of this crate wants the
rust community itself to decide which items should be included here.
If this crate becomes useful enough, it is the intention of this author
to give ownership of this crate to the rustlang-nursery or a similiar
originization.

This library is at the very early stages of development, so stability
is not yet guaranteed. What I mostly want is feedback. Am I missing
items that should "almost always" be imported? Are there ones that
shouldn't be there? Should I be including modules like `io`, `fmt`,
etc as part of the prelude?

If you think any of these things are true, please open an issue!

## Guarantees
I offer the following guarantees for this lib

- Once at v1.0 (fairly soon) all new versions of prelude will increment the
    MAJOR version (1.0.0 -> 2.0.0) since the changes will probably cause
    compiler breakages (things people imported won't need to be imported
    anymore)
