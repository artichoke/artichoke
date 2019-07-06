#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(ptr_wrapping_offset_from)]





extern crate c2rust_bitfields;
extern crate f128;
extern crate libc;


#[path = "src/codedump.rs"]
pub mod codedump;
#[path = "src/state.rs"]
pub mod state;
#[path = "src/init.rs"]
pub mod init;
#[path = "src/compar.rs"]
pub mod compar;
#[path = "src/array.rs"]
pub mod array;
#[path = "src/load.rs"]
pub mod load;
#[path = "src/enum.rs"]
pub mod enum;
#[path = "src/debug.rs"]
pub mod debug;
#[path = "src/symbol.rs"]
pub mod symbol;
#[path = "src/object.rs"]
pub mod object;
#[path = "src/string.rs"]
pub mod string;
#[path = "src/dump.rs"]
pub mod dump;
#[path = "src/numeric.rs"]
pub mod numeric;
#[path = "src/crc.rs"]
pub mod crc;
#[path = "src/error.rs"]
pub mod error;
#[path = "src/class.rs"]
pub mod class;
#[path = "src/fmt_fp.rs"]
pub mod fmt_fp;
#[path = "src/mrblib.rs"]
pub mod mrblib;
#[path = "src/backtrace.rs"]
pub mod backtrace;
#[path = "src/kernel.rs"]
pub mod kernel;
#[path = "src/etc.rs"]
pub mod etc;
#[path = "src/version.rs"]
pub mod version;
#[path = "src/proc.rs"]
pub mod proc;
#[path = "src/hash.rs"]
pub mod hash;
#[path = "src/range.rs"]
pub mod range;
#[path = "src/gc.rs"]
pub mod gc;
#[path = "src/variable.rs"]
pub mod variable;
#[path = "src/print.rs"]
pub mod print;
#[path = "src/pool.rs"]
pub mod pool;

