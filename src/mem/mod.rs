//! An in-memory filesystem.
//!
//! The [`FS`] provides an in-memory file system. Only a Unix implementation is currently
//! available. Errors returned attempt to mimic true operating sytsem error codes, but may not
//! catch subtle differences between operating systems.
//!
//! This module is platform specific and uses the proper in-memory semantics via a `pub use`
//! depending on the builder's operating system. As mentioned above, only Unix is currently
//! supported, meaning _this_ module will not work on Windows. To get a platform agnostic in-memory
//! filesystem, use the proper platform specific module. For example, if you use
//! [`rsfs::mem::unix`], you will have a cross-platform in-memory filesystem that obeys Unix
//! semantics. When `rsfs::mem::windows` is written, that can be used to get a cross-platform
//! Windows specific in-memory filesystem (additionally, once it is written, _this_ module will
//! work on Windows systems).
//!
//! This module should provide a decent alternative to FUSE if there is no need to use your in
//! memory filesystem outside of your process.
//!
//! # Example
//!
//! ```
//! use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, SeekFrom};
//! use std::path::PathBuf;
//!
//! use rsfs::*;
//! use rsfs::mem::FS;
//!
//! # async fn foo() -> std::io::Result<()> {
//! let fs = FS::new();
//! assert!(fs.create_dir_all("a/b/c").await.is_ok());
//!
//! let mut wf = fs.create_file("a/f").await?;
//! assert_eq!(wf.write(b"hello").await?, 5);
//!
//! let mut rf = fs.open_file("a/f").await?;
//! let mut output = [0u8; 5];
//! assert_eq!(rf.read(&mut output).await?, 5);
//! assert_eq!(&output, b"hello");
//! # Ok(())
//! # }
//! ```
//!
//! [`FS`]: struct.FS.html
//! [`errors`]: ../errors/index.html

pub mod unix;
pub use unix::*;

// TODO mod test;
