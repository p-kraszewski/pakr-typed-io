//! ## Example
//!
//! ### Reading data
//! ```
//! // Manually selected data sizes
//! use pakr_typed_io::*;
//!
//! // Automatically selected data sizes as BE
//! use pakr_typed_io::be::*;
//!
//! use std::io::{Cursor,Seek};
//! use eyre::Result;
//!
//! fn main() -> Result<(),eyre::Report> {
//!     // Prepare source stream
//!     let buf = vec![0, 0, 1, 1, 0xff, 0xff, 0, 0, 1, 1, 1, 1, 0, 0];
//!     let mut cur = Cursor::new(&buf);
//!
//!     // Read single byte from stream
//!     assert_eq!(0, cur.read_u8()?);
//!
//!     // Verify measured size of stream equal to underlying type
//!     assert_eq!(buf.len(), cur.len()?);
//!
//!     // Verify measurement didn't move cursor
//!     assert_eq!(1, cur.tell()?);
//!
//!     // Read another byte
//!     assert_eq!(0, cur.read_i8()?);
//!
//!     // Read two more bytes
//!     assert_eq!(1, cur.read_u8()?);
//!     assert_eq!(1, cur.read_i8()?);
//!
//!     // Verify signed and unsigned read correctly
//!     assert_eq!(255, cur.read_u8()?);
//!     assert_eq!(-1, cur.read_i8()?);
//!
//!     // Verify big-endian and little-endian read correctly
//!     assert_eq!(0x0101, cur.read_u32be()?);
//!     assert_eq!(0x0101, cur.read_u32le()?);
//!
//!     // Verify end-of-data is reported
//!     assert!(cur.read_u8().is_err());
//!
//!     // Rewind data to beginning
//!     cur.rewind() ?;
//!
//!     // Read automatically detected type as big-endian (because we have `use pakr_typed_io::be::*;`)
//!     let x: u32 = cur.read_auto() ?;
//!
//!     // Verify correct endianess
//!     assert_eq!(257, x);
//!
//!     // Verify correct size
//!     assert_eq!(4, cur.tell()?);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Writing data
//! ```
//! // Manually selected data sizes
//! use pakr_typed_io::*;
//!
//! // Automatically selected data sizes as BE
//! use pakr_typed_io::be::*;
//!
//! use eyre::Result;
//! use std::io::{Cursor, Seek};
//!
//! fn main() -> Result<(), eyre::Report> {
//!     let mut buf = vec![];
//!     {
//!         // Wrap in cursor for the scope
//!         let mut cur = Cursor::new(&mut buf);
//!
//!         // Put some automatically sized types
//!         cur.write_auto(0_u8)?;
//!         cur.write_auto(0_i8)?;
//!         cur.write_auto(1_u8)?;
//!         cur.write_auto(1_i8)?;
//!         cur.write_auto(255_u8)?;
//!         cur.write_auto(-1_i8)?;
//!
//!         // Put some manually specified types
//!         cur.write_u32_be(0x0101)?;
//!         cur.write_u32_le(0x0101)?;
//!
//!         // Release cursor
//!     }
//!
//!     // Verify vector contains expected data
//!     assert_eq!(vec![0, 0, 1, 1, 0xff, 0xff, 0, 0, 1, 1, 1, 1, 0, 0], buf);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Verifying data
//!
//! ```
//! // Manually selected data sizes
//! use pakr_typed_io::*;
//!
//! // Automatically selected data sizes as BE
//! use pakr_typed_io::be::*;
//!
//! // Activate type validators
//! use pakr_typed_io::validator::Validator;
//!
//! use eyre::Result;
//! use std::io::{Cursor, Seek};
//!
//! fn main() -> Result<(), eyre::Report> {
//!     // Prepare source stream
//!     let buf = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
//!     let mut cur = Cursor::new(&buf);
//!
//!     assert_eq!(0, cur.read_u8()?.validate_equals(0)?);
//!     assert_eq!(1, cur.read_u8()?.validate_not_equals(0)?);
//!     assert_eq!(2, cur.read_u8()?.validate_in_range(1 .. 4)?);
//!     assert_eq!(3, cur.read_u8()?.validate_not_in_range(10 .. 14)?);
//!     assert_eq!(4, cur.read_u8()?.validate_in_list(&[4, 5, 6])?);
//!     assert_eq!(5, cur.read_u8()?.validate_not_in_list(&[1, 2, 3])?);
//!     assert_eq!(6, cur.read_u8()?.validate_in_array([4, 5, 6])?);
//!     assert_eq!(7, cur.read_u8()?.validate_not_in_array([1, 2, 3])?);
//!     assert_eq!(8, cur.read_u8()?.validate(|v| v % 2 == 0)?);
//!     assert_eq!(9, cur.read_u8()?.validate_not(|v| v % 2 == 0)?);
//!
//!     assert_eq!(
//!         cur.read_u8()?.validate_equals(0).unwrap_err().to_string(),
//!         "value 0xA not equal to expected 0x0"
//!     );
//!
//!     // All validators have version with `_ctx` suffix, having extra
//!     // `&str` parameter bearing extra context for failure message
//!     assert_eq!(
//!         cur.read_u8()?
//!             .validate_equals_ctx(0, "VarX")
//!             .unwrap_err()
//!             .to_string(),
//!         "value VarX=0xB not equal to expected 0x0"
//!     );
//!
//!     // You can chain tests
//!     assert_eq!(
//!         12,
//!         cur.read_u8()?
//!             .validate_in_range(10 .. 15)?
//!             .validate_not_equals(11)?
//!             .validate(|v| v % 2 == 0)?
//!     );
//!     Ok(())
//! }
//! ```

pub mod be;
pub mod le;
pub mod reader;
pub mod tlv;
pub mod validator;
pub mod writer;

pub use reader::*;
pub use writer::*;
